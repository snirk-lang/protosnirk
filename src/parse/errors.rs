//! Error handling in parsers

use lex::{CowStr, Token, TokenType};
use parse::expression::{Expression, ExpressionType};
use parse::verify::ErrorCollector;

/// Result given from main and expression parsers
pub type ParseResult = Result<Expression, ParseError>;


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
    VerifierError {
        collection: ErrorCollector
    },
    UnknownOperator {
        text: CowStr,
        token_type: TokenType
    },
    LazyString(String)
}
