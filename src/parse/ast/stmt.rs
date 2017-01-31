/// Statement values
///
/// Function bodies are usually made up of statements. They include complex blocks
/// such as loop constructs. They are usually not accepted in as many places as
/// `Expression`s are because of their ability to use indentation.

use lex::{CowStr, Token, TokenData, TokenType};
use parse::ast::{Expression, Block, Identifier};

/// Statement representation
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Expression(Expression),
    Return(Return),
    DoBlock(DoBlock),
    // if, if let, match, loop, while, for
}
impl Statement {
    pub fn has_value(&self) -> bool {
        match *self {
            Statement::Expression(ref inner) => inner.has_value(),
            Statement::DoBlock(ref inner) => inner.has_value(),
            Statement::Return(ref inner) =>
                inner.value.map(|expr| expr.has_value()).unwrap_or(false)
        }
    }
}

/// Explicit return statement
#[derive(Debug, PartialEq, Clone)]
pub struct Return {
    pub token: Token,
    pub value: Option<Box<Expression>>
}
impl Return {
    pub fn new<V: Into<Option<Box<Expression>>>>(token: Token, value: V) -> Return {
        Return { token: token, value: value.into() }
    }
}

/// Do <block> statement.
#[derive(Debug, PartialEq, Clone)]
pub struct DoBlock {
    pub token: Token,
    pub block: Box<Block>
}
impl DoBlock {
    pub fn new(token: Token, block: Box<Block>) -> DoBlock {
        DoBlock { token: token, block: block }
    }
    pub fn has_value(&self) -> bool {
        self.block.has_value()
    }
}
