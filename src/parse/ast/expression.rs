//! Expression values
//!
//! Expression values are used in the `Expression` and `Statement` contexts.
//! They are usually emitted as asm instructions operating on variables.

use lex::{Token, TokenType, TokenData};
use parse::{ParseResult, ParseError, ExpectedNextType};
use parse::ast::{Statement, Identifier, Operator, Block};

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
    /// If expression
    IfExpression(IfExpression),
    /// Invocation of a funciton with standard named arg setup.
    FnCall(FnCall),
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
    pub fn get_left(&self) -> &Expression {
        &self.left
    }
    pub fn get_right(&self) -> &Expression {
        &self.right
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
    pub fn get_operator(&self) -> &Operator {
        &self.operator
    }
    pub fn get_inner(&self) -> &Expression {
        &self.expression
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
    pub fn get_token(&self) -> &Token {
        &self.token
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
    pub fn get_lvalue(&self) -> &Identifier {
        &self.lvalue
    }
    pub fn get_rvalue(&self) -> &Expression {
        &self.rvalue
    }
}

/// Inline if expression using `=>`
#[derive(Debug, PartialEq, Clone)]
pub struct IfExpression {
    if_token: Token,
    condition: Box<Expression>,
    true_expr: Box<Expression>,
    else_expr: Box<Expression>
}
impl IfExpression {
    pub fn new(if_token: Token, condition: Box<Expression>,
               true_expr: Box<Expression>, else_expr: Box<Expression>) -> IfExpression {
        IfExpression {
            if_token: if_token,
            condition: condition,
            true_expr: true_expr,
            else_expr: else_expr
        }
    }
    pub fn get_token(&self) -> &Token {
        &self.if_token
    }
    pub fn get_condition(&self) -> &Expression {
        &self.condition
    }
    pub fn get_true_expr(&self) -> &Expression {
        &self.true_expr
    }
    pub fn get_else(&self) -> &Expression {
        &self.else_expr
    }
}

/// Represents invocation of a function
#[derive(Debug, PartialEq, Clone)]
pub struct FnCall {
    lvalue: Identifier,
    paren_token: Token,
    args: FnCallArgs
}

impl FnCall {
    pub fn new(lvalue: Identifier, token: Token, args: FnCallArgs) -> FnCall {
        FnCall { lvalue: lvalue, paren_token: token, args: args }
    }
    pub fn named(lvalue: Identifier, token: Token, args: Vec<CallArgument>) -> FnCall {
        FnCall { lvalue: lvalue, paren_token: token, args: FnCallArgs::Arguments(args) }
    }
    pub fn single_expr(lvalue: Identifier, token: Token, arg: Expression) -> FnCall {
        FnCall {
            lvalue: lvalue,
            paren_token: token,
            args: FnCallArgs::SingleExpr(Box::new(arg))
        }
    }
    pub fn get_name(&self) -> &Identifier {
        &self.lvalue
    }
    pub fn get_token(&self) -> &Token {
        &self.paren_token
    }
    pub fn get_args(&self) -> &FnCallArgs {
        &self.args
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum FnCallArgs {
    /// Function was called with a single expression
    SingleExpr(Box<Expression>),
    /// Function was called with a list of arguments
    Arguments(Vec<CallArgument>)
}

/// Represents arguments given to call a function with
#[derive(Debug, PartialEq, Clone)]
pub struct CallArgument {
    name: Identifier,
    expr: Option<Expression>
}
impl CallArgument {
    pub fn var_name(name: Identifier) -> CallArgument {
        CallArgument { name: name, expr: None }
    }
    pub fn var_value(name: Identifier, expr: Expression) -> CallArgument {
        CallArgument { name: name, expr: Some(expr) }
    }
    pub fn get_name(&self) -> &Identifier {
        &self.name
    }
    pub fn has_value(&self) -> bool {
        self.expr.is_some()
    }
    pub fn get_expr(&self) -> Option<&Expression> {
        self.expr.as_ref()
    }
}
