use ast::*;

use ast::ScopedId;

use check::{CheckerError, ErrorCollector};
use identify::NameScopeBuilder;
use visit;
use visit::visitor::*;

/// Identifies variables in blocks.
#[derive(Debug)]
pub struct ExpressionVarIdentifier<'err, 'builder> {
    errors: &'err mut ErrorCollector,
    builder: &'builder mut NameScopeBuilder,
    current_id: ScopedId
}
impl<'err, 'builder> ExpressionVarIdentifier<'err, 'builder> {
    pub fn new(errors: &'err mut ErrorCollector,
               builder: &'builder mut NameScopeBuilder,
               current_id: ScopedId)
               -> ExpressionVarIdentifier<'err, 'builder> {
        ExpressionVarIdentifier {
            errors,
            builder,
            current_id
        }
    }
}

impl<'err, 'builder> UnitVisitor for ExpressionVarIdentifier<'err, 'builder> {
    fn visit_unit(&mut self, unit: &Unit) {
        // Keep the current_id and builder scope in line with the functions.
        visit::walk_unit(self, unit);
    }
}

impl<'err, 'builder> ItemVisitor for ExpressionVarIdentifier<'err, 'builder> {
    fn visit_block_fn_decl(&mut self, block_fn: &BlockFnDeclaration) {
        trace!("Visiting fn definition {}", block_fn.get_name());
        if block_fn.get_id().is_default() {
            debug!("Skipping block fn {} because it has no ID",
                block_fn.get_name());
            return
        }
        self.current_id = block_fn.get_id().clone();

        // Re-create param level scope that ItemVarIdentifier used
        self.current_id.push();
        self.builder.new_scope();

        for &(ref param, ref _param_type) in block_fn.get_params() {
            let param_name = param.get_name();
            if param.get_id().is_default() {
                debug!("Skipping block fn {} because param {} does no ID",
                    block_fn.get_name(), param_name);
                return
            }
            // We re-define parameters here even though they've already been
            // identified, because the NameScopeBuilder discards its scopes
            // after visiting.

            self.builder.define_local(param_name.to_string(),
                                      block_fn.get_id().clone());
        }

        // current_id = [<fn id>, 0]

        // Check the function block.
        // `visit_block` results in function blocks having a scope under the
        // parameters. Block starts with [<fn id>, 0, 0]
        self.visit_block(block_fn.get_block());

        self.builder.pop();

        // pushing handled by `visit_block`, we reset current_id on next item.
    }
}

impl<'err, 'builder> BlockVisitor for ExpressionVarIdentifier<'err, 'builder> {
    fn visit_block(&mut self, block: &Block) {
        // Give blocks scoped IDs.
        self.current_id.push();
        self.builder.new_scope();

        block.set_id(self.current_id.clone());

        visit::walk_block(self, block);

        self.current_id.pop();
        self.builder.pop();

        self.current_id.increment();
    }
}

impl<'err, 'builder> DefaultStmtVisitor
    for ExpressionVarIdentifier<'err, 'builder> { }

impl<'err, 'builder> ExpressionVisitor
    for ExpressionVarIdentifier<'err, 'builder> {

    fn visit_literal_expr(&mut self, _literal: &Literal) { }

    fn visit_if_expr(&mut self, if_expr: &IfExpression) {
        visit::walk_if_expr(self, if_expr);
    }

    fn visit_unary_op(&mut self, un_op: &UnaryOperation) {
        visit::walk_unary_op(self, un_op);
    }

    fn visit_binary_op(&mut self, bin_op: &BinaryOperation) {
        visit::walk_bin_op(self, bin_op);
    }

    fn visit_assignment(&mut self, assign: &Assignment) {
        trace!("Visiting assignment to {}", assign.get_lvalue().get_name());
        self.visit_expression(assign.get_rvalue());
        self.visit_var_ref(assign.get_lvalue());
    }

    fn visit_var_ref(&mut self, ident: &Identifier) {
        trace!("Visiting reference to {}", ident.get_name());
        if let Some(var_id) = self.builder.get(ident.get_name()).cloned() {
            ident.set_id(var_id);
        }
        else {
            debug!("Emitting error: unknown ident {}", ident.get_name());
            // Unknown var
            let err_text = format!("Unknown reference to {}",
                ident.get_name());
            self.errors.add_error(CheckerError::new(
                ident.get_token().clone(), vec![], err_text
            ));
        }
    }

    fn visit_declaration(&mut self, declaration: &Declaration) {
        trace!("Visiting declaration of {}", declaration.get_name());
        let lvalue = declaration.get_ident();
        if let Some(_var_id) = self.builder.get(lvalue.get_name()) {
            // Variable already declared. Shadowing is an error.
            // `builder.get_local` = Rust level shadowing, more or less
            // `builder.get` = no shadowing at all (even over globals).
            let err_text = format!("Variable {} is already declared",
                lvalue.get_name());
            self.errors.add_error(CheckerError::new(
                lvalue.get_token().clone(), vec![], err_text
            ));
        }
        else {
            let decl_id = self.current_id.clone();
            trace!("Created id {:?} for var {}",
                decl_id, lvalue.get_name());
            lvalue.set_id(decl_id);
            self.current_id.increment();
        }
    }

    fn visit_fn_call(&mut self, fn_call: &FnCall) {
        if let Some(fn_id) = self.builder.get(fn_call.get_text()).cloned() {
            // Set fn ident
            fn_call.get_ident().set_id(fn_id);
            // Check args
            for arg in fn_call.get_args() {
                self.visit_expression(arg.get_expression());
            }
        }
        else {
            // Args are not checked if name is not known
            let err_text = format!("Unknown function {}", fn_call.get_text());
            self.errors.add_error(CheckerError::new(
                fn_call.get_token().clone(), vec![], err_text
            ));
        }
    }
}
