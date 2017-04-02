//! Verification struct
//! Verifies things.

use std::collections::HashMap;

use lex::Token;
use parse::ast::*;
use parse::ast::Expression as BaseExpression;

/// Trait for expression checkers: visitors on the expression tree.
pub trait ASTVisitor {
    fn check_unit(&mut self, unit: &Unit) {
        for item in unit.get_items() {
            self.check_item(item);
        }
    }

    fn check_expression(&mut self, expr: &BaseExpression) {
        match *expr {
            BaseExpression::Assignment(ref assign) => {
                self.check_assignment(assign)
            },
            BaseExpression::Literal(ref lit) => {
                self.check_literal(lit)
            },
            BaseExpression::BinaryOp(ref bin) => {
                self.check_binary_op(bin)
            },
            BaseExpression::Declaration(ref decl) => {
                self.check_declaration(decl)
            },
            BaseExpression::UnaryOp(ref unary_op) => {
                self.check_unary_op(unary_op)
            }
            BaseExpression::VariableRef(ref var_ref) => {
                self.check_var_ref(var_ref)
            }
            BaseExpression::IfExpression(ref if_expr) => {
                self.check_if_expr(if_expr)
            }
            BaseExpression::FnCall(ref fn_call) => {
                self.check_fn_call(fn_call)
            }
        }
    }

    fn check_statement(&mut self, stmt: &Statement) {
        match *stmt {
            Statement::Expression(ref expr) => {
                self.check_expression(expr)
            },
            Statement::Return(ref return_) => {
                self.check_return(return_)
            },
            Statement::DoBlock(ref block) => {
                self.check_do_block(block)
            },
            Statement::IfBlock(ref block) => {
                self.check_if_block(block)
            }
        }
    }

    fn check_item(&mut self, item: &Item) {
        match *item {
            Item::FnDeclaration(ref decl) => {
                self.check_fn_declaration(decl)
            }
        }
    }

    fn check_block(&mut self, block: &Block) {
        for stmt in &block.statements {
            self.check_statement(stmt);
        }
    }

    #[inline]
    fn check_fn_declaration(&mut self, decl: &FnDeclaration) {
        self.check_block(decl.get_block())
    }

    #[inline]
    #[allow(unused_variables)]
    fn check_do_block(&mut self, block: &DoBlock) {
        self.check_block(&block.block);
    }

    #[inline]
    #[allow(unused_variables)]
    fn check_assignment(&mut self, assignment: &Assignment) {
        self.check_var_ref(&assignment.lvalue);
        self.check_expression(&assignment.rvalue);
    }
    #[inline]
    #[allow(unused_variables)]
    fn check_literal(&mut self, literal: &Literal) {
    }
    #[inline]
    #[allow(unused_variables)]
    fn check_binary_op(&mut self, bin_op: &BinaryOperation) {
        self.check_expression(&bin_op.left);
        self.check_expression(&bin_op.right);
    }
    #[inline]
    #[allow(unused_variables)]
    fn check_unary_op(&mut self, unary_op: &UnaryOperation) {
        self.check_expression(&unary_op.expression);
    }
    #[inline]
    #[allow(unused_variables)]
    fn check_return(&mut self, ret: &Return) {
        if let Some(ref val) = ret.value {
            self.check_expression(val);
        }
    }
    #[inline]
    #[allow(unused_variables)]
    fn check_var_ref(&mut self, var_ref: &Identifier) {
    }

    #[inline]
    #[allow(unused_variables)]
    fn check_declaration(&mut self, decl: &Declaration) {
        self.check_expression(&*decl.value);
    }

    fn check_if_block(&mut self, if_block: &IfBlock) {
        for conditional in if_block.get_conditionals() {
            self.check_expression(&conditional.get_condition());
            self.check_block(&conditional.get_block());
        }
        if let Some(else_info) = if_block.get_else() {
            self.check_block(&else_info.1);
        }
    }

    fn check_if_expr(&mut self, if_expr: &IfExpression) {
        self.check_expression(if_expr.get_condition());
        self.check_expression(if_expr.get_true_expr());
        self.check_expression(if_expr.get_else());
    }

    fn check_fn_call(&mut self, fn_call: &FnCall) {
        self.check_var_ref(fn_call.get_name());
    }
}
