//! Expression values
//!
//! Expression values are used in the `Expression` and `Statement` contexts.
//! They are usually emitted as asm instructions operating on variables.

use lex::{Token, TokenType, TokenData, Span, Location};
use ast::{ScopedId, Identifier, UnaryOperator, BinaryOperator};
use parse::{ParseResult, ParseError, ExpectedNextType};

use std::cell::Ref;

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
    // See https://github.com/immington-industries/protosnirk/issues/30

    /// Assignment - not considered value expression
    Assignment(Assignment),
}

impl Expression {
    /// Whether this expression has value.
    ///
    /// In typeless protosnirk, this revolves around
    /// assignments and declarations being expressions
    /// of type `()`. However, they will be disallowed
    /// from being used to represent `()`.
    pub fn has_value(&self) -> bool {
        match *self {
            Expression::Assignment(_) => false,
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

    pub fn span(&self) -> Span {
        use self::Expression::*;
        match self {
            Literal(ref l) => l.span(),
            VariableRef(ref v) => v.span(),
            Assignment(ref a) => a.span(),
            BinaryOp(ref b) => b.span(),
            FnCall(ref f) => f.span(),
            IfExpression(ref i) => i.span(),
            UnaryOp(ref u) => u.span()
        }
    }
}

/// Values held by a literal.
#[derive(Debug, PartialEq, Clone)]
pub enum LiteralValue {
    /// Literals `true` and `false`
    Bool(bool),
    /// Numeric literals
    Float(f64),
    /// `()`
    Unit
}

/// Represents a literal expression, such as a boolean or number.
#[derive(Debug, PartialEq, Clone)]
pub struct Literal {
    token: Token,
    value: LiteralValue
}
impl Literal {
    /// Creates a new `Literal` from the given token and value.
    pub fn new(token: Token, value: LiteralValue) -> Literal {
        debug_assert!(token.get_type() == TokenType::Literal,
            "Literal token created with bad token {:?}", token);
        Literal {
            token, value
        }
    }
    /// Creates a new boolean literal from the given token and boolean value.
    pub fn new_bool(token: Token, value: bool) -> Literal {
        debug_assert!(
            match token.data() {
                TokenData::BoolLiteral => true, _ => false
            },
            "Literal bool created with bad token {:?}", token);
        Literal {
            token: token,
            value: LiteralValue::Bool(value)
        }
    }

    /// Creates a new unit type literal `()` from the given token.
    pub fn new_unit(token: Token) -> Literal {
        debug_assert!(
            match token.data() {
                TokenData::UnitLiteral => true, _ => false
            },
            "Literal unit created with bad token {:?}", token);
        Literal {
            token,
            value: LiteralValue::Unit
        }
    }

    /// Creates a new floating point literal from the given token and value.
    pub fn new_float(token: Token, value: f64) -> Literal {
        debug_assert!(
            match token.data() {
                TokenData::NumberLiteral => true, _ => false
            },
            "Literal f64 called with bad token {:?}", token);
        Literal {
            token,
            value: LiteralValue::Float(value)
        }
    }

    pub fn text(&self) -> &str {
        self.token.text()
    }

    /// Gets the `LiteralValue` of this literal expression.
    pub fn value(&self) -> &LiteralValue {
        &self.value
    }

    /// Gets the span of the literal [token]
    pub fn span(&self) -> Span {
        self.token.span()
    }
}

/// Maths style binary operations (may be split up later)
#[derive(Debug, PartialEq, Clone)]
pub struct BinaryOperation {
    operator: BinaryOperator,
    left: Box<Expression>,
    right: Box<Expression>,
    span: Span
}
impl BinaryOperation {
    pub fn new(operator: BinaryOperator,
               left: Box<Expression>,
               right: Box<Expression>) -> BinaryOperation {
        BinaryOperation {
            span: Span::from(left.span() ..= right.span()),
            operator: operator,
            left: left,
            right: right
        }
    }
    pub fn operator(&self) -> BinaryOperator {
        self.operator
    }
    pub fn left(&self) -> &Expression {
        &self.left
    }
    pub fn right(&self) -> &Expression {
        &self.right
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

/// Unary operation
#[derive(Debug, PartialEq, Clone)]
pub struct UnaryOperation {
    operator: UnaryOperator,
    expression: Box<Expression>,
    span: Span
}
impl UnaryOperation {
    /// Creates a new unary operation
    pub fn new(start: Location,
               operator: UnaryOperator,
               expression: Box<Expression>) -> UnaryOperation {
        UnaryOperation {
            span: Span::from(start ..= expression.span().end()),
            operator: operator,
            expression: expression
        }
    }

    pub fn operator(&self) -> UnaryOperator {
        self.operator
    }

    pub fn inner(&self) -> &Expression {
        &self.expression
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

/// An identifier is assigned to a value
#[derive(Debug, PartialEq, Clone)]
pub struct Assignment {
    lvalue: Identifier,
    rvalue: Box<Expression>,
}
impl Assignment {
    pub fn new(name: Identifier, value: Box<Expression>) -> Assignment {
        Assignment {
            lvalue: name,
            rvalue: value,
        }
    }
    pub fn lvalue(&self) -> &Identifier {
        &self.lvalue
    }
    pub fn rvalue(&self) -> &Expression {
        &self.rvalue
    }

    pub fn span(&self) -> Span {
        Span::from(self.lvalue.span() ..= self.rvalue.span())
    }
}

/// Inline if expression using `=>`
#[derive(Debug, PartialEq, Clone)]
pub struct IfExpression {
    condition: Box<Expression>,
    true_expr: Box<Expression>,
    else_expr: Box<Expression>,
    span: Span
}
impl IfExpression {
    pub fn new(start: Location,
               condition: Box<Expression>,
               true_expr: Box<Expression>,
               else_expr: Box<Expression>) -> IfExpression {
        IfExpression {
            span: Span::from(start ..= else_expr.span().end()),
            condition: condition,
            true_expr: true_expr,
            else_expr: else_expr
        }
    }
    pub fn condition(&self) -> &Expression {
        &self.condition
    }
    pub fn true_expr(&self) -> &Expression {
        &self.true_expr
    }
    pub fn else_expr(&self) -> &Expression {
        &self.else_expr
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

/// Represents invocation of a function
#[derive(Debug, PartialEq, Clone)]
pub struct FnCall {
    lvalue: Identifier,
    args: Vec<CallArgument>,
    span: Span
}

impl FnCall {
    pub fn new(span: Span,
               lvalue: Identifier,
               args: Vec<CallArgument>) -> FnCall {
        FnCall { lvalue, args, span }
    }
    pub fn ident(&self) -> &Identifier {
        &self.lvalue
    }
    pub fn text(&self) -> &str {
        self.ident().name()
    }
    pub fn args(&self) -> &[CallArgument] {
        &self.args
    }

    pub fn id<'a>(&'a self) -> Ref<'a, ScopedId> {
        self.ident().id()
    }
    pub fn set_id(&self, id: ScopedId) {
        self.ident().set_id(id);
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

/// Represents arguments given to call a function with
#[derive(Debug, PartialEq, Clone)]
pub struct CallArgument {
    param: Identifier,
    value: Expression,
}
impl CallArgument {
    pub fn named(param: Identifier, value: Expression) -> CallArgument {
        CallArgument { param, value }
    }

    /// Gets the value of the CallArgument.
    pub fn expression(&self) -> &Expression {
        &self.value
    }

    pub fn name(&self) -> &Identifier {
        &self.param
    }

    pub fn span(&self) -> Span {
        Span::from(self.param.span() ..= self.value.span())
    }
}
