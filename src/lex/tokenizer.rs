//! Token table: make tokenizing easier and expandable

//! We need a dumb tokenizer in case users want to register operators.

use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::iter::Peekable;
use std::str::Chars;

use unicode_categories::UnicodeCategories;

use lex::{tokens, TextLocation,
          TokenizerSymbolRule, CowStr,
          Token, TokenData, TokenType,
          TextIter, PeekTextIter};

/// Trait for a tokenizer which can iterate over tokens.
pub trait Tokenizer {
    fn next(&mut self) -> Token;
}

/// If the given char is a symbol.
pub fn char_is_symbol(ch: char) -> bool {
    ch == '%' || ch == '/' ||
    ch == '(' || ch == ')' ||
    ch == '-' || ch == '*' ||
    ch == ',' || ch == ':' ||
    ch == '!' ||
    ch.is_symbol()
}

/// If the character is whitespace, but not newlines.
pub fn char_is_spacing(ch: char) -> bool {
    ch != '\r' && ch != '\n' && ch.is_whitespace()
}

/// Simple state for parser to be in
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TokenizerState {
    /// Tokenizing at beginning of line, spacing is `BlockBegin`
    LookingForIndent,
    /// Tokenized all indents, getting keywords, symbols, and idents until
    LookingForNewline,
    /// Tokenizer has been tabbed in, may need to emit 1 or more outdents
    EmittingOutdents,
    /// Tokenizer reached EOF, only EOF tokens from here
    ReachedEOF
}

/// Hacky implementation of a tokenizer.
pub struct IterTokenizer<I> where I: Iterator<Item=char> {
    /// Keywords registered with the tokenizer
    keywords: HashSet<CowStr>,
    /// Symbols registered with the tokenizer
    symbols: HashMap<CowStr, TokenizerSymbolRule>,
    /// Ezpexted number of spaces to indent per indentation level.
    ///
    /// Derived from first indent used, can emit warnings for
    /// inconsistent indenting if spaces do not match.
    expected_indent_length: usize,
    /// Whether the file is being indented with spaces or tabs.
    ///
    /// An error is emitted if tab and space indenting is mixed.
    expected_indent_spaces: bool,
    /// Whether whitespace is tokenized as indentation
    tokenizer_state: TokenizerState,
    /// Stack of indents being made.
    indent_size_stack: Vec<usize>,
    /// Peekable iterator over the characters
    iter: PeekTextIter<I>
}

impl<I: Iterator<Item=char>> Tokenizer for IterTokenizer<I> {
    #[inline]
    fn next(&mut self) -> Token {
        let next = self.next();
        trace!("> Next token {:?}", next);
        next
    }
}

impl<I: Iterator<Item=char>> IterTokenizer<I> {
    /// Creates a new StaticStrTokenizer from the given string
    pub fn new(input: I) -> IterTokenizer<I> {
        IterTokenizer {
            keywords: tokens::default_keywords(),
            symbols: tokens::default_symbols(),

            // Will be overridden later when reading file
            // If first line beins with a space, error
            expected_indent_length: 0usize,
            expected_indent_spaces: true,
            // This will discard spacing at the beginning of a file
            tokenizer_state: TokenizerState::LookingForNewline,
            indent_size_stack: vec![0usize],

            iter: PeekTextIter::new(input.peekable())
        }
    }

    /// Gets the next token from the tokenizer
    pub fn next(&mut self) -> Token {
        trace!(">Calling next on {:?}, peeked {:?}",
            self.tokenizer_state, self.iter.peek());
        match self.tokenizer_state {
            TokenizerState::LookingForIndent =>
                self.next_indent(),
            TokenizerState::LookingForNewline =>
                self.next_line(),
            TokenizerState::ReachedEOF =>
                self.next_eof(),
            TokenizerState::EmittingOutdents =>
                self.next_outdent()
        }
    }

    /// Emit remaining `BlockEnd` and `EOF` tokens
    fn next_eof(&mut self) -> Token {
        trace!("Calling next_eof, have {} indents left", self.indent_size_stack.len());
        if self.indent_size_stack.len() > 1 {
            self.indent_size_stack.pop();
            trace!("Returning an outdent");
            return Token::new_outdent(self.iter.get_location())
        }
        return Token::new_eof(self.iter.get_location())
    }

    /// Get the next `BlockBegin` token(s)
    fn next_indent(&mut self) -> Token {
        let peek_attempt = self.iter.peek();
        if peek_attempt.is_none() {
            self.tokenizer_state = TokenizerState::ReachedEOF;
            return self.next_eof()
        }
        let mut space_count = 0usize;
        let mut peeked = peek_attempt.expect("Checked expect");
        // Take all consecutive spaces
        trace!("Taking consecutive spaces starting with {:?}", peeked);
        while char_is_spacing(peeked) {
            self.iter.next();
            space_count += 1;
            let next_peek = self.iter.peek();
            if next_peek.is_none() {
                break
            }
            peeked = next_peek.expect("checked expect");
            // TODO error on mixed tabs/spaces
        }
        trace!("Peeked to {}, with {} spaces", peeked, space_count);
        // Now that indents are found, go back to regular tokens.
        self.tokenizer_state = TokenizerState::LookingForNewline;

        // We've itered over some number of spaces until a non-space.
        let current_indent = *self.indent_size_stack.last()
            .expect("Indent stack was missing leading 0");

        trace!("Indent stack = {:?} (current {})",
            self.indent_size_stack, current_indent);

        // Equal indentation: no starting block, go directly to parsing line
        if space_count == current_indent {
            trace!("Indentation is the same, calling next_line");
            self.next_line() // Mutually recursive for empty lines
        }
        // Greater Indendation: new block
        else if space_count > current_indent {
            trace!("Indentation greater, pushing {} and returning a BeginBlock", space_count);
            self.indent_size_stack.push(space_count);
            Token::new_indent(self.iter.get_location())
        }
        else { // space_count < current_indent
            trace!("Indentation is less, going to emit outdents");
            self.tokenizer_state = TokenizerState::EmittingOutdents;
            self.next_outdent()
        }
    }

    /// Emit all needed outdents until tabbing lines up.
    fn next_outdent(&mut self) -> Token {
        trace!("Calling next_outdent");
        let location = self.iter.get_location();
        trace!("Current pos: {:?}", location);
        trace!("Indent stack: {:?}", self.indent_size_stack);
        if self.indent_size_stack.len() > 1usize {
            let last_indent = *self.indent_size_stack.last()
                .expect("Checked expect");
            trace!("Checking indent {} vs {}", last_indent, location.column);
            if last_indent > location.column {
                trace!("Popping the indent");
                self.indent_size_stack.pop();
                let position = TextLocation {
                    column: last_indent,
                    index: location.index + (last_indent - location.column),
                    line: location.line
                };
                return Token::new_outdent(position)
            }
            else if last_indent < location.column {
                trace!("last_indent < location.column");
            }
        }
        // Edge case: there shouldn't be any indentation but we are indented
        else if location.column > 0 {
            // not getting hit usually
            trace!("There shouldn't be any indentation but we are indented");
            self.indent_size_stack.push(location.column);
            self.tokenizer_state = TokenizerState::LookingForNewline;
            return Token::new_indent(self.iter.get_location())
        }
        trace!("next_outdent done with outdents, calling next_line");
        self.tokenizer_state = TokenizerState::LookingForNewline;
        return self.next_line()
    }

    /// We've parsed all the indentation, so parse tokens until newline,
    /// then prepare to parse indentation again.
    fn next_line(&mut self) -> Token {
        trace!("Looking at next_line");
        let maybe_peek = self.iter.peek();
        if maybe_peek.is_none() {
            self.tokenizer_state = TokenizerState::ReachedEOF;
            return self.next_eof()
        }
        let mut peek = maybe_peek.expect("checked expect");

        // Skip spacing if between tokens.
        // TODO for linting purposes, keep track of spaces used.
        // Midline tabs are not appreciated, nor are spaces missing
        // between symbols, in some contexts.
        trace!("Looping through all chars that are spaces");
        while char_is_spacing(peek) {
            trace!("Consuming space {:?}", peek);
            self.iter.next();
            let next_peek = self.iter.peek();
            if next_peek.is_none() {
                trace!("next_line -> eof");
                self.tokenizer_state = TokenizerState::ReachedEOF;
                return self.next_eof()
            }
            else {
                peek = next_peek.expect("checked expect")
            }
        }

        // We've eliminated spaces after the last token.
        // We have the peeked char for the different token parsers to look at.

        // If we have a newline, we need to emit a token for it and go
        // back to checking idents. This means that empty whitespace at the
        // end of lines that have text is okay.

        trace!("Consumed all the spacing chars");

        // We handle \r first, then look at the following \n.
        // TODO warn on mixed \r\n and \n
        if peek == '\r' {
            self.iter.next(); // comsume \r
            // Give an error for \r at EOF
            if self.iter.peek().is_none() {
                // TODO error here
                panic!("Hanging `\\r` at EOF, {:?}", self.iter.get_location());
            }
            // Peek for the \n
            let expected_newline = self.iter.peek().expect("Already peeked");
            if expected_newline != '\n' {
                // TODO need to format i.e. `\t` -> `\\t` here...
                panic!("Invalid control sequence `\\r{}`", expected_newline);
            }
            peek = expected_newline; // peeked \n here
        }

        // We either ran into it after some amount of whitespace, or found it
        // after `\r`. Line is done, parse the indents on the next one.
        if peek == '\n' {
            self.iter.next(); // Original `peek` OR `peek` from the if above
            self.tokenizer_state = TokenizerState::LookingForIndent;
            self.next_indent() // Mutually recursive for emtpy lines
        }
        else if peek.is_number() {
            self.parse_float_literal()
        } else if peek == '_' || peek.is_letter() {
            self.parse_keyword_or_ident()
        } else if char_is_symbol(peek) {
            self.parse_symbol()
        } else {
            panic!("Unknown character `{:?}` in next_line", peek);
        }
    }

    /// Parse a symbol
    ///
    /// This logic differs from that of keyword parsing in that
    /// it attempts to match bigger symbols
    fn parse_symbol(&mut self) -> Token {
        use lex::TokenizerSymbolRule::*;
        let location = self.iter.get_location();
        let mut sym = String::new();

        loop {
            let more: bool;
            if let Some(peeked) = self.iter.peek() {
                more = true;
                sym.push(peeked);
            } else {
                more = false;
            }// Infinite loop??
            // We can take newlines off of comments in symbol parsing.
            // The newlines at the end of comments shouldn't show up
            // as tokens anyway.
            if sym.starts_with("///") {
                // doc comment - will be implemented later on
                self.take_while(|ch| ch != '\n', &mut sym);
                return self.next()
            } else if sym.starts_with("//") {
                self.skip_while(|ch| ch != '\n');
                return self.next()
            }

            let symbol_type = self.symbols.get(&Cow::Borrowed(&*sym)).cloned();
            match symbol_type {
                // No symbol matched - we started out bad or peeked too far
                None => {
                    if sym.len() == 1 {
                        panic!("Couldn't find symbol {:?}", sym);
                    } else {
                        sym.pop();
                        match self.symbols.get(&Cow::Borrowed(&*sym)).cloned() {
                            // We can't have stepped past these
                            None | Some(Complete) => unreachable!(),
                            // We stepped past a CompletePrefix token
                            Some(CompletePrefix) => {
                                return Token::new(sym, location, TokenData::Symbol)
                            },
                            // We stepped past a partial token but did not complete it
                            Some(Partial) => {
                                panic!("Could not complete partial token {:?}", sym);
                            }
                        }
                    }
                }
                // We found a complete symbol - consume what we peeked and return it.
                Some(Complete) => {
                    self.iter.next();
                    return Token::new(sym, location, TokenData::Symbol);
                },
                // We have more to go, consume what we peeked and continue the loop
                Some(CompletePrefix) | Some(Partial) => {
                    if !more {
                        return Token::new(sym, location, TokenData::Symbol)
                    }
                    self.iter.next();
                }
            }
        }
    }

    /// Parse keyword or identifier
    fn parse_keyword_or_ident(&mut self) -> Token {
        let mut token_string = String::new();
        let location = self.iter.get_location();
        let is_kw = self.take_while_ident(&mut token_string);
        if is_kw && self.keywords.get(&Cow::Borrowed(&*token_string)).is_some() {
            Token::new(token_string, location, TokenData::Keyword)
        } else {
            Token::new_ident(token_string, location)
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
