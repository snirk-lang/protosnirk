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
#[derive(Debug, Clone, Default)]
pub struct Token {
    /// Location of the token in a file
    start: Location,
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
    pub fn start(&self) -> Location {
        self.start
    }

    pub fn end(&self) -> Location {
        self.start.offset(self.text.len() as u32)
    }

    /// Get the span of this token including its source text
    pub fn span(&self) -> Span {
        Span::from(self.start ..= self.start.offset(self.text.len() as u32))
    }

    /// Creates a new token with the given information.
    pub fn new<T: Into<CowStr>>(text: T,
                                start: Location,
                                data: TokenData) -> Token {
        Token { text: text.into(), start, data }
    }

    /// Creates a new token representing an identifier
    pub fn new_ident<T: Into<CowStr>>(text: T, start: Location) -> Token {
        Token {
            text: text.into(),
            data: TokenData::Ident,
            start
        }
    }

    /// Creates a new token representing an indentation
    pub fn new_indent(start: Location) -> Token {
        Token {
            text: Cow::Borrowed("    "),
            data: TokenData::BeginBlock,
            start
        }
    }

    /// Creates a new token representing an outdentation
    pub fn new_outdent(start: Location) -> Token {
        Token {
            text: Cow::Borrowed(""),
            data: TokenData::EndBlock,
            start
        }
    }

    /// Creates a new token representing an EOF
    pub fn new_eof(start: Location) -> Token {
        Token {
            text: Cow::Borrowed(""),
            data: TokenData::EOF,
            start
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "({:?}, {:?})", self.get_type(), self.text)
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Token) -> bool {
        self.text == other.text && self.start == other.start
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
    /// Token is some symbol
    Symbol,
    /// Token is a keyword
    Keyword,
    /// Token is a shortcut for the name of a type.
    TypeName,

    /// Indendation of block
    BeginBlock,
    /// Outdendation of block
    EndBlock,
    /// Token represents the end of a line
    EndLine,
    /// Token is an EOF
    EOF
}

impl Default for TokenData {
    fn default() -> TokenData {
        TokenData::EOF
    }
}
