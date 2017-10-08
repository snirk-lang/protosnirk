//! Visitor traits for walking the AST.

use std::collections::HashMap;

use lex::Token;
use parse::ast::*;
use parse::ast::Expression as BaseExpression;

pub use visit::default::*; // Reexport `DefaultVisitor`s here.

/// A visitor which can visit a unit of code.
pub trait UnitVisitor {
    fn visit_unit(&mut self, unit: &Unit);
}

/// A visitor which can visit items in code.
///
/// Eligible to become a `UnitVisitor` through `DefaultUnitVisitor`.
pub trait ItemVisitor {
    fn visit_item(&mut self, item: &Item) {
        match *item {
            Item::BlockFnDeclaration(ref block_fn_decl) => {
                self.visit_block_fn_decl(block_fn_decl);
            }
        }
    }

    fn visit_block_fn_decl(&mut self, block_fn_decl: &BlockFnDeclaration);
}

/// A visitor which can visit type expressions in code.
pub trait TypeVisitor {
    fn visit_type_expr(&mut self, expr: &TypeExpression) {
        match *expr {
            TypeExpression::Named(ref named_ty) => {
                self.visit_named_type_expr(named_ty);
            },
            TypeExpression::Function(ref block_fn_ty) => {
                self.visit_fn_type_expr(block_fn_ty);
            },
            TypeExpression::Primitive(ref prim) => {
                self.visit_primitive_type_expr(prim);
            }
        }
    }

    fn visit_named_type_expr(&mut self, named_ty: &NamedTypeExpression);
    fn visit_fn_type_expr(&mut self, fn_ty: &FnTypeExpression);
    fn visit_primitive_type_expr(&mut self, prim: &Primitive);
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
pub trait StatementVisitor : ExpressionVisitor {
    fn visit_stmt(&mut self, stmt: &Statement) {
        match *stmt {
            Statement::Expression(ref expr) => {
                self.visit_expression(expr);
            },
            Statement::Return(ref return_) => {
                self.visit_return_stmt(return_);
            },
            Statement::DoBlock(ref do_block) => {
                self.visit_do_block(do_block);
            },
            Statement::IfBlock(ref if_block) => {
                self.visit_if_block(if_block);
            }
        }
    }
    fn visit_return_stmt(&mut self, return_: &Return);
    fn visit_if_block(&mut self, if_block: &IfBlock);
    fn visit_do_block(&mut self, do_block: &DoBlock);
}

/// A visitor which can visit expressions of code.
pub trait ExpressionVisitor {
    fn visit_expression(&mut self, expr: &Expression) {
        match *expr {
            Expression::Literal(ref literal) => {
                self.visit_literal_expr(literal);
            },
            Expression::VariableRef(ref ident) => {
                self.visit_var_ref(ident);
            },
            Expression::BinaryOp(ref bin_op) => {
                self.visit_binary_op(bin_op);
            },
            Expression::UnaryOp(ref un_op) => {
                self.visit_unary_op(un_op);
            },
            Expression::IfExpression(ref if_expr) => {
                self.visit_if_expr(if_expr);
            },
            Expression::FnCall(ref fn_call) => {
                self.visit_fn_call(fn_call);
            },
            Expression::Assignment(ref assign) => {
                self.visit_assignment(assign);
            },
            Expression::Declaration(ref declare) => {
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
