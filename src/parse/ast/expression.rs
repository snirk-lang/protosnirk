//! Expression values
//!
//! Expression values are used in the `Expression` and `Statement` contexts.
//! They are usually emitted as asm instructions operating on variables.

use lex::{Token, TokenType, TokenData};
use parse::{ParseResult, ParseError, ExpectedNextType, ScopedId};
use parse::ast::{Statement, Identifier, Operator, Block};
use parse::ast::types::TypeExpression;

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
    // I _guess_ they could return `()`, but why?

    /// Assignment - not considered value expression
    Assignment(Assignment),
    /// Declaration - not considered value expression
    // TODO this should be parsed as a statement
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
            match *token.get_data() {
                TokenData::BoolLiteral(_) => true, _ => false
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
            match *token.get_data() {
                TokenData::UnitLiteral => true, _ => false
            },
            "Literal unit created with bad token {:?}", token);
        Literal {
            token,
            value: LiteralValue::Unit
        }
    }

    /// Creates a new floating point literal from the given token and value.
    pub fn new_f64(token: Token, value: f64) -> Literal {
        debug_assert!(
            match *token.get_data() {
                TokenData::NumberLiteral(_) => true, _ => false
            },
            "Literal f64 called with bad token {:?}", token);
        Literal {
            token,
            value: LiteralValue::Float(value)
        }
    }

    /// Gets the `LiteralValue` of this literal expression.
    pub fn get_value(&self) -> &LiteralValue {
        &self.value
    }
    /// Gets the token of this literal.
    pub fn get_token(&self) -> &Token {
        &self.token
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
    pub ident: Identifier,
    pub value: Box<Expression>,
    type_decl: Option<TypeExpression>
}
impl Declaration {
    pub fn new(ident: Identifier,
               mutable: bool,
               type_decl: Option<TypeExpression>,
               value: Box<Expression>) -> Declaration {
        Declaration { ident, mutable, type_decl, value }
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
        self.ident.get_token()
    }
    pub fn get_type_decl(&self) -> Option<&TypeExpression> {
        self.type_decl.as_ref()
    }
    pub fn has_declared_type(&self) -> bool {
        self.type_decl.is_some()
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
    pub fn get_ident(&self) -> &Identifier {
        &self.lvalue
    }
    pub fn get_text(&self) -> &str {
        self.get_ident().get_name()
    }
    pub fn get_token(&self) -> &Token {
        &self.paren_token
    }
    pub fn get_args(&self) -> &FnCallArgs {
        &self.args
    }

    pub fn get_id<'a>(&'a self) -> Ref<'a, ScopedId> {
        self.get_ident().get_id()
    }
    pub fn set_id(&self, id: ScopedId) {
        self.get_ident().set_id(id);
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum FnCallArgs {
    /// Function was called with a single expression
    SingleExpr(Box<Expression>),
    /// Function was called with a list of arguments
    Arguments(Vec<CallArgument>)
}
impl FnCallArgs {
    pub fn len(&self) -> usize {
        match *self {
            FnCallArgs::SingleExpr(_) => 1,
            FnCallArgs::Arguments(ref args) => args.len()
        }
    }
}

/// The value of an argument to a function call.
///
/// This is either the implicit local variable or an expression.
#[derive(Debug, PartialEq, Clone)]
pub enum CallArgumentValue {
    /// Argument is just a name that refers to both the param and a local name.
    ///
    /// This `Identifier` has the `ScopedId` of the local variable.
    LocalVar(Identifier),
    /// Argument is an expression.
    Expression(Expression)
}

/// Represents arguments given to call a function with
#[derive(Debug, PartialEq, Clone)]
pub struct CallArgument {
    param: Identifier,
    value: CallArgumentValue
}
impl CallArgument {
    pub fn implicit_name(param: Identifier) -> CallArgument {
        CallArgument { param: param.clone(), value: CallArgumentValue::LocalVar(param.clone()) }
    }
    pub fn named(param: Identifier, value: Expression) -> CallArgument {
        CallArgument { param, value: CallArgumentValue::Expression(value) }
    }

    /// Gets the name of the param being referenced.
    ///
    /// The `ScopedId` of this `Identifier` should match the fn param.
    pub fn get_ident(&self) -> &Identifier {
        &self.param
    }
    pub fn get_name(&self) -> &str {
        self.param.get_name()
    }
    /// Gets the value of the CallArgument.
    pub fn get_value(&self) -> &CallArgumentValue {
        &self.value
    }

    pub fn get_id<'a>(&'a self) -> Ref<'a, ScopedId> {
        self.param.get_id()
    }

    pub fn set_id(&self, id: ScopedId) {
        self.param.set_id(id);
    }
}
