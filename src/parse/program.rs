//! Represents a completely parsed program.
//!
//! A program will contain the following:
//! - `Block`: list of statements to be executed
//! - `SymbolTable`: containing scopes and variables created in said scopes,
//! along with type information and unique IDs for said variables
//!

use parse::ast::Unit;
use parse::verify::ErrorCollector;
use parse::verify::scope::SymbolTable;

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    unit: Unit,
    symbol_table: SymbolTable,
    errors: ErrorCollector
}
impl Program {
    pub fn new(unit: Unit, symbol_table: SymbolTable, errors: ErrorCollector) -> Program {
        Program {
            unit: unit,
            symbol_table: symbol_table,
            errors: errors
        }
    }
    pub fn is_errored(&self) -> bool {
        !self.errors.get_errors().is_empty()
    }
    pub fn get_symbol_table(&self) -> &SymbolTable {
        &self.symbol_table
    }
    pub fn get_unit(&self) -> &Unit {
        &self.unit
    }
    pub fn decompose(self) -> (Unit, SymbolTable, ErrorCollector) {
        (self.unit, self.symbol_table, self.errors)
    }
}
