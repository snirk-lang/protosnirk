//! Default visitors.

use visit;
use visit::visitor::*;
use parse::ast::*;

/// A trait which allows a `ItemVisitor` to become a `UnitVisitor`
/// by visiting all items in a unit.
pub trait DefaultUnitVisitor : ItemVisitor { }
impl<T: DefaultUnitVisitor + ItemVisitor> UnitVisitor for T {
    fn visit_unit(&mut self, unit: &Unit) { visit::walk_unit(self, unit); }
}

/// A trait which allows a `BlockVisitor` to become an `ItemVisitor`
/// by just looking at block fns.
pub trait DefaultItemVisitor : BlockVisitor { }
impl<T: DefaultItemVisitor + BlockVisitor + StatementVisitor + ExpressionVisitor> ItemVisitor for T {
    fn visit_block_fn_decl(&mut self, block_fn: &BlockFnDeclaration) {
        visit::walk_block(self, block_fn.get_block());
    }
}

/// A trait which allows a `StatementVisitor` to become a
/// `BlockVisitor` by visiting all statments in a block.
pub trait DefaultBlockVisitor : StatementVisitor { }
impl<T: DefaultBlockVisitor> BlockVisitor for T {
    #[inline]
    fn visit_block(&mut self, block: &Block) {
        visit::walk_block(self, block);
    }
}

pub trait DefaultStmtVisitor : ExpressionVisitor { }
impl<V: DefaultStmtVisitor + ExpressionVisitor + BlockVisitor> StatementVisitor for V {
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
