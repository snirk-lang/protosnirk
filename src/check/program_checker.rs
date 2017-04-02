use parse::{ASTVisitor, Program, ErrorCollector};
use parse::ast::Unit;
use parse::verify::checker::*;
use parse::verify::scope::SymbolTable;

/// Runs verifications against code
///
/// The verifier is used to bring to gether the various expresison
/// checkers in this module and it produces a complete program,
/// with a symbol table and possible compilation errrors.
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct UnitChecker { }

impl UnitChecker {
    pub fn verify_unit(&mut self, unit: Unit) -> Result<Program, ErrorCollector> {
        let errors = ErrorCollector::new();
        let mut symbol_builder = SymbolTableChecker::new(errors);
        symbol_builder.check_unit(&unit);
        let (symbol_table, mut errors) = symbol_builder.decompose();
        if !errors.get_errors().is_empty() {
            return Err(errors)
        }
        UsageChecker { }.warn_for_unsused(&mut errors, &symbol_table);
        Ok(Program::new(unit, symbol_table, errors))
    }
}
