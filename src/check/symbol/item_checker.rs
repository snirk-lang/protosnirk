use parse::{ScopedId};
use parse::ast::*;

use check::{ASTVisitor, ErrorCollector};
use check::SymbolTable;

/// Does the first pass of symbol checking
/// to ensure items can be used before being declared.
#[derive(Debug, PartialEq, Clone)]
pub struct ItemChecker<'err, 'table> {
    table: &'table mut SymbolTable,
    errors: &'err mut ErrorCollector,
    current_id: SymbolId
}

impl<'err, 'table> ItemChecker<'err, 'table> {
    pub fn new(errors: &'err mut ErrorCollector,
               table: &'table mut SymbolTable)
               -> ItemChecker<'err, 'table> {
        ItemChecker {
            errors: errors,
            current_id: SymbolId
        }
    }
}

impl<'err, 'table> ASTVisitor for ItemChecker<'err, 'builder> {
}
