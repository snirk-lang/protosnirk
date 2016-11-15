/// The token is a slice into the string that the tokenizer is parsing.
/// Each token has a `TokenType` indicating what it is and its data.

use std::ops::Range;

/// The token struct.
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    range: Range<usize>,
    text: String,
    type_: TokenType
}

/// Enum representing which type of token this is.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TokenType {
    /// An identifier
    Identifier,
    /// A numeric literal
    NumberLiteral,

    /// +
    Plus,
    /// -
    Minus,
    /// *
    Star,
    /// /
    Slash,
    /// =
    Assign,
    /// %
    Percent,

    /// let
    Let,
    /// mut
    Mut,
    /// return
    Return,

    /// (
    LeftParen,
    /// )
    RightParen,
    /// [
    LeftBrace,
    /// ]
    RightBrace,
    /// {
    LeftSquiggle,
    /// }
    RightSquiggle,
    /// <
    LeftAngle,
    /// >
    RightAngle,

    /// Abstract token used to indicate an increase in indentation
    IncreaseIndent,
    /// Abstract token used to indiciate a decrease in indentation
    EndBlock,
    /// Token used to indicate EOF.
    End
}

/// Which category of token the given `TokenType` is.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TokenCategory {
    /// Token is used to declare a variable
    Declare,
    /// Token is used what why
    Statement,
    /// It's a name duh
    Name,
    /// Token is used literally
    Literal,
    /// Token is used to operate
    Operator,
    /// Token is parenthetical
    Paren,
    /// Token is used to indicate indentation
    Control
}

impl Token {
    pub fn new<S: Into<String>>(start: usize, end: usize, text: S, type_: TokenType)
                   -> Token {
        let into_text: String = text.into();
        debug_assert!(start <= end,
                      "range: start {} <= end {}", start, end);
        debug_assert!(into_text.len() > 0,
                      "text: empty string {}", into_text);
        debug_assert!(into_text.len() == end - start,
                      "text: {} does not match range {:?}", into_text, start .. end);

        Token {
            range: start .. end,
            text: into_text,
            type_: type_
        }
    }

    pub fn end(index: usize) -> Token {
        Token {
            range: index .. index,
            text: "".into(),
            type_: TokenType::End
        }
    }

    pub fn get_type(&self) -> TokenType {
        self.type_
    }
    pub fn get_text(&self) -> &String {
        &self.text
    }
    pub fn range(&self) -> &Range<usize> {
        &self.range
    }
}
impl Into<String> for Token {
    fn into(self) -> String {
        self.text
    }
}

impl TokenType {
    pub fn get_category(&self) -> TokenCategory {
        use self::TokenType::*;
        match *self {
            Plus | Minus | Star | Slash | Percent =>
                TokenCategory::Operator,

            Let | Mut | Assign =>
                TokenCategory::Declare,

            LeftParen | LeftBrace | LeftSquiggle | LeftAngle |
            RightParen | RightBrace | RightSquiggle | RightAngle =>
                TokenCategory::Paren,

            Identifier => TokenCategory::Name,
            NumberLiteral => TokenCategory::Literal,
            Return => TokenCategory::Statement,
            IncreaseIndent | EndBlock | End => TokenCategory::Control
        }
    }
}
