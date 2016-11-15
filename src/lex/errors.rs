//! Error handling in parsers

use lex::{Expression, ExpressionType, Token, TokenType};

/// Result given from main and expression parsers
pub type ParseResult = Result<Expression, ParseError>;

pub type TokenResult<'a> = Result<Token, ParseError>;

/// Error given from parsers
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    ExpectedToken {
        expected: TokenType,
        got: Token
    },
    ExpectedExpression {
        expected: ExpressionType,
        got: Expression
    },
    ExpectedLValue(Expression),
    ExpectedRValue(Expression),
    GenericError {

    },
    LazyString(String)
}
