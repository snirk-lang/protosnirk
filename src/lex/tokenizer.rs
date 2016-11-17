//! Token table: make tokenizing easier and expandable

//! We need a dumb tokenizer in case users want to register operators.

use std::borrow::Cow;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::iter::Peekable;
use std::str::Chars;

use unicode_categories::UnicodeCategories;

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

pub fn get_default_symbols() -> HashMap<char, Vec<(Cow<'static, str>, TokenData)>> {
    hashmap! {
        '=' => vec![ (Cow::Borrowed("="), Assign) ],

        '+' => vec![ (Cow::Borrowed("+"), Plus), (Cow::Borrowed("+="), PlusAssign) ],
        '-' => vec![ (Cow::Borrowed("-"), Minus), (Cow::Borrowed("-="), MinusAssign) ],
        '*' => vec![ (Cow::Borrowed("*"), Star), (Cow::Borrowed("*="), StarAssign) ],
        '%' => vec![ (Cow::Borrowed("%"), Percent), (Cow::Borrowed("%="), PercentAssign) ],

        '(' => vec![ (Cow::Borrowed("("), LeftParen) ],
        ')' => vec![ (Cow::Borrowed(")"), RightParen) ],
        '[' => vec![ (Cow::Borrowed("["), LeftBrace) ],
        ']' => vec![ (Cow::Borrowed("]"), RightBrace) ],
        '{' => vec![ (Cow::Borrowed("{"), LeftSquiggle) ],
        '}' => vec![ (Cow::Borrowed("}"), RightSquiggle) ],
        '<' => vec![ (Cow::Borrowed("<"), LeftAngle) ],
        '>' => vec![ (Cow::Borrowed(">"), RightAngle) ]
    }
}

/// Token enum - tokens are pretty simple, mostly dependent on string matching.
#[derive(Debug, PartialEq, Clone)]
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

    /// Token indicates an increase in newline
    IncreaseIndent,
    /// Token indicates an end of block
    EndBlock,
    /// Token is known to be an EOF
    EOF
}

/// Starting location of a token or expression.
///
/// Contains information to
#[derive(Debug, PartialEq, Eq, Clone, Hash, Default)]
pub struct TextLocation {
    /// Which byte of the initial string the token starts on
    pub start_byte: usize,
    /// Size in bytes of this TextLocation
    pub span_bytes: usize,
    /// Which line of the initial string the token starts on
    pub line: usize,
    /// Which column of the initial string the token starts on
    pub column: usize,
    /// Name of the file the token appears in
    pub file_name: String
}
impl TextLocation {
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
pub struct StaticStrTokenizer {
    /// Keywords registered with the tokenizer
    keywords: HashMap<Cow<'static, str>, TokenData>,
    chars: Peekable<Chars<'static>>
}

impl StaticStrTokenizer {
    /// Creates a new StaticStrTokenizer from the given string
    pub fn new(input: &'static str) -> StaticStrTokenizer {
        StaticStrTokenizer {
            keywords: get_default_tokens(),
            chars: input.chars().peekable()
        }
    }

    /// Gets the next token from the tokenizer
    pub fn next(&mut self) -> Token {
        let peek_attempt = self.chars.peek().cloned();
        if !peek_attempt.is_some() {
            return Token {
                location: TextLocation::default(),
                data: TokenData::EOF
            }
        }
        let mut peek = peek_attempt.expect("Checked expect");
        while peek.is_whitespace() {
            self.chars.next();
            let next = self.chars.peek().cloned();
            if next.is_none() {
                return Token {
                    location: TextLocation::default(),
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
                    location: TextLocation::default(),
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
        self.take_while(|ch|
            ch.is_letter() || ch.is_number() || ch == '_',
            &mut token_string);
        if self.keywords.get(&Cow::Borrowed(&*token_string)).is_some() {
            Token {
                location: TextLocation::default(),
                data: TokenData::Keyword(Cow::Owned(token_string))
            }
        } else {
            Token {
                location: TextLocation::default(),
                data: TokenData::Ident(Cow::Owned(token_string))
            }
        }
    }

    /// Parse a floating point literal
    fn parse_float_literal(&mut self) -> Token {
        let mut token_string = String::new();
        self.take_while(char::is_number, &mut token_string);
        // First part of number done. Is it a decimal?
        if *self.chars.peek().unwrap_or(&' ') == '.' {
            // Push the decmial point
            token_string.push(self.chars.next().expect("Checked expect"));
            if !self.chars.peek().unwrap_or(&' ').is_number() {
                // Actually, let's not
                token_string.pop();
                let parsed: f64 = token_string.parse()
                    .expect("Couldn't parse float");
                return Token {
                    location: TextLocation::default(),
                    data: TokenData::NumberLiteral(parsed)
                }
            }
            self.take_while(char::is_number, &mut token_string);
        }
        if self.chars.peek().unwrap_or(&' ').to_lowercase().collect::<String>() != "e" {
            let parsed: f64 = token_string.parse()
                .expect("Couldn't parse float");
            return Token {
                location: TextLocation::default(),
                data: TokenData::NumberLiteral(parsed)
            }
        }
        token_string.push(self.chars.next().expect("Checked expect"));
        // Need numbers after the E
        if !self.chars.peek().unwrap_or(&' ').is_number() {
            let parsed: f64 = token_string.parse()
                .expect("Couldn't parse float");
            return Token {
                location: TextLocation::default(),
                data: TokenData::NumberLiteral(parsed)
            }
        }
        self.take_while(char::is_number, &mut token_string);
        let parsed: f64 = token_string.parse()
            .expect("Couldn't parse float");
        return Token {
            location: TextLocation::default(),
            data: TokenData::NumberLiteral(parsed)
        }
    }

    /// Continue taking characters while a condition is met
    #[inline]
    fn take_while<F: Fn(char) -> bool>(&mut self, func: F, acc: &mut String) {
        loop {
            if let Some(peeked) = self.chars.peek() {
                if !func(*peeked) {
                    return
                } else {
                    acc.push(*peeked);
                }
            } else {
                return
            }
            self.chars.next();
        }
    }

    /// Skip characters while a condition is met
    #[inline]
    fn skip_while<F: Fn(char) -> bool>(&mut self, func: F) {
        loop {
            if let Some(peeked) = self.chars.peek() {
                if !func(*peeked) {
                    return
                }
            } else {
                return
            }
            self.chars.next();
        }
    }

    /// Grab the next charcter
    fn next_char(&mut self) -> Option<char> {
        self.chars.next()
    }
}
