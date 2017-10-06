use parse::ScopedId;
use parse::ast::*;

use check::{CheckerError, ErrorCollector};
use identify::NameScopeBuilder;
use visit;
use visit::visitor::*;

/// Identifies variables in blocks.
#[derive(Debug)]
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
            errors,
            builder,
            item_id: ScopedId::default()
        }
    }
}

impl<'err, 'builder> DefaultUnitVisitor
    for ExpressionVarIdentifier<'err, 'builder> { }

impl<'err, 'builder> ItemVisitor for ExpressionVarIdentifier<'err, 'builder> {
    fn visit_block_fn_decl(&mut self, block_fn: &BlockFnDeclaration) {
        if !block_fn.get_ident().has_id() {
            trace!("Skipping block fn {} because it does not have an ID",
                block_fn.get_name());
            return
        }
        trace!("Checking block fn {} with id {:?}",
            block_fn.get_name(), block_fn.get_ident().get_id());
        self.current_id = block_fn.get_ident().get_id().clone();
        self.current_id.push(); // This puts it at param level
        self.current_id.push(); // This defines the entry block level.
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

impl<'err, 'builder> StatementVisitor
    for ExpressionVarIdentifier<'err, 'builder> {

    fn visit_if_block(&mut self, if_block: &IfBlock) {
        self.current_id.increment();
        if_block.set_id(self.current_id.clone());
        for condition in if_block.get_conditionals() {
            self.visit_expression(condition.get_conditional());
            self.visit_block(condition.get_block());
        }
        if let Some(_, else_block) = if_block.get_else() {
            self.visit_block(else_block);
        }
    }
}

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
                    // Check that the param has been identified.
                    let mut param_id = fn_call.get_id().pushed();
                    param_id.increment();

                    if !self.builder.contains_id(&param_id) {
                        let error_text =
                            format!("Function {} does not have parameters",
                                fn_call.get_text());
                        self.builder.add_error(
                            fn_call.get_token().clone(), vec![], error_text
                        );
                        // Check call expression anyway
                    }
                    self.check_expression(expr);
                },
                FnCallArgs::Arguments(args) => {
                    for arg in args {
                        let arg_name = arg.get_name();
                        let full_param_name = format!("{}:{}", fn_call.get_text(), arg_name);
                        if let Some(param_id) = self.builder.get(full_param_name) {
                            arg_name.set_id(param_id);
                        }
                        else {
                            let error_text = format!("Unknown parameter {} of {}",
                                arg_name.get_text(), fn_call.get_text());
                            self.builder.add_error(
                                arg_name.get_token().clone(), vec![], error_text
                            );
                            return // Stop checking expression
                        }
                        match arg.get_value() {
                            CallArgumentValue::LocalVar(var_ident) => {
                                // Set the id of the `value` to be the local var.
                                self.check_var_ref(var_ident);
                            },
                            CallArgumentValue::Expression(arg_expr) => {
                                self.check_expression(arg_expr);
                            }
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
