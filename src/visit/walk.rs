//! Methods for walking the AST.

use parse::ast::*;
use super::*;

/// Visit each item in a `Unit`.
#[inline]
pub fn walk_unit<V>(visitor: &mut V, unit: &Unit)
                where V: ItemVisitor {
    for item in unit.get_items() {
        visitor.visit_item(item);
    }
}

/// Visit the `BlockFnDeclaration`'s block.
#[inline]
pub fn walk_fn_decl<V>(visitor: &mut V, fn_decl: &BlockFnDeclaration)
                    where V: BlockVisitor {
    visitor.visit_block(fn_decl.get_block());
}

/// Visit each statement in the block.
#[inline]
pub fn walk_block<V>(visitor: &mut V, block: &Block)
                 where V: StatementVisitor {
    for stmt in block.get_stmts() {
        visitor.visit_stmt(stmt);
    }
}

/// Visit the `condition`, `true_expr`, and `else` of the IfExpression.
#[inline]
pub fn walk_if_expr<V>(visitor: &mut V, if_expr: &IfExpression)
                where V: ExpressionVisitor {
    visitor.visit_expression(if_expr.get_condition());
    visitor.visit_expression(if_expr.get_true_expr());
    visitor.visit_expression(if_expr.get_else());
}

#[inline]
pub fn walk_bin_op<V>(visitor: &mut V, bin_op: &BinaryOperation)
                  where V: ExpressionVisitor {
    visitor.visit_expression(bin_op.get_left());
    visitor.visit_expression(bin_op.get_right());
}

#[inline]
pub fn walk_unary_op<V>(visitor: &mut V, un_op: &UnaryOperation)
                    where V: ExpressionVisitor {
    visitor.visit_expression(un_op.get_expr());
}

#[inline]
pub fn walk_return<V>(visitor: &mut V, ret: &Return)
                     where V: ExpressionVisitor {
    if let Some(expr) = ret.get_expr() {
        visitor.visit_expression(expr);
    }
}

#[inline]
pub fn walk_do_block<V>(visitor: &mut V, block: &DoBlock)
                        where V: BlockVisitor {
    visitor.visit_block(block);
}

#[inline]
pub fn walk_if_block<V>(visitor: &mut V, if_block: &IfBlock)
                        where V: BlockVisitor + ExpressionVisitor {
    for cond in if_block.get_conditionals() {
        visitor.visit_expression(cond.get_condition());
        visitor.visit_block(cond.get_block());
    }
    if let Some((_, block)) = if_block.get_else() {
        visitor.visit_block(block);
    }
}

#[inline]
pub fn walk_fn_type<V>(visitor: &mut V, fn_type: &FnTypeExpression)
                       where V: TypeVisitor {
    for (_param_name, param_type) in fn_type.get_params() {
        visitor.visit_type(param_type);
    }
    visitor.visit_type(fn_type.get_return_type());
}
