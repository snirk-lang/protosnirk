//! Represents a completely parsed program.
//!
//! A program will contain the following:
//! - `Block`: list of statements to be executed
//! - `SymbolTable`: containing scopes and variables created in said scopes,
//! along with type information and unique IDs for said variables
//!

use parse::build::SymbolTable;
use parse::expression::Expression;
use parse::verify::ErrorCollector;
use run::{Instruction, Value};

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    block: Vec<Expression>,
    symbol_table: SymbolTable,
    constants: Vec<Value>,
    errors: ErrorCollector
}
impl Program {
    pub fn new(block: Vec<Expression>, symbol_table: SymbolTable, constants: Vec<Value>, errors: ErrorCollector) -> Program {
        Program {
            block: block,
            symbol_table: symbol_table,
            constants: constants,
            errors: errors
        }
    }
    pub fn is_errored(&self) -> bool {
        !self.errors.get_errors().is_empty()
    }
    pub fn get_symbol_table(&self) -> &SymbolTable {
        &self.symbol_table
    }
    pub fn get_constants(&self) -> &Vec<Value> {
        &self.constants
    }
    pub fn get_block(&self) -> &Vec<Expression> {
        &self.block
    }
    pub fn decompose(self) -> (Vec<Expression>, SymbolTable, Vec<Value>, ErrorCollector) {
        (self.block, self.symbol_table, self.constants, self.errors)
    }
}
