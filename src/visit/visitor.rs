//! Visitor traits for walking the AST.

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
            Item::BlockFnDeclaration(block_fn_decl) => {
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
            TypeExpression::Named(named_ty) => {
                self.visit_named_type_expr(named_ty);
            },
            TypeExpression::Function(block_fn_ty) => {
                self.visit_fn_ty_expr(block_fn_ty);
            },
        }
    }

    fn visit_named_type_expr(&mut self, named_ty: &NamedTypeExpression);
    fn visit_fn_type_expr(&mut self, fn_ty: &FnTypeExpression);
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
