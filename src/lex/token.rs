/// The token is a slice into the string that the tokenizer is parsing.
/// Each token has a `TokenType` indicating what it is and its data.

use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::fmt::Result as FmtResult;

use lex::{Location, Span, CowStr};

/// A token returned by the tokenizer.
///
/// Each token has a definite beginning position in the file,
/// a string, and its `TokenData` value - an enum of literals,
/// identifier name, or various keywords.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Token {
    /// Location of the token in a file
    location: Location,
    /// Text of the token at that location
    text: CowStr,
    /// Additional data (type/literal) provided by the lexer
    data: TokenData
}

impl Token {
    /// Gets the original source text of this token.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// The data associated with this token
    pub fn data(&self) -> &TokenData {
        &self.data
    }

    /// The location of this token where it starts in its source text
    pub fn location(&self) -> Location {
        self.location
    }

    /// Get the span of this token including its source text
    pub fn span(&self) -> Span {
        Span::in_line(self.location, self.text.len() as u32)
    }

    /// Creates a new token with the given information.
    pub fn new<T: Into<CowStr>>(text: T,
                                location: Location,
                                data: TokenData) -> Token {
        Token { text: text.into(), location, data }
    }

    /// Creates a new token representing an identifier
    pub fn new_ident<T: Into<CowStr>>(text: T, location: Location) -> Token {
        Token {
            text: text.into(),
            data: TokenData::Ident,
            location: location
        }
    }

    /// Creates a new token representing an indentation
    pub fn new_indent(location: Location) -> Token {
        Token {
            text: Cow::Borrowed(""),
            data: TokenData::BeginBlock,
            location: location
        }
    }

    /// Creates a new token representing an outdentation
    pub fn new_outdent(location: Location) -> Token {
        Token {
            text: Cow::Borrowed(""),
            data: TokenData::EndBlock,
            location: location
        }
    }

    /// Creates a new token representing an EOF
    pub fn new_eof(location: Location) -> Token {
        Token {
            text: Cow::Borrowed(""),
            data: TokenData::EOF,
            location: location
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "({:?}, {:?})", self.get_type(), self.text)
    }
}

impl Eq for Token { }

/// Token enum - tokens are pretty simple, mostly dependent on string matching.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenData {
    /// Token is a numeric literal
    NumberLiteral(f64),
    /// Token is unit type literal `()`
    UnitLiteral,
    /// Token is boolean literal `true` or `false`
    BoolLiteral(bool),
    /// Token is some name
    Ident,
    /// Token is a keyword
    Keyword,
    /// Token is a shortcut for the name of a type.
    TypeName,
    /// Token is some symbol
    Symbol,
    /// Indendation of block
    BeginBlock,
    /// Outdendation of block
    EndBlock,
    /// Token is an EOF
    EOF
}

impl Default for TokenData {
    fn default() -> TokenData {
        TokenData::EOF
    }
}
