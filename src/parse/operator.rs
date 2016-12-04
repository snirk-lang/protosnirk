//! Operator expressions are

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
    /// Custom operator
    Custom
}
