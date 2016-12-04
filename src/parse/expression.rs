//! Expression types

use lex::{CowStr, Token, TokenData, TokenType};
use parse::ParseError;

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

/// Reference to a Variable
/// the name of the variable...
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Identifier {
    pub token: Token
}

impl Identifier {
    pub fn new(token: Token) -> Self {
        Identifier { token: token }
    }
    pub fn get_name(&self) -> &str {
        &self.token.text
    }
}
impl Into<Token> for Identifier {
    fn into(self) -> Token {
        self.token
    }
}

/// Maths style binary operations (may be split up later)
#[derive(Debug, PartialEq, Clone)]
pub struct BinaryOperation {
    pub operator: Token,
    pub left: Box<Expression>,
    pub right: Box<Expression>
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
    pub operator: Token,
    pub expression: Box<Expression>
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
    pub lvalue: Identifier,
    pub rvalue: Box<Expression>
}
impl Assignment {
    pub fn new(name: Identifier, value: Box<Expression>) -> Assignment {
        Assignment { lvalue: name, rvalue: value }
    }
}

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
