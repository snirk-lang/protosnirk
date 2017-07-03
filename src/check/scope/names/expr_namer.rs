use parse::ScopedId;
use parse::ast::*;

use check::{CheckerError, ErrorCollector};
use check::scope::NameScopeBuilder;
use check::visitor::*;

/// Identifies variables in blocks.
pub struct ExpressionVarIdentifier<'err, 'builder> {
    errors: &'err mut ErrorCollector,
    builder: &'builder mut NameScopeBuilder,
    item_id: ScopedId
}
impl<'err, 'builder> ExpressionVarIdentifier<'err, 'builder> {
    pub fn new(errors: &'err mut ErrorCollector,
               builder: &'builder mut NameScopeBuilder)
               -> ExpressionVarIdentifier<'err, 'builder> {
        ExpressionVarIdentifier {
            errors, builder
        }
    }
}

impl<'err, 'builder> DefaultUnitVisitor
    for ExpressionVarIdentifier<'err, 'builder> { }

impl<'err, 'builder> ItemVisitor for ExpressionVarIdentifier<'err, 'builder> {
    fn visit_inline_fn_decl(&mut self, inline_fn: &InlineFnDeclaration) {
        if !inline_fn.get_ident().has_id() {
            trace!("Skipping inline fn {} because it does not have an ID",
                inline_fn.get_name());
            return
        }
        trace!("Checking inline fn {} with id {:?}",
            inline_fn.get_name(), inline_fn.get_ident().get_id());
        self.current_id = inline_fn.get_ident().get_id().clone();
        self.current_id.push();
        for (param, _param_type) in inline_fn.get_params() {
            if let Some(declared_ix) = self.builder.get(param.get_name()).cloned() {
                trace!("Encountered duplicate param {} on {}",
                    param.get_name(), inline_fn.get_name());
                let err_text = format!("Argument {} is already declared",
                    param.get_name());
                self.errors.add_error(CheckerError::new(
                    param.get_token().clone(), vec![], err_text
                ));
                // Skip adding an ID to this param
                // -> giving IDs to error-defines and keeping track of which
                // ids are in error is probably a better solution!
                continue
            }

            self.current_id.increment();
            let param_id = self.current_id.clone();
            trace!("Identifying {} param {} with {:?}",
                inline_fn.get_name(), param.get_name(), param_id);
            param.set_id(param_id);
        }
        // Check the function block
        self.visit_expression(inline_fn.get_expression());
    }

    fn visit_block_fn_decl(&mut self, block_fn: &BlockFnDeclaration) {
        if !block_fn.get_ident().has_id() {
            trace!("Skipping block fn {} because it does not have an ID",
                block_fn.get_name());
            return
        }
        trace!("Checking block fn {} with id {:?}",
            block_fn.get_name(), block_fn.get_ident().get_id());
        self.current_id = block_fn.get_ident().get_id().clone();
        self.current_id.push();
        for (param, _param_type) in block_fn.get_params() {
            if let Some(declared_ix) = self.builder.get(param.get_name()).cloned() {
                trace!("Encountered duplicate param {} on {}",
                    param.get_name(), block_fn.get_name());
                let err_text = format!("Argument {} is already declared",
                    param.get_name());
                self.errors.add_error(CheckerError::new(
                    param.get_token().clone(), vec![], err_text
                ));
                // Skip adding an ID to this param
                // -> giving IDs to error-defines and keeping track of which
                // ids are in error is probably a better solution!
                continue
            }

            self.current_id.increment();
            let param_id = self.current_id.clone();
            trace!("Identifying {} param {} with {:?}",
                block_fn.get_name(), param.get_name(), param_id);
            param.set_id(param_id);
        }
        // Check the function block
        self.visit_block(block_fn.get_block());
    }
}

impl<'err, 'builder> BlockVisitor for ExpressionVarIdentifier<'err, 'builder> {
    fn visit_block(&mut self, block: &Block) {
        // Give blocks scoped IDs.
        // For top-level blocks in fns this becomes
        // the ID after their params (which are already pushed)
        self.current_id.increment();
        block.set_id(self.current_id.clone());
        self.current_id.push();
        self.builder.new_scope();
        visit::walk_block(self, block);
        self.current_id.pop();
        self.builder.pop();
    }
}

impl<'err, 'builder> DefaultStmtVisitor
    for ExpressionVarIdentifier<'err, 'builder> { }

impl<'err, 'builder> ExpressionVisitor
    for ExpressionVarIdentifier<'err, 'builder> {

    fn visit_literal_expr(&mut self, literal: &Literal) { }

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
        self.visit_expression(assign.get_rvalue());
        let lvalue = assign.get_lvalue();
        if let Some(lvalue_id) = self.builder.get(lvalue.get_name()).cloned() {
            lvalue.set_id(lvalue_id);
        }
        else {
            // lvalue does not exist
            let err_text = format!("Unknown variable {}",
                lvalue.get_name());
            self.errors.add_error(CheckerError::new(
                lvalue.get_ident().get_token().clone(), vec![], err_text
            ));
        }
    }

    fn visit_var_ref(&mut self, ident: &Identifier) {
        if let Some(var_id) = self.builder.get(ident.get_name()).cloned() {
            ident.set_id(var_id);
        }
        else {
            // Unknown var
            let err_text = format!("Unknown reference to {}",
                ident.get_name());
            self.errors.add_error(CheckerError::new(
                ident.get_token().clone(), vec![], err_text
            ));
        }
    }

    fn visit_declaration(&mut self, declaration: &Declaration) {
        let lvalue = declaration.get_lvalue();
        if let Some(_var_id) = self.builder.get(lvalue.get_name()) {
            // Variable already declared.
            // `builder.get_local` = Rust level shadowing, more or less
            // `builder.get` = no shadowing at all (even over globals).
            let err_text = format!("Variable {} is already declared",
                lvalue.get_name());
            self.errors.add_error(CheckerError::new(
                lvalue.get_token().clone(), vec![], err_text
            ));
        }
        else {
            self.current_id.increment();
            let decl_id = self.current_id.clone();
            trace!("Created id {:?} for var {}",
                decl_id, lvalue.get_name());
            lvalue.set_id(decl_id);
        }
    }

    fn visit_fn_call(&mut self, fn_call: &FnCall) {
        if let Some(fn_id) = self.builder.get(fn_call.get_text()).cloned() {
            // Set fn ident
            fn_call.get_ident().set_id(fn_id);
            // Check args
            match fn_call.get_args() {
                FnCallArgs::SingleExpr(expr) => {
                    self.check_expression(expr);
                },
                FnCallArgs::Arguments(args) => {
                    for arg in args {
                        // Check arg expressions
                        if let Some(expr) = arg.get_expr() {
                            self.check_expression(expr);
                        }
                        // Check implicit names as local idents.
                        else {
                            self.check_var_ref(arg.get_ident());
                        }
                    }
                }
            }
        }
        else {
            // Args are not checked if name is not known
            let err_text = format!("Unknown function {}", fn_call.get_name());
            self.errors.add_error(CheckerError::new(
                fn_call.get_token().clone(), vec![], err_text
            ));
        }
    }
}
