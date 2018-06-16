//! Set the `ScopedId`s of expressions in the AST.

use lex::{Token, TextLocation};
use ast::{*, visit::*};
use identify::{NameScopeBuilder, OriginManager};
use check::{CheckerError, ErrorCollector};

use std::mem;

/// Identifies variables in blocks.
#[derive(Debug)]
pub struct ExpressionVarIdentifier<'err, 'builder> {
    errors: &'err mut ErrorCollector,
    builder: &'builder mut NameScopeBuilder,
    /// `ScopedId` to give to expressions
    current_id: ScopedId,
    /// `ScopedId` of the current function which we
    current_fn_id: ScopedId,
    /// Stack of lvalues which can be assigned to the current expression.
    /// For example, a block in a function which returns a value would have
    /// an lvalue of the function's ID.
    lvalues: OriginManager
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
            current_fn_id: ScopedId::default(),
            lvalues: OriginManager::new()
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
        trace!("Visiting fn definition {}", block_fn.name());
        if block_fn.id().is_default() {
            debug!("Skipping block fn {} because it has no ID",
                block_fn.name());
            return
        }
        self.current_id = block_fn.id().clone();

        // Re-create param level scope that ItemVarIdentifier used
        self.current_id.push();
        self.builder.new_scope();

        for &(ref param, ref _param_type) in block_fn.params() {
            let param_name = param.name();
            if param.id().is_default() {
                debug!("Skipping block fn {} because param {} does no ID",
                    block_fn.name(), param_name);
                return
            }
            // We re-define parameters here even though they've already been
            // identified, because the NameScopeBuilder discards its scopes
            // after visiting.

            self.builder.define_local(param_name.to_string(),
                                      param.id().clone());
        }

        if block_fn.has_explicit_return_type() {
            self.lvalues.add_source(block_fn.id().clone());
        }

        self.current_fn_id = block_fn.id().clone();

        // current_id = [<fn id>, 0]

        // Check the function block.
        // `visit_block` results in function blocks having a scope under the
        // parameters. Block starts with [<fn id>, 0, 0]
        self.visit_block(block_fn.block());

        self.lvalues.pop_source();

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
        if self.lvalues.has_source() {
            // Take the current lvalue stack to prevent the non-last
            // statements in the block from attempting to return a value.
            if block.stmts().len() == 0 {
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
                self.lvalues.pop_source();
                return
            }
            // Set source before visiting
            block.set_source(self.lvalues.pop_source()
                                         .expect("Checked expect")
                                         .clone());
            self.lvalues.begin_block();
            for i in 0 .. block.stmts().len() - 1 {
                self.visit_stmt(&block.stmts()[i]);
            }
            self.lvalues.end_block();
            // The last expression in the block is returning to the block.

            // Put the existing stack up (minus the last one which this block
            // is returning to)
            // Ensure the last statement should return to this block.
            self.lvalues.add_source(block.id().clone());
            // We want the last source
            self.visit_stmt(block.stmts().last().expect("Checked expect"));
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
        visit::walk_block(self, do_block.block());
    }

    fn visit_if_block(&mut self, if_block: &IfBlock) {
        trace!("Visiting if block");

        if_block.set_id(self.current_id.clone());
        self.current_id.push();

        let has_lvalue = self.lvalues.has_source();
        if has_lvalue {
            trace!("Found expression if block");
            if !if_block.has_else() {
                debug!("Expression if block did not have else");
                self.errors.add_error(CheckerError::new(
                    if_block.conditionals()[0].token().clone(),
                    vec![],
                    format!("If block needed to return a value but did not")
                ));
                return
            }

            let source = self.lvalues.pop_source().expect("Checked expect");
            if_block.set_source(source);
        }

        // Visit each conditional, sourcing it to the if and visiting the block.
        for cond in if_block.conditionals() {
            trace!("Checking conditional");
            self.visit_expression(cond.condition());

            if has_lvalue {
                trace!("Mapping conditional to if");
                self.lvalues.add_source(if_block.id().clone());
            }
            self.visit_block(cond.block());
            // We know that if the block visiting worked the block will pop the
            // source.
        }

        if let Some(&(ref _token, ref else_block)) = if_block.else_block() {
            trace!("Visting else");
            if has_lvalue {
                trace!("Adding source to else");
                self.lvalues.add_source(if_block.id().clone());
            }
            self.visit_block(else_block);
            // We know block visiting will pop the source.
        }

        self.current_id.pop();
        self.current_id.increment();
    }

    fn visit_return_stmt(&mut self, return_stmt: &Return) {
        trace!("Visiting return statement");
        if let Some(ret_expr) = return_stmt.value() {
            trace!("Adding fn id source to return expr");
            self.lvalues.add_source(self.current_fn_id.clone());
            self.visit_expression(ret_expr);
            if self.lvalues.has_top_source(&self.current_fn_id) {
                self.lvalues.pop_source();
            }
        }
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
        trace!("Visiting assignment to {}", assign.lvalue().name());
        // Give the required rvalue to the expression
        // Enables https://github.com/immington-industries/protosnirk/issues/27
        let lvalue_id = assign.lvalue().id().clone();
        if lvalue_id.is_default() {
            trace!("Found assignment to unknown var");
            let error_message = format!(
                "Unknown variable {} to assign to",
                assign.lvalue().name()
            );
            self.errors.add_error(CheckerError::new(
                assign.lvalue().token().clone(),
                vec![],
                error_message
            ));
        }
        self.lvalues.add_source(lvalue_id);
        self.visit_expression(assign.rvalue());
        if self.lvalues.has_top_source(&assign.lvalue().id()) {
            self.lvalues.pop_source();
        }
        self.visit_var_ref(assign.lvalue());
    }

    fn visit_var_ref(&mut self, ident: &Identifier) {
        trace!("Visiting reference to {}", ident.name());
        if let Some(var_id) = self.builder.get(ident.name()).cloned() {
            ident.set_id(var_id);
        }
        else {
            debug!("Emitting error: unknown ident {}", ident.name());
            // Unknown var
            let err_text = format!("Unknown reference to {}",
                ident.name());
            self.errors.add_error(CheckerError::new(
                ident.token().clone(), vec![], err_text
            ));
        }
    }

    fn visit_declaration(&mut self, declaration: &Declaration) {
        trace!("Visiting declaration of {}", declaration.name());
        let lvalue = declaration.ident();
        if let Some(_var_id) = self.builder.get(lvalue.name()) {
            debug!("Found an already defined variable");
            // Variable already declared. Shadowing is an error.
            // `builder.local` = Rust level shadowing, more or less
            // `builder.get` = no shadowing at all (even over globals).
            let err_text = format!("Variable {} is already declared",
                lvalue.name());
            self.errors.add_error(CheckerError::new(
                lvalue.token().clone(), vec![], err_text
            ));
            return
        }
        let decl_id = self.current_id.clone();
        self.builder.define_local(declaration.name().into(), decl_id.clone());
        trace!("Created id {:?} for var {}", decl_id, lvalue.name());
        lvalue.set_id(decl_id);
        self.current_id.increment();
    }

    fn visit_fn_call(&mut self, fn_call: &FnCall) {
        if let Some(fn_id) = self.builder.get(fn_call.text()).cloned() {
            // Set fn ident
            fn_call.ident().set_id(fn_id);
            // Check args
            for arg in fn_call.args() {
                self.visit_expression(arg.expression());
            }
        }
        else {
            // Args are not checked if name is not known
            let err_text = format!("Unknown function {}", fn_call.text());
            self.errors.add_error(CheckerError::new(
                fn_call.token().clone(), vec![], err_text
            ));
        }
    }
}
