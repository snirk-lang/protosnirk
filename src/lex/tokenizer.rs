//! Token table: make tokenizing easier and expandable

//! We need a dumb tokenizer in case users want to register operators.

use std::borrow::Cow;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::iter::Peekable;
use std::str::Chars;

use unicode_categories::UnicodeCategories;

use lex::{tokens, Token, TokenData, TokenType, TextIter, PeekTextIter};

/// Trait for a tokenizer which can iterate over tokens.
pub trait Tokenizer {
    fn next(&mut self) -> Token;
}

/// Hacky implementation of a tokenizer.
pub struct IterTokenizer<I> where I: Iterator<Item=char> {
    /// Keywords registered with the tokenizer
    keywords: HashSet<Cow<'static, str>>,
    /// Symbols registered with the tokenizer
    symbols: HashSet<Cow<'static, str>>,
    iter: PeekTextIter<I>
}

impl<I: Iterator<Item=char>> IterTokenizer<I> {
    /// Creates a new StaticStrTokenizer from the given string
    pub fn new(input: I) -> IterTokenizer<I> {
        IterTokenizer {
            keywords: tokens::default_keywords(),
            symbols: tokens::default_symbols(),
            iter: PeekTextIter::new(input.peekable())
        }
    }

    /// Gets the next token from the tokenizer
    pub fn next(&mut self) -> Token {
        let peek_attempt = self.iter.peek();
        if !peek_attempt.is_some() {
            return Token {
                location: self.iter.get_location(),
                text: Cow::Borrowed(""),
                data: TokenData::EOF
            }
        }
        let mut peek = peek_attempt.expect("Checked expect");
        while peek.is_whitespace() {
            self.iter.next();
            let next = self.iter.peek();
            if next.is_none() {
                return Token {
                    location: self.iter.get_location(),
                    text: Cow::Borrowed(""),
                    data: TokenData::EOF
                }
            } else {
                peek = next.expect("Checked expect");
            }
        }
        if peek.is_number() {
            self.parse_float_literal()
        } else if peek.is_letter() {
            self.parse_keyword_or_ident()
        } else if peek.is_symbol() || peek == '%' || peek == '/' {
            self.parse_symbol()
        } else {
            panic!("Got weird character {:?}", peek);
        }
    }

    /// Parse a symbol
    ///
    /// This logic differs from that of keyword parsing in that
    /// it attempts to match bigger symbols
    fn parse_symbol(&mut self) -> Token {
        let mut sym = String::new();
        let location = self.iter.get_location();
        self.take_while(|ch| ch == '%' || ch == '/' || ch.is_symbol(), &mut sym);
        if sym.starts_with("///") {
            // doc comment - will be implemented later on
            self.take_while(|ch| ch != '\n', &mut sym);
            return self.next()
        } else if sym.starts_with("//") {
            self.skip_while(|ch| ch != '\n');
            return self.next()
        }
        loop {
            println!("Looking for symbols with {:?}", sym);
            if self.symbols.contains(&Cow::Borrowed(&*sym)) {
                return Token {
                    location: location,
                    text: Cow::Owned(sym),
                    data: TokenData::Symbol
                }
            } else {
                if sym.is_empty() {
                    panic!("Couldn't find symbol for {:?}", sym);
                } else {
                    sym.pop();
                }
            }
        }
    }

    fn parse_keyword_or_ident(&mut self) -> Token {
        let mut token_string = String::new();
        let location = self.iter.get_location();
        let is_kw = self.take_while_ident(&mut token_string);
        if is_kw && self.keywords.get(&Cow::Borrowed(&*token_string)).is_some() {
            Token {
                location: location,
                text: Cow::Owned(token_string),
                data: TokenData::Keyword
            }
        } else {
            Token {
                location: location,
                text: Cow::Owned(token_string),
                data: TokenData::Ident
            }
        }
    }

    /// Parse a floating point literal
    fn parse_float_literal(&mut self) -> Token {
        let mut token_string = String::new();
        let location = self.iter.get_location();
        self.take_while(char::is_number, &mut token_string);
        // First part of number done. Is it a decimal?
        if self.iter.peek().unwrap_or(' ') == '.' {
            // Push the decmial point
            token_string.push(self.iter.next().expect("Checked expect"));
            if !self.iter.peek().unwrap_or(' ').is_number() {
                // Actually, let's not
                token_string.pop();
                let parsed: f64 = token_string.parse()
                    .expect("Couldn't parse float");
                return Token {
                    location: location,
                    text: Cow::Owned(token_string),
                    data: TokenData::NumberLiteral(parsed)
                }
            }
            self.take_while(char::is_number, &mut token_string);
        }
        if self.iter.peek().unwrap_or(' ').to_lowercase().collect::<String>() != "e" {
            let parsed: f64 = token_string.parse()
                .expect("Couldn't parse float");
            return Token {
                location: location,
                text: Cow::Owned(token_string),
                data: TokenData::NumberLiteral(parsed)
            }
        }
        token_string.push(self.iter.next().expect("Checked expect"));
        // Need numbers after the E
        if !self.iter.peek().unwrap_or(' ').is_number() {
            let parsed: f64 = token_string.parse()
                .expect("Couldn't parse float");
            return Token {
                location: location,
                text: Cow::Owned(token_string),
                data: TokenData::NumberLiteral(parsed)
            }
        }
        self.take_while(char::is_number, &mut token_string);
        let parsed: f64 = token_string.parse()
            .expect("Couldn't parse float");
        return Token {
            location: location,
            text: Cow::Owned(token_string),
            data: TokenData::NumberLiteral(parsed)
        }
    }

    /// Continue taking characters while a condition is met
    #[inline]
    fn take_while<F: Fn(char) -> bool>(&mut self, func: F, acc: &mut String) {
        loop {
            if let Some(peeked) = self.iter.peek() {
                if !func(peeked) {
                    return
                } else {
                    acc.push(peeked);
                }
            } else {
                return
            }
            self.iter.next();
        }
    }

    fn take_while_ident(&mut self, acc: &mut String) -> bool {
        let mut parsing_kw = true;
        loop {
            if let Some(peeked) = self.iter.peek() {
                if peeked.is_number() || peeked == '_' {
                    parsing_kw = false;
                    acc.push(peeked);
                } else if peeked.is_letter() {
                    acc.push(peeked);
                } else {
                    return parsing_kw
                }
            } else {
                return parsing_kw
            }
            self.iter.next();
        }
    }

    /// Skip characters while a condition is met
    #[inline]
    fn skip_while<F: Fn(char) -> bool>(&mut self, func: F) {
        loop {
            if let Some(peeked) = self.iter.peek() {
                if !func(peeked) {
                    return
                }
            } else {
                return
            }
            self.iter.next();
        }
    }

    /// Grab the next charcter
    fn next_char(&mut self) -> Option<char> {
        self.iter.next()
    }
}
