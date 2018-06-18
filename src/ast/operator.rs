//! Operators are used to indicate whether the parser has encountered
//! a standard operator or a custom one.

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryOperator {
    /// Numeric addition
    Addition,
    /// Numeric subtraction
    Subtraction,
    /// Numeric multiplication
    Multiplication,
    /// Numeric division
    Division,
    /// Numeric modulus
    Modulus,
    /// Equality test
    Equality,
    /// Non-equality test
    NonEquality,
    // Numeric less than test
    LessThan,
    /// Numeric reater than test
    GreaterThan,
    /// Numeric less than equals test
    LessThanEquals,
    /// Numeric greater than equals test
    GreaterThanEquals,
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryOperator {
    /// Negation
    Negation,
    /// No-op
    Addition,
}
