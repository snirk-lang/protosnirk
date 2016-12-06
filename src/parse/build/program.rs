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

pub struct Program {
    block: Vec<Expression>,
    symbol_table: SymbolTable,
    errors: ErrorCollector
}
impl Program {
    pub fn new(block: Vec<Expression>, symbol_table: SymbolTable, errors: ErrorCollector) -> Program {
        Program {
            block: block,
            symbol_table: symbol_table,
            errors: errors
        }
    }
    pub fn is_errored(&self) -> bool {
        !self.errors.get_errors().is_empty()
    }
}
