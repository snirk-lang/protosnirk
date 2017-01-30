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
    Declaration(Declaration),
    Assignment(Assignment),
    Return(Return),
    DoBlock(DoBlock),
    // if, if let, match, loop, while, for
}
impl Statement {
    pub fn has_value(&self) -> bool {
        match *self {
            Statement::Expression(_) => true,
            Statement::Assignment(_) => false,
            Statement::DoBlock(ref inner) => inner.has_value(),
        }
    }
}

/// Variable declaration
#[derive(Debug, PartialEq, Clone)]
pub struct Declaration {
    pub mutable: bool,
    pub token: Token,
    pub value: Box<Expression>
}
impl Declaration {
    pub fn new(token: Token, mutable: bool, value: Box<Expression>) -> Self {
        Declaration { token: token, mutable: mutable, value: value }
    }
    pub fn get_name(&self) -> &str {
        &self.token.text
    }
    pub fn get_value(&self) -> &Expression {
        &self.value
    }
    pub fn is_mut(&self) -> bool {
        self.mutable
    }
}

/// An identifier is assigned to a value
#[derive(Debug, PartialEq, Clone)]
pub struct Assignment {
    pub lvalue: Identifier,
    pub rvalue: Box<Expression>
}
impl Assignment {
    pub fn new(name: Identifier, value: Box<Expression>) -> Assignment {
        Assignment { lvalue: name, rvalue: value }
    }
}

/// Do <block> statement.
#[derive(Debug, PartialEq, Clone)]
pub struct DoBlock {
    pub do_token: Token,
    pub block: Box<Block>
}
impl DoBlock {
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
