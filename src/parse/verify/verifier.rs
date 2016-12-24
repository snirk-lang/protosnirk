use parse::expression::Expression;

use parse::build::{SymbolTable, Program};
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
    pub fn verify_program(&mut self, block: Vec<Expression>) -> Result<Program, ErrorCollector> {
        let errors = ErrorCollector::new();
        let symbol_table = SymbolTable::new();
        let mut symbol_builder = SymbolTableChecker::new(errors, symbol_table);
        symbol_builder.check_block(&block);
        let (symbol_table, mut errors) = symbol_builder.decompose();
        if !errors.get_errors().is_empty() {
            return Err(errors)
        }
        UsageChecker { }.warn_for_unsused(&mut errors, &symbol_table);
        let mut constant_assembler = ConstantAssembler::new();
        constant_assembler.check_block(&block);
        let constants = constant_assembler.into();
        Ok(Program::new(block, symbol_table, constants, errors))
    }
}
