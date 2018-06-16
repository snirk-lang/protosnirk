//! Operators are used to indicate whether the parser has encountered
//! a standard operator or a custom one.

/// Standard set of operators + custom
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operator {
    /// Addition
    Addition,
    /// Subtraction **and** negation
    Subtraction,
    /// Multiplication
    Multiplication,
    /// Division
    Division,
    /// Modulus
    Modulus,
    /// Equality test
    Equality,
    /// Non-equality test
    NonEquality,
    // Less than test
    LessThan,
    /// Greater than test
    GreaterThan,
    /// Less than equals test
    LessThanEquals,
    /// Greater than equals test
    GreaterThanEquals,
    /// Custom operator
    Custom
}
