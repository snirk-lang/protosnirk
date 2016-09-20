//! Expression types

/// Expression type
pub enum Expression {
    BinaryOp(BinOp),
    Parens(Box<Expression>),
    Negate(Box<Expression>),
    Literal(Literal)
}
