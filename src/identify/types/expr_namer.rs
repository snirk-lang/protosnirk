//! AST visitor which assigns the ScopedIds of type expressions
//! within items.

use ast::{*, visit::*};
use check::ErrorCollector;
use identify::TypeScopeBuilder;
use identify::types::TypeIdentifier;

/// Identifies the names of type expressions within functions, namely
/// cases where named types are explicitly declared.
pub struct ExprTypeIdentifier<'err, 'builder> {
    errors: &'err mut ErrorCollector,
    builder: &'builder mut TypeScopeBuilder,
}

impl<'err, 'builder> ExprTypeIdentifier<'err, 'builder> {
    pub fn new(errors: &'err mut ErrorCollector,
               builder: &'builder mut TypeScopeBuilder)
               -> ExprTypeIdentifier<'err, 'builder> {
        ExprTypeIdentifier { errors, builder }
    }
}

impl<'err, 'builder> UnitVisitor for ExprTypeIdentifier<'err, 'builder> {
    fn visit_unit(&mut self, unit: &Unit) {
        trace!("Visiting a unit");
        visit::walk_unit(self, unit);
    }
}

impl<'err, 'builder> ItemVisitor for ExprTypeIdentifier<'err, 'builder> {
    fn visit_block_fn_decl(&mut self, block_fn: &BlockFnDeclaration) {
        trace!("Visiting declaration of fn {}", block_fn.name());
        if block_fn.id().is_default() {
            trace!("Skipping unidentified block fn {}", block_fn.name());
            return
        }
        self.visit_block(block_fn.block());
    }

    fn visit_typedef(&mut self, _typedef: &Typedef) {
        // skip, only visiting expressions
    }
}

impl<'err, 'builder> BlockVisitor for ExprTypeIdentifier<'err, 'builder> {
    fn visit_block(&mut self, block: &Block) {
        trace!("Visiting a block");
        visit::walk_block(self, block);
    }
}

// The goal of being an ExpressionVisitor is to find places where types are
// explicitly used within a function. For now this occurs only within a
// declaration, which is an expression:
// https://github.com/immington-industries/protosnirk/issues/30

impl<'err, 'builder> StatementVisitor for ExprTypeIdentifier<'err, 'builder> {
    fn visit_return_stmt(&mut self, return_: &Return) {
        trace!("Visiting a return statement");
        visit::walk_return(self, return_);
    }

    fn visit_if_block(&mut self, if_block: &IfBlock) {
        trace!("Visiting an if block");
        visit::walk_if_block(self, if_block);
    }

    fn visit_do_block(&mut self, do_block: &DoBlock) {
        trace!("Visiting a do block");
        visit::walk_do_block(self, do_block);
    }
}

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
        for arg in fn_call.args() {
            self.visit_expression(arg.expression());
        }
    }

    fn visit_assignment(&mut self, assign: &Assignment) {
        self.visit_expression(assign.rvalue());
    }

    fn visit_declaration(&mut self, declaration: &Declaration) {
        if let Some(ref decl_ty) = declaration.type_decl() {
            // visit the declaration
            TypeIdentifier::new(self.errors, self.builder)
                          .visit_type_expr(decl_ty);
        }
    }
}
