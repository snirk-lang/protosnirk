//! Expression types

/// Variable declaration
#[derive(Debug, PartialEq, Clone)]
pub struct Declaration {
    name: String,
    mutable: bool,
    value: Option<Expression>
}

impl Declaration {
    pub fn new(name: String, value: Expression) -> Self {
        Declaration { name: name, value: value }
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_value(&self) -> &Expression {
        &self.value
    }
    pub fn is_mut(&self) -> bool {
        self.is_mut
    }
}

/// Literal value
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Literal(f64);

impl Literal {
    pub fn new(value: f64) -> Self {
        Literal(value)
    }
    pub fn get_value(&self) -> f64 {
        *self.0
    }
}

/// Reference to a Variable
/// the name of the variable...
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Identifier(String);

impl Identifier {
    pub fn new(name: Into<String>) -> Self {
        Reference(name.into())
    }
    pub fn get_name(&self) -> &str {
        &self.0
    }
}

/// Maths style binary operations (may be split up later)
#[derive(Debug, PartialEq, Clone)]
pub struct BinaryOperation {
    operator: BinaryOperator,
    left: Box<Expression>,
    right: Box<Expression>
}
impl BinaryOperation {
    pub fn new(operator: BinaryOperator, left: Expression, right: Expression) {
        BinaryOperation { operator: operator, left: left, right: right }
    }
    pub fn get_operator(&self) -> BinaryOperator {

    }
}

/// Binary operators
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Divide,
    Multiply,
    Modulus
}

impl Display for BinaryOperator {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        write!(formatter, "{}",
            match *self {
                BinaryOperator::Add => "+",
                BinaryOperator::Subtract => "-",
                BinaryOperator::Divide => "/",
                BinaryOperator::Multiply => "*",
                BinaryOperator::Modulus => "%"
            })
    }
}

/// Unary operators (negate)
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum UnaryOperator {
    Negate
}

impl Display for UnaryOperator {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        write!(formatter, "-")
    }
}

/// Unary operation
#[derive(Debug, PartialEq, Clone)]
pub struct UnaryOperation {
    operator: UnaryOperator,
    expression: Box<Expression>
}

impl UnaryOperation {

}

/// An identifier is assigned to a value
pub struct Assignment {
    lvalue: Identifier,
    rvalue: Box<Expression>
}
impl Assignment {

}

/// Expression
enum Expression {
    Literal(Literal),
    VariableRef(Identifier),
    BinaryOp(BinaryOperation),
    UnaryOp(UnaryOperation),
    Block(Vec<Statement>, Expression)
}

/// Statement within a program
enum Statement {
    Declaration(Declaration),
    Assignment(Assignment),
}

/// A compiled program
struct Program {
    statements: Vec<Statement>
}
/// Expression type
pub enum Expression {
    BinaryOp(BinOp),
    Parens(Box<Expression>),
    Negate(Box<Expression>),
    Literal(Literal)
}
