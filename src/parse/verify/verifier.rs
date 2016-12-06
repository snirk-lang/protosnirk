use parse::expression::Expression;

use parse::build::Program;
use parse::verify::{ExpressionChecker, ErrorCollector};
use parse::verify::checker::*;

/// Runs verifications against code
///
/// The verifier is used to bring to gether the various expresison
/// checkers in this module and it produces a complete program,
/// with a symbol table and possible compilation errrors.
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Verifier {
}
impl Verifier {
    pub fn verify_program(&mut self, block: Vec<Expression>) -> Program {
        let mut errors = ErrorCollector::new();
        let mut symbol_builder = SymbolTableChecker::new();
        symbol_builder.check_block(&mut errors, &block);
        let symbol_table = symbol_builder.into();
        UsageChecker { }.warn_for_unsused(&mut errors, &symbol_table);
        Program::new(block, symbol_table, errors)
    }
}
