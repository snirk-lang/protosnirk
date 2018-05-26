//! AST visitor which assigns the ScopedIds of type expressions
//! within items.

use ast::*;
use check::{CheckerError, ErrorCollector};
use identify::TypeScopeBuilder;
use identify::types::TypeIdentifier;
use visit;
use visit::visitor::*;

/// Identifies the names of type expressions within functions, namely
/// cases where named types are explicitly declared.
pub struct ExprTypeIdentifier<'err, 'builder> {
    errors: &'err mut ErrorCollector,
    builder: &'builder mut TypeScopeBuilder,
    current_id: ScopedId
}

impl<'err, 'builder> ExprTypeIdentifier<'err, 'builder> {
    pub fn new(errors: &'err mut ErrorCollector,
               builder: &'builder mut TypeScopeBuilder)
               -> ExprTypeIdentifier<'err, 'builder> {
        ExprTypeIdentifier { errors, builder, current_id: ScopedId::default() }
    }
}

impl<'err, 'builder> DefaultUnitVisitor
    for ExprTypeIdentifier<'err, 'builder> { }

impl<'err, 'builder> ItemVisitor for ExprTypeIdentifier<'err, 'builder> {
    fn visit_block_fn_decl(&mut self, block_fn: &BlockFnDeclaration) {
        if block_fn.get_id().is_default() {
            trace!("Skipping unidentified block fn {}", block_fn.get_name());
            return
        }
        visit::walk_block(self, block_fn.get_block());
    }
}

impl<'err, 'builder> DefaultBlockVisitor
    for ExprTypeIdentifier<'err, 'builder> { }

// The goal of being an ExpressionVisitor is to find places where types are
// explicitly used within a function. For now this occurs only within a
// declaration, which is an expression:
// https://github.com/immington-industries/protosnirk/issues/30

impl<'err, 'builder> DefaultStmtVisitor
    for ExprTypeIdentifier<'err, 'builder> { }

impl<'err, 'builder> ExpressionVisitor
    for ExprTypeIdentifier<'err, 'builder> {

    fn visit_literal_expr(&mut self, _literal: &Literal) { }

    fn visit_var_ref(&mut self, _ident: &Identifier) { }

    fn visit_if_expr(&mut self, if_expr: &IfExpression) {
        visit::walk_if_expr(self, if_expr);
    }

    fn visit_unary_op(&mut self, unary_op: &UnaryOperation) {
        visit::walk_unary_op(self, unary_op);
    }

    fn visit_binary_op(&mut self, binary_op: &BinaryOperation) {
        visit::walk_bin_op(self, binary_op);
    }

    fn visit_fn_call(&mut self, fn_call: &FnCall) {
        // Technically shouldn't be allowed, but see #30 above.
        for arg in fn_call.get_args() {
            self.visit_expression(arg.get_expression());
        }
    }

    fn visit_assignment(&mut self, assign: &Assignment) { }

    fn visit_declaration(&mut self, declaration: &Declaration) {
        if let Some(ref decl_ty) = declaration.get_type_decl() {
            // visit the declaration
            TypeIdentifier::new(self.errors, self.builder)
                          .visit_type_expr(decl_ty);
        }
    }
}
