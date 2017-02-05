//! Expression values
//!
//! Expression values are used in the `Expression` and `Statement` contexts.
//! They are usually emitted as asm instructions operating on variables.

use lex::{Token, TokenType, TokenData};
use parse::{ParseResult, ParseError, ExpectedNextType};
use parse::ast::{Statement, Identifier, Operator};

/// Expression types
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    /// Literal value expression
    Literal(Literal),
    /// Variable reference
    VariableRef(Identifier),
    /// Binary operation
    BinaryOp(BinaryOperation),
    /// Unary operation
    UnaryOp(UnaryOperation),

    // "Non-value expressions"
    // I _guess_ they could return `()`, but why?

    /// Assignment - not considered value expression
    Assignment(Assignment),
    /// Declaration - not considered value expression
    Declaration(Declaration),
}
impl Expression {
    /// Convert this expression to a `Statement::Expression`
    #[inline]
    pub fn to_statement(self) -> Statement {
        Statement::Expression(self)
    }
    /// Whether this expression has value.
    ///
    /// In typeless protosnirk, this revolves around
    /// assignments and declarations being expressions
    /// of type `()`. However, they will be disallowed
    /// from being used to represent `()`.
    pub fn has_value(&self) -> bool {
        match *self {
            Expression::Assignment(_) | Expression::Declaration(_) => false,
            _ => true
        }
    }
    pub fn expect_value(self) -> ParseResult<Expression> {
        if !self.has_value() {
            Err(ParseError::ExpectedExpression {
                expected: ExpectedNextType::AnyExpression,
                got: self
            })
        } else {
            Ok(self)
        }
    }
    pub fn expect_identifier(self) -> ParseResult<Identifier> {
        match self {
            Expression::VariableRef(ident) => Ok(ident),
            other => Err(ParseError::ExpectedLValue(other))
        }
    }
}

/// Literal value
#[derive(Debug, PartialEq, Clone)]
pub struct Literal {
    pub token: Token
}
impl Literal {
    pub fn new(token: Token) -> Self {
        debug_assert!(token.data.get_type() == TokenType::Literal,
            "Literal token created with bad token {:?}", token);
        Literal {
            token: token
        }
    }
    pub fn get_value(&self) -> f64 {
        match self.token.data {
            TokenData::NumberLiteral(num) => num,
            ref bad => panic!("Invalid token {:?} owned by Literal", bad)
        }
    }
}

/// Maths style binary operations (may be split up later)
#[derive(Debug, PartialEq, Clone)]
pub struct BinaryOperation {
    pub operator: Operator,
    pub op_token: Token,
    pub left: Box<Expression>,
    pub right: Box<Expression>
}
impl BinaryOperation {
    pub fn new(operator: Operator, op_token: Token,
        left: Box<Expression>, right: Box<Expression>) -> BinaryOperation {
        BinaryOperation {
            operator: operator,
            op_token: op_token,
            left: left,
            right: right
        }
    }
    pub fn get_operator(&self) -> Operator {
        self.operator
    }
}

/// Unary operation
#[derive(Debug, PartialEq, Clone)]
pub struct UnaryOperation {
    pub operator: Operator,
    pub op_token: Token,
    pub expression: Box<Expression>
}
impl UnaryOperation {
    /// Creates a new unary operation
    pub fn new(operator: Operator, op_token: Token, expression: Box<Expression>) -> UnaryOperation {
        UnaryOperation {
            operator: operator,
            op_token: op_token,
            expression: expression
        }
    }
}

/// Variable declaration
#[derive(Debug, PartialEq, Clone)]
pub struct Declaration {
    pub mutable: bool,
    pub token: Token,
    pub ident: Identifier,
    pub value: Box<Expression>
}
impl Declaration {
    pub fn new(token: Token, mutable: bool, ident: Identifier, value: Box<Expression>) -> Self {
        Declaration { token: token, mutable: mutable, ident: ident, value: value }
    }
    pub fn get_name(&self) -> &str {
        &self.ident.get_name()
    }
    pub fn get_value(&self) -> &Expression {
        &self.value
    }
    pub fn is_mut(&self) -> bool {
        self.mutable
    }
    pub fn get_ident(&self) -> &Identifier {
        &self.ident
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
