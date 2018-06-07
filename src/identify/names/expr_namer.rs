//! Set the `ScopedId`s of expressions in the AST.

use lex::{Token, TextLocation};
use ast::*;
use check::{CheckerError, ErrorCollector};
use identify::NameScopeBuilder;
use visit;
use visit::visitor::*;

/// Identifies variables in blocks.
#[derive(Debug)]
pub struct ExpressionVarIdentifier<'err, 'builder> {
    errors: &'err mut ErrorCollector,
    builder: &'builder mut NameScopeBuilder,
    current_id: ScopedId,
    lvalues: Vec<ScopedId>
}
impl<'err, 'builder> ExpressionVarIdentifier<'err, 'builder> {
    pub fn new(errors: &'err mut ErrorCollector,
               builder: &'builder mut NameScopeBuilder,
               current_id: ScopedId)
               -> ExpressionVarIdentifier<'err, 'builder> {
        ExpressionVarIdentifier {
            errors,
            builder,
            current_id,
            lvalues: Vec::new()
        }
    }
}

impl<'err, 'builder> UnitVisitor for ExpressionVarIdentifier<'err, 'builder> {
    fn visit_unit(&mut self, unit: &Unit) {
        trace!("Visiting a unit");
        self.builder.new_scope();

        // Keep the current_id and builder scope in line with the functions.
        visit::walk_unit(self, unit);

        self.builder.pop();
        self.current_id.increment();
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
                                      param.get_id().clone());
        }

        if block_fn.has_explicit_return_type() {
            self.lvalues.push(block_fn.get_id().clone());
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
        // Give blocks scoped IDs in line with the current block
        block.set_id(self.current_id.clone());

        self.current_id.push();
        self.builder.new_scope();

        // Check if block is expression block.
        // Remove the parent ID from the stack so the first expression doesn't
        // try to return to it.
        if let Some(return_to) = self.lvalues.pop() {
            if block.get_stmts().len() == 0 {
                // Can't possibly return a value if there are no statements

                // We don't have a `Token` on `Block` to use for this error,
                // so we have to construct a terrible one.
                // https://github.com/immington-industries/protosnirk/issues/39

                let error_message = format!(
                    "Code includes an empty block expression"
                );
                self.errors.add_error(CheckerError::new(
                    Token::new_eof(TextLocation::default()),
                    vec![],
                    error_message
                ));
                return
            }

            // Set source before visiting
            block.set_source(return_to.clone());

            for i in 0 .. block.get_stmts().len() - 1 {
                self.visit_stmt(&block.get_stmts()[i]);
            }
            // The last expression in the block is returning to the block.
            self.lvalues.push(block.get_id().clone());
            self.visit_stmt(block.get_stmts().last().expect("Checked expect"));
        }
        else {
            visit::walk_block(self, block);
        }

        self.current_id.pop();
        self.builder.pop();

        self.current_id.increment();
    }
}

impl<'err, 'builder> StatementVisitor
    for ExpressionVarIdentifier<'err, 'builder> {

    fn visit_do_block(&mut self, do_block: &DoBlock) {
        trace!("Visiting do block");
        visit::walk_block(self, do_block.get_block());
    }

    fn visit_if_block(&mut self, if_block: &IfBlock) {
        trace!("Visiting if block");
        if if_block.get_id().is_default() {
            debug!("Skipping if block without ID");
            return
        }

        let current_lvalue: ScopedId;
        let has_lvalue = !self.lvalues.is_empty();
        if has_lvalue {
            trace!("Found expression if block");
            if !if_block.has_else() {
                debug!("Expression if block did not have else");
                self.errors.add_error(CheckerError::new(
                    if_block.get_conditionals()[0].get_token().clone(),
                    vec![],
                    format!("If block needed to return a value but did not")
                ));
            }

            current_lvalue = self.lvalues.pop().expect("Checked expect");
            if_block.set_source(current_lvalue.clone());
        }

    }

    fn visit_return_stmt(&mut self, return_stmt: &Return) {

    }
}

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
        // Give the required rvalue to the expression
        // Enables https://github.com/immington-industries/protosnirk/issues/27
        self.lvalues.push(assign.get_lvalue().get_id().clone());
        self.visit_expression(assign.get_rvalue());
        if self.lvalues.last() == Some(&assign.get_lvalue().get_id()) {
            self.lvalues.pop();
        }
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
