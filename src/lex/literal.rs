//! Literal values

// ident = ~reserved ("_"|letter) ("_"|letter|digit)*
// number
// = digit+
// | digit+ "." digit+
// | "0" ("X"|"x") (hexnum)+

/// Numeric literal value
pub struct NumberLiteral(f64);

pub enum Literal {
    Number(NumberLiteral);
}
