//! Types used in the constable parser.

/// Identifier
type Ident = String;

/// Literal
pub enum Literal {
    Quote,
    Symbol(String),
    Number(f64)
}

pub enum Expression {
    Literal(Box<Literal>),
    Func { name: Ident, params: Vec<Expression> },
}

