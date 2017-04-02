use parse::ast::Unit;

use check::{ASTVisitor, Program, ErrorCollector};

/// Runs verifications against code
///
/// The verifier is used to bring to gether the various expresison
/// checkers in this module and it produces a complete program,
/// with a symbol table and possible compilation errrors.
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct UnitChecker { }

impl UnitChecker {
    pub fn verify_unit(&mut self, unit: Unit) -> Program {
        let mut errors = ErrorCollector::new();
        {
            let mut scope_checker = ScopeChecker::new(&mut errors);
            scope_checker.check_unit(unit);
        }
        let mut symbol_checker = SymbolChecker::new(&mut errors);
        symbol_checker.check_unit(&unit);
        let symbol_table = symbol_checker.into_table();
        let mut type_checker = TypeChecker::new(&mut errors, &symbol_table);
        type_checker.check_unit(&unit);
        let type_table = type_checker.into_table();
        // lints
        UsageChecker { }.warn_for_unsused(&mut errors, &symbol_table);
        Program {
            unit: unit,
            symbols: symbol_table,
            types: TypeTable,
            errors: errors
        }
    }
}
