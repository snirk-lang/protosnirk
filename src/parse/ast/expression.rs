/// Expression values
///
/// Expression values are used in the `Expression` and `Statement` contexts.
/// They are usually emitted as asm instructions operating on variables.

/// Expression types
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Literal(Literal),
    VariableRef(Identifier),
    BinaryOp(BinaryOperation),
    UnaryOp(UnaryOperation),
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
