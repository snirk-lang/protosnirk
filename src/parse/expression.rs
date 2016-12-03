//! Expression types

use lex::{CowStr, Token, TokenType};
use parse::ParseError;

/// Variable declaration
#[derive(Debug, PartialEq, Clone)]
pub struct Declaration {
    name: CowStr,
    mutable: bool,
    value: Box<Expression>
}
impl Declaration {
    pub fn new(name: CowStr, mutable: bool, value: Box<Expression>) -> Self {
        Declaration { name: name, mutable: mutable, value: value }
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_value(&self) -> &Expression {
        &self.value
    }
    pub fn is_mut(&self) -> bool {
        self.mutable
    }
}

/// Literal value
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Literal(f64);
impl Literal {
    pub fn new(value: f64) -> Self {
        Literal(value)
    }
    pub fn get_value(&self) -> f64 {
        self.0
    }
}

/// Reference to a Variable
/// the name of the variable...
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Identifier(CowStr);

impl Identifier {
    pub fn new(name: CowStr) -> Self {
        Identifier(name)
    }
    pub fn get_name(&self) -> &str {
        &self.0
    }
}
impl Into<CowStr> for Identifier {
    fn into(self) -> CowStr {
        self.0
    }
}

/// Maths style binary operations (may be split up later)
#[derive(Debug, PartialEq, Clone)]
pub struct BinaryOperation {
    operator: Token,
    left: Box<Expression>,
    right: Box<Expression>
}
impl BinaryOperation {
    pub fn new(operator: Token, left: Box<Expression>, right: Box<Expression>) -> BinaryOperation {
        BinaryOperation {
            operator: operator,
            left: left,
            right: right
        }
    }
    pub fn get_operator(&self) -> &Token {
        &self.operator
    }
}

/// Unary operation
#[derive(Debug, PartialEq, Clone)]
pub struct UnaryOperation {
    operator: Token,
    expression: Box<Expression>
}
impl UnaryOperation {
    /// Creates a new unary operation
    pub fn new(operator: Token, expression: Box<Expression>) -> UnaryOperation {
        UnaryOperation { operator: operator, expression: expression }
    }
}

/// An identifier is assigned to a value
#[derive(Debug, PartialEq, Clone)]
pub struct Assignment {
    lvalue: Identifier,
    rvalue: Box<Expression>
}
impl Assignment {
    pub fn new(name: Identifier, value: Box<Expression>) -> Assignment {
        Assignment { lvalue: name, rvalue: value }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Return {
    value: Option<Box<Expression>>
}
impl Return {
    pub fn new<V: Into<Option<Box<Expression>>>>(value: V) -> Return {
        Return { value: value.into() }
    }
}

/// Expression
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    /// Literal value in source code
    Literal(Literal),
    /// Value of an identifier is being used
    VariableRef(Identifier),
    /// Binary operation
    BinaryOp(BinaryOperation),
    /// Unary operation
    UnaryOp(UnaryOperation),

    /// Block of statements with a return
    Block(Vec<Expression>),

    // non-expressions should return () or not be allowed in stmt positions later on

    /// Declaring a new variable
    Declaration(Declaration),
    /// Assigning a mutable variable
    Assignment(Assignment),
    /// Returning an expression
    Return(Return),
}
impl Expression {
    /// If this expression is a statement and shouldn't be trusted to represent a value
    pub fn is_statement(&self) -> bool {
        use self::Expression::*;
        match *self {
            Declaration(_) | Assignment(_) | Return(_) => true,
            _ => false
        }
    }

    /// Return an error if this expression cannot be used as an "lvalue"
    pub fn expect_identifier(self) -> Result<Identifier, ParseError> {
        match self {
            Expression::VariableRef(ident) => Ok(ident),
            other => {
                Err(ParseError::ExpectedLValue(other))
            }
        }
    }

    /// Return an error if this expression cannot be used as an "rvalue"
    pub fn expect_value(self) -> Result<Expression, ParseError> {
        if !self.is_statement() {
            Ok(self)
        } else {
            Err(ParseError::ExpectedRValue(self))
        }
    }
}

/// What type an expression is. Useful for filtering out
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ExpressionType {
    /// The expression identifies a literal
    Literal,
    /// The expression identifies a name
    Name,
    /// The expression can be resolved to some value
    Value,
    /// The expression is a satement and shouldn't be trusted to be a value
    Statement
}
