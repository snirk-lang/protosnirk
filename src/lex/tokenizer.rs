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
    /// Symbols known by the tokenizer
    //symbols: HashMap<Cow<'static, str>, TokenData>,
    symbols: HashMap<char, Vec<(Cow<'static, str>, TokenData)>>,
    chars: Peekable<Chars<'static>>
}

impl StaticStrTokenizer {
    pub fn new(input: &'static str) -> StaticStrTokenizer {
        StaticStrTokenizer {
            keywords: get_default_tokens(),
            symbols: get_default_symbols(),
            chars: input.chars().peekable()
        }
    }

    pub fn next(&mut self) -> Token {
        let next_char: char;
        {
            let maybe_next = self.chars.next();
            if maybe_next.is_none() {
                return Token {
                    location: TextLocation::default(),
                    data: TokenData::EOF
                }
            }
            next_char = maybe_next.expect("Checked expect");
        }
        let mut token_string = next_char.to_string();
        // If we start with a letter, keep grabbing letters/numbers/_ for ident/kw
        if next_char.is_letter() {
            let mut could_be_kw = true;
            while let Some(peek) = self.next_char() {
                // If next ones are all letters, keep grabbing
                if peek.is_letter() {
                    token_string.push(peek);
                // If we get a number or _ we can't have a keyword anymore
                } else if peek.is_number() || peek == '_' {
                    could_be_kw = false;
                    token_string.push(peek);
                // Anything else means we're done with this; see if its a ketword, else return an ident
                } else {
                    if could_be_kw {
                        if let Some(_matched_data) = self.keywords.get(&*token_string) {
                            return Token {
                                location: TextLocation::default(),
                                data: TokenData::Keyword(Cow::Owned(token_string))
                            }
                        // No token matched (also no letters/numbers)
                        } else {
                            return Token {
                                location: TextLocation::default(),
                                data: TokenData::Ident(Cow::Owned(token_string))
                            }
                        }
                    } else {
                        return Token {
                            location: TextLocation::default(),
                            data: TokenData::Ident(Cow::Owned(token_string))
                        }
                    }
                }
            }
            // We're all out of chars!
            if let Some(_matched_data) = self.keywords.get(&*token_string) {
                return Token {
                        location: TextLocation::default(),
                        data: TokenData::Keyword(Cow::Owned(token_string))
                }
            } else {
                return Token {
                    location: TextLocation::default(),
                    data: TokenData::Ident(Cow::Owned(token_string))
                }
            }
        // Start matching a number
        } else if next_char.is_number() {
            self.take_while(char::is_number, &mut token_string);
            // First part of number done. Is it a decimal?
            if *self.chars.peek().unwrap_or(&' ') == '.' {
                if !self.chars.peek().unwrap_or(&' ').is_number() {
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
            let _e = self.chars.next();
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
        // Get them comments!
        // Line comments only
        } else if next_char == '/' {
             if *self.chars.peek().unwrap_or(&' ') == '/' {
                 self.skip_while(|ch| ch != '\n');
                 return self.next(); // Recursive :(
             }
        }
        if next_char.is_symbol() {
            if let Some(possible_symbols) = self.symbols.get(&next_char) {
                let mut matches = possible_symbols.clone();
                loop {
                    if matches.is_empty() {
                        panic!("No symbol matched :/");
                    } else if matches.len() == 1 {
                        let (_match, token) = matches.remove(0);
                        debug_assert_eq!(_match, token_string);
                        return Token {
                            location: TextLocation::default(),
                            data: token
                        }
                    } else {
                        for index in 0..matches.len() - 1 {
                            let drop;
                            {
                                let (ref name, ref _data) = matches[index];
                                drop = !name.starts_with(&*token_string);
                            } if drop {
                                matches.remove(index);
                            }
                        }
                        token_string.push(self.chars.next().unwrap_or(' '));
                    }
                }
            } else {
                panic!("Unknown symbol {}", next_char);
                // Skip over bad tokens.
            }
        } else if next_char.is_whitespace() {
            self.skip_while(char::is_whitespace);
            return self.next() // recursive :(
        } else {
            panic!("Unable to match {:?}", next_char);
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
                    acc.push(*peeked)
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

    fn next_char(&mut self) -> Option<char> {
        self.chars.next()
    }
}
