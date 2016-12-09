//! Operators are used to indicate whether the parser has encountered
//! a standard operator or a custom one.

use run::OpCode;

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
    /// Custom operator
    Custom
}
impl Operator {
    pub fn get_opcode(&self) -> OpCode {
        use self::Operator::*;
        match *self {
            Addition => OpCode::Add,
            Subtraction => OpCode::Sub,
            Multiplication => OpCode::Mul,
            Division => OpCode::Div,
            Modulus => OpCode::Mod,
            Custom => unimplemented!()
        }
    }
}
