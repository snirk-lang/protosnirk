//! Token table: make tokenizing easier and expandable

//! We need a dumb tokenizer in case users want to register operators.

use std::borrow::Cow;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::iter::Peekable;
use std::str::Chars;

use unicode_categories::UnicodeCategories;

use lex::{TextIter, PeekTextIter};

/// Trait for a tokenizer which can iterate over tokens.
pub trait Tokenizer {
    fn next(&mut self) -> Token;
}

macro_rules! declare_tokens {
    ($( $(#[$attr:meta])* pub const $name:ident : $typ:tt = $value:expr;)*) => {
        $(
            $(#[$attr])*
            #[allow(dead_code, non_upper_case_globals)]
            pub const $name : TokenData = TokenData::$typ(Cow::Borrowed($value));
        )*
        pub fn get_default_tokens() -> HashMap<Cow<'static, str>, TokenData> {
            // Yo dawg, I heard you like macros
            hashmap! [
                $(Cow::Borrowed($value) => $name),*
            ]
        }
    }
}

declare_tokens! {
    pub const Plus: Symbol = "+";
    pub const Minus: Symbol = "-";
    pub const Star: Symbol = "*";
    pub const Slash: Symbol = "/";
    pub const Assign: Symbol = "=";
    pub const Percent: Symbol = "%";

    pub const PlusAssign: Symbol = "+=";
    pub const MinusAssign: Symbol = "-=";
    pub const StarAssign: Symbol = "*=";
    pub const PercentAssign: Symbol = "%=";

    pub const Let: Keyword = "let";
    pub const Mut: Keyword = "mut";
    pub const Return: Keyword = "return";

    pub const LeftParen: Symbol = "(";
    pub const RightParen: Symbol = ")";
    pub const LeftBrace: Symbol = "[";
    pub const RightBrace: Symbol = "]";
    pub const LeftSquiggle: Symbol = "{";
    pub const RightSquiggle: Symbol = "}";
    pub const LeftAngle: Symbol = "<";
    pub const RightAngle: Symbol = ">";

    pub const GitDiffSymbol: Symbol = "<<<<<<<";
}

/// Token enum - tokens are pretty simple, mostly dependent on string matching.
#[derive(Debug, Clone, Hash)]
pub enum TokenData {
    /// Token is a numeric literal
    NumberLiteral(f64),
    // ... other literals

    /// Token is some name
    Ident(Cow<'static, str>),
    /// Token is a keyword
    Keyword(Cow<'static, str>),
    /// Token is some symbol
    Symbol(Cow<'static, str>),

    /// Token is known to be an EOF
    EOF
}
impl TokenData {
    /// If this token is an identifier
    #[inline]
    pub fn get_type(&self) -> TokenType {
        use self::TokenData::*;
        match *self {
            NumberLiteral(_) => TokenType::Literal,
            Ident(_) => TokenType::Ident,
            Keyword(_) => TokenType::Keyword,
            Symbol(_) => TokenType::Symbol,
            EOF => TokenType::EOF
        }
    }
}

/// Which type of token this is.
///
/// Can be used by the parser for defaulting to Ident parsing,
/// or individual parsers for error handling
pub enum TokenType {
    /// Token is a name
    Ident(Cow<'static, str),
    /// Token is a literal
    Literal,
    /// Token is a registered keyword
    Keyword,
    /// Token is a registered symbol
    Symbol,
    /// Token is an EOF
    EOF
}

/// Starting location of a token or expression.
///
/// Contains information to
#[derive(Debug, PartialEq, Eq, Clone, Hash, Default)]
pub struct TextLocation {
    /// Which char position of the initial string the token starts on
    ///
    /// Should respect Unicode boundaries, etc.
    pub start_char: usize,
    /// Which line of the initial string the token starts on
    pub start_line: usize,
    /// Which column of the initial string the token starts on
    pub start_column: usize,
    // /// Name of the file the token appears in
    // pub file_name: String
}

/// A token returned by the tokenizer.
///
/// Each token has a definite
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub location: TextLocation,
    pub data: TokenData
}

/// Hacky implementation of a tokenizer.
pub struct IterTokenizer<I> where I: Iterator<Item=char> {
    /// Keywords registered with the tokenizer
    keywords: HashMap<Cow<'static, str>, TokenData>,
    iter: PeekTextIter<I>
}

impl<I: Iterator<Item=char>> IterTokenizer<I> {
    /// Creates a new StaticStrTokenizer from the given string
    pub fn new(input: I) -> IterTokenizer<I> {
        IterTokenizer {
            keywords: get_default_tokens(),
            iter: PeekTextIter::new(input.peekable())
        }
    }

    /// Gets the next token from the tokenizer
    pub fn next(&mut self) -> Token {
        let peek_attempt = self.iter.peek();
        if !peek_attempt.is_some() {
            return Token {
                location: self.iter.get_location(),
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
            if self.keywords.get(&Cow::Borrowed(&*sym)).is_some() {
                return Token {
                    location: self.iter.get_location(),
                    data: TokenData::Symbol(Cow::Owned(sym))
                }
            } else {
                if sym.is_empty() {
                    panic!("Couldn't find a symbol");
                } else {
                    sym.pop();
                }
            }
        }
    }

    fn parse_keyword_or_ident(&mut self) -> Token {
        let mut token_string = String::new();
        let is_kw = self.take_while_ident(&mut token_string);
        if is_kw && self.keywords.get(&Cow::Borrowed(&*token_string)).is_some() {
            Token {
                location: self.iter.get_location(),
                data: TokenData::Keyword(Cow::Owned(token_string))
            }
        } else {
            Token {
                location: self.iter.get_location(),
                data: TokenData::Ident(Cow::Owned(token_string))
            }
        }
    }

    /// Parse a floating point literal
    fn parse_float_literal(&mut self) -> Token {
        let mut token_string = String::new();
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
                    location: self.iter.get_location(),
                    data: TokenData::NumberLiteral(parsed)
                }
            }
            self.take_while(char::is_number, &mut token_string);
        }
        if self.iter.peek().unwrap_or(' ').to_lowercase().collect::<String>() != "e" {
            let parsed: f64 = token_string.parse()
                .expect("Couldn't parse float");
            return Token {
                location: self.iter.get_location(),
                data: TokenData::NumberLiteral(parsed)
            }
        }
        token_string.push(self.iter.next().expect("Checked expect"));
        // Need numbers after the E
        if !self.iter.peek().unwrap_or(' ').is_number() {
            let parsed: f64 = token_string.parse()
                .expect("Couldn't parse float");
            return Token {
                location: self.iter.get_location(),
                data: TokenData::NumberLiteral(parsed)
            }
        }
        self.take_while(char::is_number, &mut token_string);
        let parsed: f64 = token_string.parse()
            .expect("Couldn't parse float");
        return Token {
            location: self.iter.get_location(),
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
