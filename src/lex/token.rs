/// The token is a slice into the string that the tokenizer is parsing.
/// Each token has a `TokenType` indicating what it is and its data.

use std::borrow::Cow;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::fmt::Result as FmtResult;
use std::ops::Range;

use lex::{TextLocation, CowStr};

/// A token returned by the tokenizer.
///
/// Each token has a definite
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    /// Location of the token in a file
    pub location: TextLocation,
    /// Text of the token at that location
    pub text: CowStr,
    /// Additional data (type/literal) provided by the lexer
    pub data: TokenData
}

impl Token {
    #[inline]
    pub fn new_symbol<T: Into<CowStr>>(text: T, location: TextLocation) -> Token {
        Token {
            text: text.into(),
            data: TokenData::Symbol,
            location: location
        }
    }
    #[inline]
    pub fn new_keyword<T: Into<CowStr>>(text: T, location: TextLocation) -> Token {
        Token {
            text: text.into(),
            data: TokenData::Keyword,
            location: location
        }
    }
    #[inline]
    pub fn new_ident<T: Into<CowStr>>(text: T, location: TextLocation) -> Token {
        Token {
            text: text.into(),
            data: TokenData::Ident,
            location: location
        }
    }

    #[inline]
    pub fn new_indent(location: TextLocation) -> Token {
        Token {
            text: Cow::Borrowed(""),
            data: TokenData::BeginBock,
            location: location
        }
    }

    #[inline]
    pub fn new_outdent(location: TextLocation) -> Token {
        Token {
            text: Cow::Borrowed(""),
            data: TokenData::EndBlock,
            location: location
        }
    }

    #[inline]
    pub fn new_eof(location: TextLocation) -> Token {
        Token {
            text: Cow::Borrowed(""),
            data: TokenData::EOF,
            location: location
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "({:?}, {:?})", self.data.get_type(), self.text)
    }
}

impl Default for Token {
    fn default() -> Token {
        Token {
            data: TokenData::EOF,
            .. Default::default()
        }
    }
}

/// Token enum - tokens are pretty simple, mostly dependent on string matching.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenData {
    /// Token is a numeric literal
    NumberLiteral(f64),

    /// Token is some name
    Ident,
    /// Token is a keyword
    Keyword,
    /// Token is some symbol
    Symbol,
    /// Indendation of block
    BeginBock,
    /// Outdendation of block
    EndBlock,
    /// Token is an EOF
    EOF
}
impl TokenData {
    /// If this token is an identifier
    #[inline]
    pub fn get_type(&self) -> TokenType {
        use self::TokenData::*;
        match *self {
            NumberLiteral(_) => TokenType::Literal,
            Ident => TokenType::Ident,
            Keyword => TokenType::Keyword,
            Symbol => TokenType::Symbol,
            BeginBock => TokenType::BeginBlock,
            EndBlock => TokenType::EndBlock,
            EOF => TokenType::EOF
        }
    }
}

/// Which type of token this is.
///
/// Can be used by the parser for defaulting to Ident parsing,
/// or individual parsers for error handling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    /// Token is a name
    Ident,
    /// Token is a literal
    Literal,
    /// Token is a registered keyword
    Keyword,
    /// Token is a registered symbol
    Symbol,
    /// Token is a begin/end block
    BeginBlock,
    EndBlock,
    /// Token is an EOF
    EOF
}
