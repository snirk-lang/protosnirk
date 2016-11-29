/// The token is a slice into the string that the tokenizer is parsing.
/// Each token has a `TokenType` indicating what it is and its data.

use std::borrow::Cow;
use std::collections::HashSet;
use std::ops::Range;

/// A token returned by the tokenizer.
///
/// Each token has a definite
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    /// Location of the token in a file
    pub location: TextLocation,
    /// Text of the token at that location
    pub text: Cow<'static, str>,
    /// Additional data (type/literal) provided by the lexer
    pub data: TokenData
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

    BeginBock,
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
            BeginBock | EndBlock => TokenType::Block,
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
    Block,
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
