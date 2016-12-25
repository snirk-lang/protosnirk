//! Assembles constants from a parse tree into a list.

use parse::expression::Literal;
use parse::verify::{ExpressionChecker};
use run::Value;

/// Assembles constants from parse tree into a list
#[derive(Debug, Default)]
pub struct ConstantAssembler {
    constants: Vec<Value>
}
impl ConstantAssembler {
    #[inline]
    pub fn new() -> ConstantAssembler {
        ConstantAssembler::default()
    }
}
impl Into<Vec<Value>> for ConstantAssembler {
    fn into(self) -> Vec<Value> {
        self.constants
    }
}
impl ExpressionChecker for ConstantAssembler {
    fn check_literal(&mut self, literal: &Literal) {
        let literal_value = Value(literal.get_value());
        if let Some(_known_ix) = self.constants.iter()
            .position(|&known| known == literal_value) {
                return
        }
        println!("Adding constant {:?} to constant table", literal_value);
        self.constants.push(literal_value);
    }
}
