//! Verification struct
//! Verifies things.

use std::collections::HashMap;

use lex::Token;
use parse::ast::*;
use parse::ast::Expression as BaseExpression;

/// A visitor which can visit a unit of code.
pub trait UnitVisitor {
    fn visit_unit(&self, unit: &Unit);
}

/// A visitor which can visit items in code.
///
/// Eligible to become a `UnitVisitor` through `DefaultUnitVisitor`.
pub trait ItemVisitor {
    fn visit_item(&mut self, item: &Item) {
        match *item {
            Item::InlineFnDeclaration(inline_fn_decl) => {
                self.visit_inline_fn_decl(inline_fn_decl);
            },
            Item::BlockFnDeclaration(block_fn_decl) => {
                self.visit_block_fn_decl(block_fn_decl);
            }
        }
    }

    fn visit_inline_fn_decl(&mut self, inline_fn_decl: &InlineFnDeclaration);
    fn visit_block_fn_decl(&mut self, block_fn_decl: &BlockFnDeclaration);
}

/// A visitor which can visit type expressions in code.
pub trait TypeVisitor {
    fn visit_type_expr(&mut self, expr: &TypeExpression) {
        match *expr {
            TypeExpression::Named(named_ty) => {
                self.visit_named_type_expr(named_ty);
            },
            TypeExpression::Function(block_fn_ty) => {
                self.visit_block_fn_ty_expr(block_fn_ty);
            },
            TypeExpression::InlineFn(inline_fn_ty) => {
                self.visit_inline_fn_ty(inline_fn_ty);
            }
        }
    }

    fn visit_named_type_expr(&mut self, named_ty: &NamedTypeExpression);
    fn visit_fn_type_expr(&mut self, fn_ty: &FnTypeExpression);
    fn visit_inline_fn_ty_expr(&mut self, inline_fn_ty: &InlineFnTypeExpression);
}

/// A visitor which can visit blocks of code.
///
/// Usually implemented via `DefaultBlockVisitor` for an
/// existing `StatementVisitor`.
pub trait BlockVisitor {
    fn visit_block(&mut self, block: &Block);
}

/// A visitor which can visit statements of code.
///
/// See also `DefaultBlockVisitor` which allows this
/// visitor to become a `BlockVisitor`.
pub trait StatementVisitor {
    fn visit_stmt(&mut self, stmt: &Statement) {
        match *stmt {
            Statement::Expression(expr) => {
                self.visit_expression(expr);
            },
            Statement::Return(return_) => {
                self.visit_return_stmt(return_);
            },
            Statement::DoBlock(do_block) => {
                self.visit_do_block(do_block);
            },
            Statement::IfBlock(if_block) => {
                self.visit_if_block(if_block);
            }
        }
    }
    fn visit_return_stmt(&mut self, return_: &Return);
    fn visit_if_block(&mut self, if_block: &IfBlock);
    fn visit_do_block(&mut self, do_block: &DoBlock);
    fn visit_expression(&mut self, expr: &Expression);
}

/// A visitor which can visit expressions of code.
pub trait ExpressionVisitor {
    fn visit_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::Literal(literal) => {
                self.visit_literal_expr(literal);
            },
            Expression::VariableRef(ident) => {
                self.visit_var_ref(ident);
            },
            Expression::BinaryOp(bin_op) => {
                self.visit_binary_op(bin_op);
            },
            Expression::UnaryOp(un_op) => {
                self.visit_unary_op(un_op);
            },
            Expression::IfExpression(if_expr) => {
                self.visit_if_expr(if_expr);
            },
            Expression::FnCall(fn_call) => {
                self.visit_fn_call(fn_call);
            },
            Expression::Assignment(assign) => {
                self.visit_assignment(assign);
            },
            Expression::Declaration(declare) => {
                self.visit_declaration(declare);
            }
        }
    }
    fn visit_literal_expr(&mut self, literal: &Literal);
    fn visit_var_ref(&mut self, ident: &Identifier);
    fn visit_if_expr(&mut self, if_expr: &IfExpression);
    fn visit_unary_op(&mut self, unary_op: &UnaryOperation);
    fn visit_binary_op(&mut self, bin_op: &BinaryOperation);
    fn visit_fn_call(&mut self, fn_call: &FnCall);
    fn visit_assignment(&mut self, assign: &Assignment);
    fn visit_declaration(&mut self, declare: &Declaration);
}

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
    fn visit_inline_fn_decl(&mut self, fn_decl: &mut InlineFnDeclaration) {
        visit::walk_inline_fn_decl(self, fn_decl);
    }
    fn visit_block_fn_decl(&mut self, fn_decl: &mut FnDeclaration) {
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

/// Algorithms for visiting children of AST nodes.
pub mod visit {
    use parse::ast::*;
    use super::*;

    #[inline]
    pub fn walk_unit<V>(visitor: &mut V, unit: &Unit)
                    where V: ItemVisitor {
        for item in unit.get_items() {
            visitor.visit_item(item);
        }
    }

    #[inline]
    pub fn walk_fn_decl<V>(visitor: &mut V, fn_decl: &FnDeclaration)
                        where V: BlockVisitor {
        visitor.visit_block(fn_decl.get_block());
    }

    #[inline]
    pub fn walk_inline_fn_decl<V>(visitor: &mut V,
                                  fn_decl: &InlineFnDeclaration)
                                  where V: BlockVisitor + ExpressionVisitor {
        visitor.visit_expression(fn_decl.get_expression());
    }

    #[inline]
    pub fn walk_block<V>(visitor: &mut V, block: &Block)
                     where V: StatementVisitor {
        for stmt in block.get_stmts() {
            visitor.visit_stmt(stmt);
        }
    }

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
    pub fn walk_inline_fn_type<V>(visitor: &mut V,
                                  fn_type: &InlineFnTypeExpression)
                                  where V: TypeVisitor {
        for (_param_name, param_type) in fn_type.get_params() {
            visitor.visit_type(param_type);
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
}
