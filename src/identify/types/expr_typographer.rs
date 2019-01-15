//! Builds the `TypeGraph` using code within functions

use ast::{*, visit::*};
use identify::TypeScopeBuilder;
use identify::types::{TypeGraph, InferenceSource};
use check::{CheckerError, ErrorCollector};

use petgraph::graph::NodeIndex;

use std::ops::Deref;

/// Creates type equations for functions.
#[derive(Debug)]
pub struct ExprTypographer<'err, 'builder, 'graph> {
    /// Constructs mapping from ScopedId -> ConcreteType
    builder: &'builder TypeScopeBuilder,
    /// Collects errors
    errors: &'err mut ErrorCollector,
    /// Performs type inference on expressions
    graph: &'graph mut TypeGraph,
    /// Type index obtained by visiting an expression
    current_type: NodeIndex,
    /// Return type of the current function
    fn_ret_type: NodeIndex,
}

impl<'err, 'builder, 'graph> ExprTypographer<'err, 'builder, 'graph> {
    pub fn new(builder: &'builder TypeScopeBuilder,
               errors: &'err mut ErrorCollector,
               graph: &'graph mut TypeGraph)
               -> ExprTypographer<'err, 'builder, 'graph> {
        ExprTypographer {
            builder,
            errors,
            graph,
            current_type: NodeIndex::default(),
            fn_ret_type: NodeIndex::default(),
        }
    }

    /// Get the "injected" primitive type
    fn primitive_type_ix(&self, name: &str) -> NodeIndex {
        self.builder.named_type_id(name)
            .and_then(|unary_id| self.graph.get_type(unary_id))
            .expect(&format!("Did not have primitive {}", name))
    }
}

impl<'err, 'builder, 'graph> UnitVisitor
    for ExprTypographer<'err, 'builder, 'graph> {

    fn visit_unit(&mut self, unit: &Unit) {
        trace!("Visiting a unit");
        visit::walk_unit(self, unit);
    }
}

impl<'err, 'builder, 'graph> ItemVisitor
    for ExprTypographer<'err, 'builder, 'graph> {

    fn visit_block_fn_decl(&mut self, block_fn: &BlockFnDeclaration) {
        trace!("Visiting fn {}", block_fn.name());
        let fn_id = block_fn.id();
        if fn_id.is_default() {
            debug!("Skipping fn {}, has no ID", block_fn.name());
            return
        }
        if block_fn.return_type().id().is_default() {
            debug!("Skipping fn {}, unknown return type", block_fn.name());
            return
        }

        // We've already set up information about `fn_ty` and the params.
        // Now, we're gonna require that the block returns the same thing
        // as the function, and check the block.
        // We're also going to save the block type id so it can be used for
        // inferring returns.

        let need_ret_value =
            block_fn.return_type().id().deref() !=
            self.builder.named_type_id("()").expect("Primitive");

        trace!("fn {} needs ret value? {}",
            block_fn.name(), need_ret_value);

        let fn_ret_type = self.graph.get_type(
            &block_fn.return_type().id())
            .expect("Could not determine fn return type");

        self.fn_ret_type = fn_ret_type;

        let fn_ix = self.graph.add_variable(fn_id.clone());
        let fn_ty_ix = self.graph.add_type(fn_id.clone());

        // var_f: ty_f
        self.graph.add_inference(fn_ix, fn_ty_ix,
            InferenceSource::FnSignature(block_fn.ident().clone()));

        // Add in connections to the parameter variables.
        for &(ref param_ident, ref param_expr) in block_fn.params() {
            trace!("Checking {} param {}",
                block_fn.name(), param_ident.name());
            let param_id = param_ident.id();
            let param_ty_id = param_expr.id();

            // x: <param type>
            let param_ix = self.graph.add_variable(param_id.clone());
            let param_ty_ix = self.graph.add_type(param_ty_id.clone());

            // var_param: ty_param
            self.graph.add_inference(param_ix, param_ty_ix,
                InferenceSource::FnParameter(param_ident.clone()));
        }

        self.visit_block(block_fn.block());

        // If the function needs a return expression, the block _should have_
        // set its type to the current type and added an inference from its
        // type to the last expression, setting it up for an implicit return.
        if need_ret_value {
            trace!("Inferring return value of {}", block_fn.name());
            // expr_inret: ty_fn_ret
            self.graph.add_inference(self.current_type,
                                     fn_ret_type,
                InferenceSource::FnReturnType(block_fn.ident().clone()));
        }
    }

    fn visit_typedef(&mut self, _typedef: &Typedef) {
        // Only looking at expressions
    }

}

impl<'err, 'builder, 'graph> BlockVisitor
    for ExprTypographer<'err, 'builder, 'graph> {

    fn visit_block(&mut self, block: &Block) {
        trace!("Visiting block with id {:?}", block.id());
        if block.id().is_default() {
            debug!("Skipping block without ID");
            return
        }

        // A valued empty block will also have previously triggered an error.
        if block.stmts().is_empty() {
            trace!("Visited empty block");
            self.current_type = self.primitive_type_ix("()");
            return
        }

        visit::walk_block(self, block);

        if block.has_source() {
            trace!("Checking block {:?} with source {:?}",
                block.id(), block.source().as_ref().expect("Checked"));
            let block_ix = self.graph.add_variable(block.id().clone());
            self.graph.add_inference(block_ix, self.current_type,
                InferenceSource::ImplicitReturn);
            self.current_type = block_ix;
        }
        else {
            trace!("Block does not require return value");
            self.current_type = self.primitive_type_ix("()");
        }
    }
}

impl<'err, 'builder, 'graph> StatementVisitor
    for ExprTypographer<'err, 'builder, 'graph> {

    // Use standard block handling.
    fn visit_do_block(&mut self, block: &DoBlock) {
        trace!("Visiting do block");
        visit::walk_do_block(self, block);
    }

    fn visit_if_block(&mut self, if_block: &IfBlock) {
        trace!("Visiting if block");
        if if_block.id().is_default() {
            debug!("Skipping if block without ID");
            return
        }

        let valued_if = if_block.has_source();
        let if_block_type = self.graph.add_variable(if_block.id().clone());

        let bool_ty_ix = self.primitive_type_ix("bool");
        for conditional in if_block.conditionals() {
            trace!("Checking conditional");
            self.visit_expression(conditional.condition());
            let cond_ty_id = self.current_type;
            // tcond = tbool
            self.graph.add_inference(cond_ty_id, bool_ty_ix,
                InferenceSource::IfConditionalBool);

            self.visit_block(conditional.block());
            trace!("Checking conditional block");
            if valued_if {
                trace!("Conditional block must match: {:?} == {:?}",
                    if_block_type, self.current_type);
                self.graph.add_inference(if_block_type, self.current_type,
                    InferenceSource::IfBranchesSame);
            }
        }

        if let Some(&(_, ref block)) = if_block.else_block() {
            trace!("Checking block else");
            self.visit_block(block);
            if valued_if {
                self.graph.add_inference(self.current_type, if_block_type,
                    InferenceSource::IfBranchesSame);
            }
        }

        // Update the current type, only if asked and we could theoretically
        // match.
        if valued_if {
            self.current_type = if_block_type;
        }
        // Otherwise always set `()`
        else {
            self.current_type = self.primitive_type_ix("()");
        }
    }

    fn visit_declaration(&mut self, decl: &Declaration) {
        trace!("Visiting declaration of {}", decl.ident().name());

        self.visit_expression(decl.value());

        // var matches declaration and declared type.
        let var_ix = self.graph.add_variable(decl.id().clone());

        if let Some(ty_expr) = decl.type_decl() {
            let ty_ix = self.graph.get_type(&ty_expr.id())
                .expect("Did not have type for existing type");

            // t_var: ty_expr
            self.graph.add_inference(var_ix, ty_ix,
                                     InferenceSource::ExplicitDecl(decl.ident().clone()));
        }
        // tvar = texpr
        self.graph.add_inference(var_ix, self.current_type,
                                 InferenceSource::Declaration(decl.ident().clone()));

        self.current_type = self.primitive_type_ix("()");
    }

    fn visit_return_stmt(&mut self, return_: &Return) {
        trace!("Visiting return type");
        // Expr matches block's return.
        // t_ret_expr = tfn
        self.current_type = self.primitive_type_ix("()");
        // return <expr>
        if let Some(expr) = return_.value() {
            self.visit_expression(expr);
            if self.current_type != self.primitive_type_ix("()") {
                self.graph.add_inference(self.current_type,
                                         self.fn_ret_type,
                                         InferenceSource::ExplicitReturn);
            }
        }
        // return
        else {
            // Need to make sure this is a type error.
            // ty_fn : ty_()
            let unary_type = self.primitive_type_ix("()");
            self.graph.add_inference(self.fn_ret_type, unary_type,
                InferenceSource::ExplicitReturn);
        }
    }
}

impl<'err, 'builder, 'graph> ExpressionVisitor
    for ExprTypographer<'err, 'builder, 'graph> {

    fn visit_var_ref(&mut self, ident: &Identifier) {
        trace!("Checking reference to {}", ident.name());
        // Set the type id to be the ident's type.
        // tx
        if ident.id().is_default() {
            debug!("Skipping unidentified var {}", ident.name());
            return
        }
        self.current_type = self.graph.variable(&ident.id())
            .expect("Graph did not contain identified variable");
    }

    fn visit_if_expr(&mut self, if_expr: &IfExpression) {
        // So first of all, we set the condition to be a boolean.

        // An interesting consideration is if we want to force the `if_expr`
        // to retain a type if not needed. For example:

        // fn foo() => ()
        // fn bar() => ()
        //
        // fn baz()
        //     if true => foo() else bar()
        //     // ...

        // In Rust:
        // fn foo() { if true { 1 } else { 2 } }
        //                      ^          ^ expected () got integer
        // fn foo() { if true { 1 } else { 2 }; } // Okay
        // fn foo() { if true { 1 } else { "str" } }
        // expected () got integer ^      ^ expected () got reference
        // fn foo() { if true { 1 } else { "str" }; }
        //            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected integer got reference
        // Rust wants you to use semicolons on the if branches:
        // fn foo() { if true { 1; } else { "str"; } } // Okay

        // In this case, the if expr doesn't _need_ to be an expr, it's never
        // used as a value.

        // I _think_ it's better to have the syntax also imply a balanced, valued
        // if expression.

        let if_expr_ty = self.graph.add_expression();

        self.visit_expression(if_expr.condition());
        let bool_ty_ix = self.primitive_type_ix("bool");

        self.graph.add_inference(self.current_type, bool_ty_ix,
            InferenceSource::IfConditionalBool);

        self.visit_expression(if_expr.true_expr());
        let left_ty_id = self.current_type;

        self.visit_expression(if_expr.else_expr());
        let right_ty_id = self.current_type;

        // We do not point them at each other here to avoid a loop.
        // I don't think inference can handle this right now.

        // ty_if_cond = ty_if_else
        self.graph.add_inference(right_ty_id, left_ty_id,
            InferenceSource::IfBranchesSame);

        // ty_if_expr: ty_if_cond
        self.graph.add_inference(if_expr_ty, left_ty_id,
            InferenceSource::IfBranchesSame);
        // ty_if_expr: ty_if_else
        self.graph.add_inference(if_expr_ty, left_ty_id,
            InferenceSource::IfBranchesSame);

        self.current_type = if_expr_ty;
    }

    fn visit_unary_op(&mut self, unary_op: &UnaryOperation) {
        let float_type = self.primitive_type_ix("float");
        // Require a numeric value for `-expr`
        match unary_op.operator() {
            UnaryOperator::Negation | UnaryOperator::Addition => {
                self.visit_expression(unary_op.inner());
                // t_expr = tint
                self.graph.add_inference(self.current_type, float_type,
                    InferenceSource::NumericOperator);
                let unary_op_expr_ty = self.graph.add_expression();
                self.graph.add_inference(unary_op_expr_ty, float_type,
                    InferenceSource::NumericOperator);
            },
        }
    }

    fn visit_binary_op(&mut self, bin_op: &BinaryOperation) {
        use ast::BinaryOperator::*;
        // Depending on the binary operation, we can infer types of each side.
        // Get the left and right TypeIds.
        self.visit_expression(bin_op.left());
        let left_type_id = self.current_type;

        self.visit_expression(bin_op.right());
        let right_type_id = self.current_type;

        let binop_type = self.graph.add_expression();

        match bin_op.operator() {
            Equality | NonEquality => {
                let bool_type = self.primitive_type_ix("bool");
                // lhs and rhs must be the same type, result is bool.
                // tright: tleft
                self.graph.add_inference(right_type_id, left_type_id,
                    InferenceSource::EqualityOperator);
                // t_binop = t_bool
                self.graph.add_inference(binop_type,
                    bool_type,
                    InferenceSource::EqualityOperator);
            },
            LessThan | GreaterThan | GreaterThanEquals | LessThanEquals => {
                // lhs and rhs are numeric, result is bool
                let float_type = self.primitive_type_ix("float");
                let bool_type = self.primitive_type_ix("bool");
                // ty_lhs: ty_number
                self.graph.add_inference(left_type_id, float_type,
                    InferenceSource::NumericOperator);
                // ty_rhs: ty_number
                self.graph.add_inference(right_type_id, float_type,
                    InferenceSource::NumericOperator);
                // ty_binop = ty_bool
                self.graph.add_inference(binop_type, bool_type,
                    InferenceSource::BooleanOperator);
            },
            Addition | Subtraction | Multiplication | Division | Modulus => {
                // lhs and rhs are numeric, result is numeric
                let float_type = self.primitive_type_ix("float");
                // lhs = number
                self.graph.add_inference(left_type_id, float_type,
                    InferenceSource::NumericOperator);
                // rhs = number
                self.graph.add_inference(right_type_id, float_type,
                    InferenceSource::NumericOperator);
                // tresult = number
                self.graph.add_inference(binop_type, float_type,
                    InferenceSource::NumericOperator);
            },
        }
        self.current_type = binop_type;
    }

    fn visit_assignment(&mut self, assign: &Assignment) {
        trace!("Visiting assignment");
        self.visit_expression(assign.rvalue());

        // var matches assignment type.
        let lvalue_id = assign.lvalue().id();
        if lvalue_id.is_default() {
            debug!("Skipping assignment to unknown lvalue {}",
                assign.lvalue().name());
            return
        }

        let lvalue_type = self.graph.variable(&lvalue_id)
            .expect("Graph did not have known lvalue of assignment");

        // tleft = tright
        self.graph.add_inference(self.current_type, lvalue_type,
            InferenceSource::Assignment);

        self.current_type = self.primitive_type_ix("()");
    }

    fn visit_literal_expr(&mut self, literal: &Literal) {
        trace!("Visiting literal");
        // We create a new ID with the literal's type.
        let literal_type_id =
            match *literal.value() {
                LiteralValue::Bool(_) => self.primitive_type_ix("bool"),
                LiteralValue::Float(_) => self.primitive_type_ix("float"),
                LiteralValue::Unit => self.primitive_type_ix("()")
            };
        let expr_ty = self.graph.add_expression();
        self.graph.add_inference(expr_ty, literal_type_id,
            InferenceSource::LiteralValue(literal.clone()));
        self.current_type = expr_ty;
    }

    fn visit_fn_call(&mut self, fn_call: &FnCall) {
        trace!("Visting a call to {}", fn_call.text());

        // fn foo() => ...
        // let x = foo
        // let y = x(1, 2)

        let fn_id = fn_call.id();
        if fn_id.is_default() {
            debug!("Ignoring call to unknown function {}", fn_call.text());
            return
        }

        // Attempt to find the function, either through top-level declaration
        // or through local binding.
        let fn_ix = self.graph.get_type(&fn_id)
            .or_else(|| self.graph.variable(&fn_id));

        if fn_ix.is_none() {
            debug!("Could not find type of function {}", fn_call.text());
            self.errors.add_error(CheckerError::new(
                fn_call.token().clone(),
                vec![],
                format!("Unknown function {}", fn_call.text())
            ));
            return
        }
        let fn_ix = fn_ix.expect("Checked");

        // We create an indirect node between call arguments and the function
        // type which the graph will simplify later.
        for (_arg_num, arg) in fn_call.args().iter().enumerate() {
            // t_arg: fnArg(arg, fn)
            self.visit_expression(arg.expression());
            let expr_ty = self.current_type;
            // https://github.com/immington-industries/protosnirk/issues/45
            // implicit params will be available in the future.
            let arg_infer = /*if let Some(ident) = arg.name()*/ {
                self.graph.add_named_call_arg(arg.name().name().into(), fn_ix)
            }
            /*else {
                self.graph.add_call_arg(arg_num, fn_ix)
            }*/;
            self.graph.add_inference(arg_infer, expr_ty,
                InferenceSource::CallArgument(fn_call.ident().clone()));
        }

        // t_current = t_return(fn)
        let fn_return_type = self.graph.add_call_return_type(fn_ix);
        self.current_type = fn_return_type;
    }
}
