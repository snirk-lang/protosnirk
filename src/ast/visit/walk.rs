//! Methods for walking the AST.

use ast::*;
use ast::visit::visitor::*;

/// Visit each item in a `Unit`.
#[inline]
pub fn walk_unit<V>(visitor: &mut V, unit: &Unit)
                where V: ItemVisitor {
    for item in unit.items() {
        visitor.visit_item(item);
    }
}

/// Visit the `BlockFnDeclaration`'s block.
#[inline]
pub fn walk_fn_decl<V>(visitor: &mut V, fn_decl: &BlockFnDeclaration)
                    where V: BlockVisitor {
    visitor.visit_block(fn_decl.block());
}

/// Visit each statement in the block.
#[inline]
pub fn walk_block<V>(visitor: &mut V, block: &Block)
                 where V: StatementVisitor {
    for stmt in block.stmts() {
        visitor.visit_stmt(stmt);
    }
}

/// Visit the `condition`, `true_expr`, and `else` of the IfExpression.
#[inline]
pub fn walk_if_expr<V>(visitor: &mut V, if_expr: &IfExpression)
                where V: ExpressionVisitor {
    visitor.visit_expression(if_expr.condition());
    visitor.visit_expression(if_expr.true_expr());
    visitor.visit_expression(if_expr.else_expr());
}

#[inline]
pub fn walk_bin_op<V>(visitor: &mut V, bin_op: &BinaryOperation)
                  where V: ExpressionVisitor {
    visitor.visit_expression(bin_op.left());
    visitor.visit_expression(bin_op.right());
}

#[inline]
pub fn walk_unary_op<V>(visitor: &mut V, un_op: &UnaryOperation)
                    where V: ExpressionVisitor {
    visitor.visit_expression(un_op.inner());
}

#[inline]
pub fn walk_return<V>(visitor: &mut V, ret: &Return)
                     where V: ExpressionVisitor {
    if let Some(expr) = ret.value() {
        visitor.visit_expression(expr);
    }
}

#[inline]
pub fn walk_do_block<V>(visitor: &mut V, block: &DoBlock)
                        where V: BlockVisitor {
    visitor.visit_block(block.block());
}

#[inline]
pub fn walk_if_block<V>(visitor: &mut V, if_block: &IfBlock)
                        where V: BlockVisitor + ExpressionVisitor {
    for cond in if_block.conditionals() {
        visitor.visit_expression(cond.condition());
        visitor.visit_block(cond.block());
    }
    if let Some(ref block) = if_block.else_block() {
        visitor.visit_block(block);
    }
}
