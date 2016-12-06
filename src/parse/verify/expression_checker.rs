//! Verification struct
//! Verifies things.

use std::collections::HashMap;

use lex::Token;
use parse::expression::*;
use parse::expression::Expression as BaseExpression;
use super::{ErrorCollector, VerifyError};

/// Trait for expression checkers: visitors on the expression tree.
pub trait ExpressionChecker {
    fn check_expression(&mut self, errors: &mut ErrorCollector, expr: &BaseExpression) {
        match *expr {
            BaseExpression::Assignment(ref assign) => {
                self.check_assignment(errors, assign)
            },
            BaseExpression::Literal(ref lit) => {
                self.check_literal(errors, lit)
            },
            BaseExpression::BinaryOp(ref bin) => {
                self.check_binary_op(errors, bin)
            },
            BaseExpression::Block(ref block) => {
                self.check_block(errors, block)
            },
            BaseExpression::Declaration(ref decl) => {
                self.check_declaration(errors, decl)
            },
            BaseExpression::Return(ref ret) => {
                self.check_return(errors, ret)
            },
            BaseExpression::UnaryOp(ref unary_op) => {
                self.check_unary_op(errors, unary_op)
            },
            BaseExpression::VariableRef(ref var_ref) => {
                self.check_var_ref(errors, var_ref)
            }
        }
    }
    #[inline]
    #[allow(unused_variables)]
    fn check_assignment(&mut self, errors: &mut ErrorCollector, assignment: &Assignment) {
        self.check_var_ref(errors, &assignment.lvalue);
        self.check_expression(errors, &assignment.rvalue);
    }
    #[inline]
    #[allow(unused_variables)]
    fn check_literal(&mut self, errors: &mut ErrorCollector, literal: &Literal) {
    }
    #[inline]
    #[allow(unused_variables)]
    fn check_binary_op(&mut self, errors: &mut ErrorCollector, bin_op: &BinaryOperation) {
        self.check_expression(errors, &bin_op.left);
        self.check_expression(errors, &bin_op.right);
    }
    #[inline]
    #[allow(unused_variables)]
    fn check_unary_op(&mut self, errors: &mut ErrorCollector, unary_op: &UnaryOperation) {
        self.check_expression(errors, &unary_op.expression);
    }
    #[inline]
    #[allow(unused_variables)]
    fn check_return(&mut self, errors: &mut ErrorCollector, ret: &Return) {
        if let Some(ref val) = ret.value {
            self.check_expression(errors, val);
        }
    }
    #[inline]
    #[allow(unused_variables)]
    fn check_var_ref(&mut self, errors: &mut ErrorCollector, var_ref: &Identifier) {
    }
    fn check_block(&mut self, errors: &mut ErrorCollector, block: &Vec<Expression>) {
        for expr in block {
            self.check_expression(errors, expr);
        }
    }
    #[inline]
    #[allow(unused_variables)]
    fn check_declaration(&mut self, errors: &mut ErrorCollector, decl: &Declaration) {
    }
}
