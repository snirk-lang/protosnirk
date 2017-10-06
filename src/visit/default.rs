//! Default visitors.

use super::*;
use parse::ast::*;

/// A trait which allows a `ItemVisitor` to become a `UnitVisitor`
/// by visiting all items in a unit.
pub trait DefaultUnitVisitor : ItemVisitor {
    #[inline]
    fn visit_unit(&self, unit: &Unit) {
        visit::walk_unit(self, unit);
    }
}
impl UnitVisitor for DefaultUnitVisitor { }

pub trait DefaultItemVisitor : BlockVisitor { }
impl ItemVisitor for DefaultItemVisitor {
    fn visit_block_fn_decl(&mut self, fn_decl: &BlockFnDeclaration) {
        visit::walk_fn_decl(self, fn_decl);
    }
}

/// A trait which allows a `StatementVisitor` to become a
/// `BlockVisitor` by visiting all statments in a block.
pub trait DefaultBlockVisitor : StatementVisitor {
    #[inline]
    fn visit_block(&mut self, block: &Block) {
        visit::walk_block(self, block);
    }
}
impl BlockVisitor for DefaultBlockVisitor { }

pub trait DefaultStmtVisitor : ExpressionVisitor { }
impl<V: DefaultStmtVisitor + ExpressionVisitor> StatementVisitor for V {
    #[inline]
    fn visit_expression(&mut self, expr: &Expression) {
        ExpressionVisitor::visit_expression(self, expr);
    }
    #[inline]
    fn visit_do_block(&mut self, do_block: &DoBlock) {
        visit::walk_do_block(self, do_block);
    }
    #[inline]
    fn visit_if_block(&mut self, if_block: &IfBlock) {
        visit::walk_if_block(self, if_block);
    }
    #[inline]
    fn visit_return_stmt(&mut self, return_: &Return) {
        visit::walk_return(self, return_);
    }
}
