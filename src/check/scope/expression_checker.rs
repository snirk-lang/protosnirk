use parse::ScopedId;
use parse::ast::*;

use check::{ASTVisitor, ErrorCollector, CheckerError};
use check::scope::scope_builder::ScopeBuilder;

/// Establishes variable scopes.
///
/// The `ExpressionChecker` is the first verify pass.
/// Having been given only the parsed AST, its job
/// is to give meaningful unique IDs to each `Ident`
/// token - whether variable reference, function call
/// reference, type name, etc.
///
/// This is no small task - a C compiler can't do this
/// because of C's famous type/name ambiguity. Using this design
/// could also run into the same problem, even for a less
/// ambiguous language - I don't think this would work as
/// well in Rust.
///
/// We're just checking variables - types are
/// established in a later pass.
#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionChecker<'err, 'builder> {
    /// Build up the map of all name declarations.
    builder: &'builder mut ScopeBuilder,
    /// Mutably borrow an ErrorCollector to push to while we're running.
    errors: &'err mut ErrorCollector,
    /// ID of the function or top-level scope we're in.
    fn_id: ScopedId
}

impl<'err, 'builder> ExpressionChecker<'err, 'builder> {
    pub fn new(errors: &'err mut ErrorCollector,
               builder: &'builder mut ScopeBuilder) -> ExpressionChecker<'err, 'builder> {
        ExpressionChecker {
            builder: builder, errors: errors
        }
    }
}

impl<'err, 'builder> ASTVisitor for ExpressionChecker<'err, 'builder> {
    fn check_unit(&mut self, unit: &Unit) {
        // We increment the id first in `Unit` so that the first
        // index of `ScopedId` is never 0. We also _assume_ that
        // we can check multiple units in a row, even though the
        // rest of the system isn't designed to do that.
        self.current_id.increment();
        self.current_id.push();
        for item in unit.get_items() {
            self.check_item(item);
        }
        self.current_id.pop();
    }

    fn check_fn_declaration(&mut self, fn_decl: &FnDeclaration) {
        trace!("Checking fn {}", fn_decl.get_name());

        // If there's no ID on the function it's a dupe. Ignore for now.
        // In the future, adding more information to the ErrorCollector would
        // allow us to check variables inside the function, and report the error
        // futher out. If we did check this function def, at best we'd have
        // a bunch of GCC-style duplicated errors, at worst we could break
        // `ScopedId` invariants in the AST (i.e. IDs being unique). We take the
        // safe route for errors, i.e. not having them :( for now.
        // Duplicate function error is already handled in the ItemChecker.
        if fn_decl.get_ident().get_id() == ScopedId::default() {
            trace!("fn {} has no ScopedId, skipping!", fn_decl.get_name());
            return
        }

        // Save the top level ID when checking a new fn declaration.
        self.fn_id = fn_decl.get_ident().get_id().clone();

        // Check in the functions params (not in the global scope)
        self.fn_id.push();
        self.builder.new_scope();

        for param in fn_decl.get_args() {
            trace!("Checking {}'s param {}", fn_decl.get_name(), param.get_name());

            // Check for duplicate param names
            if let Some(declared_index) = self.builder.get(param.get_name()).cloned() {
                trace!("Encountered duplicate param {}", param.get_name());
                // TODO get previous declaration index
                // We can maintain this information in the builder and possibly pass it
                // onto the symbol checker to reduce the amount of repetition there.
                let err_text = format!("Argument {} is already declared", param.get_name());
                self.errors.add_error(CheckerError::new(
                    param.get_token().clone(), vec![], err_text
                ));
                // Skip adding a ScopedId to this param!
                continue
            }
            // Set the ScopedId of the param.
            let param_id = self.fn_id.clone();
            self.fn_id.increment();
            trace!("Created ID {:?} for fn {} arg {}",
                param_id, fn_decl.get_name(), param.get_name());
            self.builder.define_local(param.get_name().to_string(), param_id.clone());
            param.set_id(param_id);
        }

        // Immediately start checking function statements, instead of having the args
        // be in a separate scope. This isn't really needed, but having fewer scopes
        // could improve performance of smallvec. If we ever _use_ the scopes in
        // `ScopedId` for anything, it might be nicer to have them separate.
        for stmt in fn_decl.get_block().get_stmts() {
            self.check_stmt(&stmt);
        }
        // Don't need to clean up self.fn_id in this checker.
    }

    fn check_block(&mut self, block: &Block) {
        trace!("Checking a block");
        self.fn_id.push();
        self.builder.new_scope();
        for stmt in block.get_stmts() {
            self.check_statement(stmt);
        }
        self.builder.pop();
        self.fn_id.pop();
        self.fn_id.increment();
    }

    fn check_declaration(&mut self, decl: &Declaration) {
        // Check rvalue first
        self.check_expression(decl.get_value());
        trace!("Checking declaration of {}", decl.get_name());

        if let Some(declared_id) = self.builder.get(decl.get_name()).cloned() {
            // TODO reference the previous declaration
            let err_text = format!("Variable {} is already declared", decl.get_name());
            self.errors.add_error(CheckerError::new(
                decl.get_ident().get_token().clone(), vec![], err_text
            ));
            return
        }

        let decl_id = self.fn_id.clone();
        self.fn_id.increment();
        trace!("Created ID {:?} for variable {}", decl_id, decl.get_name());
    }

    fn check_var_ref(&mut self, var_ref: &Identifier) {

    }
}
