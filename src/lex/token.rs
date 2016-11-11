/// The token is a slice into the string that the tokenizer is parsing.
/// Each token has a `TokenType` indicating what it is and its data.

use std::ops::Range;

/// The token struct.
#[derive(Debug, PartialEq, Clone)]
pub struct Token<'a> {
    range: Range<usize>,
    text: &'a str,
    type_: TokenType
}

/// What kind of token this is.
///
/// Similar token types are grouped together (i.e. brackets) into sub-enums.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TokenType {
    /// A literal number.
    Literal,
    /// An identifier.
    /// See `.get_text()` for the value within.
    Identifier,
    /// Assignment operator `=`
    Assign,
    /// Declaration, `let` or `mut`
    Declare(DeclareType),
    /// An infix operator, such as `and`, `or`, `+`, `%`
    InfixOperator(InfixType),
    /// A prefix operator, such as `-` or `not`
    PrefixOperator(PrefixType),
    /// A bracket, one of `{[(<>)]}`
    Bracket(BracketType, bool),
    /// Special token used to indicate the end of the file.
    EOF
}

/// A type of
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DeclareType {
    Let,
    Mut
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum InfixType {
    Add,
    Sub,
    Mul,
    Div,
    Mod
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum PrefixType {
    Negate
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum BracketType {
    Paren,
    Square,
    Angle,
    Squiggle
}

impl<'a> Token<'a> {
    pub fn new(start: usize, end: usize, text: &'a str, type_: TokenType)
                   -> Token<'a> {
        debug_assert!(start <= end,
                      "range: start {} <= end {}", start, end);
        debug_assert!(text.len() > 0,
                      "text: empty string {}", text);
        debug_assert!(text.len() == end - start,
                      "text: {} does not match range {:?}", text, start .. end);

        Token {
            range: start .. end,
            text: text,
            type_: type_
        }
    }

    pub fn get_type(&self) -> TokenType {
        self.type_
    }
    pub fn get_text(&self) -> &str {
        self.text
    }
    pub fn range(&self) -> &Range<usize> {
        &self.range
    }
    pub fn start(&self) -> usize {
        self.range.start
    }
    pub fn end(&self) -> usize {
        self.range.end
    }
}
