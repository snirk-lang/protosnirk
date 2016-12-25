//! Verification struct
//! Verifies things.

use std::collections::HashMap;

use lex::Token;
use parse::expression::*;
use parse::expression::Expression as BaseExpression;
use super::{ErrorCollector, VerifyError};

/// Trait for expression checkers: visitors on the expression tree.
pub trait ExpressionChecker {
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
            BaseExpression::Block(ref block) => {
                self.check_block(block)
            },
            BaseExpression::Declaration(ref decl) => {
                self.check_declaration(decl)
            },
            BaseExpression::Return(ref ret) => {
                self.check_return(ret)
            },
            BaseExpression::UnaryOp(ref unary_op) => {
                self.check_unary_op(unary_op)
            },
            BaseExpression::VariableRef(ref var_ref) => {
                self.check_var_ref(var_ref)
            }
        }
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
    fn check_block(&mut self, block: &Vec<Expression>) {
        for expr in block {
            self.check_expression(expr);
        }
    }
    #[inline]
    #[allow(unused_variables)]
    fn check_declaration(&mut self, decl: &Declaration) {
        self.check_expression(&*decl.value);
    }
}
