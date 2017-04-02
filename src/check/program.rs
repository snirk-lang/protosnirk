//! A fully checked protosnirk program, ready to be compiled.
//!
//! Contains the `Unit` AST node as well as a collection of
//! mappings of semantic information - errors, symbols, and
//! types.

use parse::ast::Unit;
use check::{SymbolTable, TypeTable};

/// Represents a fully parsed and checked program, ready
/// to be compiled.
#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    unit: Unit,
    symbols: SymbolTable,
    types: TypeTable,
    errors: ErrorCollector
}

impl Program {
    /// Create a `Program` from a collection of fields
    pub fn new(unit: Unit,
               symbol_table: SymbolTable,
               type_table: TypeTable,
               errors: ErrorCollector) -> Program {
        Program {
            unit: unit, symbols: symbols, type_tableL type_table, errors: errors
        }
    }

    /// Get the AST `Unit` for this program
    pub fn get_unit(&self) -> &Unit {
        &self.unit
    }

    /// Get the symbol table of the program
    pub fn get_symbols(&self) -> &SymbolTable {
        &self.symbols
    }

    /// Get the type table of the program
    pub fn get_types(&self) -> &TypeTable {
        &self.types
    }

    /// Get the errors found in the program
    pub fn get_errors(&self) -> &ErrorCollector {
        &self.errors
    }

    /// Checks whether this program compiled successfully
    pub fn has_errors(&self) -> bool {
        self.errors.get_errors().is_empty()
    }
}
