//! Error handling in parsers

use lex::{CowStr, Token, TokenType};
use parse::ast::{Expression};

/// Result given from main and expression parsers
pub type ParseResult<T> = Result<T, ParseError>;

/// Error given from parsers
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    ExpectedToken {
        expected: TokenType,
        got: Token
    },
    ExpectedExpression {
        expected: ExpectedNextType,
        got: Expression
    },
    ExpectedLValue(Expression),
    ExpectedRValue(Expression),
    UnknownOperator {
        text: CowStr,
        token_type: TokenType
    },
    EOF,
    LazyString(String)
}

/// Information of what the parser was expecting to get
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExpectedNextType {
    AnyStatement,
    AnyExpression,
    AnyItem,
    Lvalue,
    Rvalue,
    SpecificToken(CowStr),
}
