//! Builds the `TypeGraph` using code within functions

use ast::*;
use visit;
use visit::visitor::*;
use identify::{ConcreteType, FnType, TypeScopeBuilder};
use check::{CheckerError, ErrorCollector};
use check::types::{TypeGraph, InferenceSource};

use petgraph::graph::NodeIndex;

use std::collections::HashMap;
use std::ops::Deref;

/// Creates type equations for functions.
#[derive(Debug)]
pub struct ExprTypeChecker<'err, 'builder, 'graph> {
    /// Constructs mapping from ScopedId -> ConcreteType
    builder: &'builder TypeScopeBuilder,
    /// Collects errors
    errors: &'err mut ErrorCollector,
    /// Performs type inference on expressions
    graph: &'graph mut TypeGraph,
    /// Type index obtained by visiting an expression
    current_type: NodeIndex,
    /// The return type of the function for use of the `return` keyword
    current_fn_return_type: NodeIndex,
    /// Whether the current expression is used as an rvalue
    current_expr_rvalue: bool
}

impl<'err, 'builder, 'graph> ExprTypeChecker<'err, 'builder, 'graph> {
    pub fn new(builder: &'builder TypeScopeBuilder,
               errors: &'err mut ErrorCollector,
               graph: &'graph mut TypeGraph)
               -> ExprTypeChecker<'err, 'builder, 'graph> {
        ExprTypeChecker {
            builder,
            errors,
            graph,
            current_type: NodeIndex::default(),
            current_fn_return_type: NodeIndex::default(),
            current_expr_rvalue: false,
        }
    }

    /// Get the "injected" primitive type
    fn primitive_type_ix(&self, name: &str) -> NodeIndex {
        self.builder.get_named_type_id(name)
            .and_then(|unary_id| self.graph.get_type(unary_id))
            .expect("Primitive")
    }
}

impl<'err, 'builder, 'graph> DefaultUnitVisitor
    for ExprTypeChecker<'err, 'builder, 'graph> { }

impl<'err, 'builder, 'graph> ItemVisitor
    for ExprTypeChecker<'err, 'builder, 'graph> {

    fn visit_block_fn_decl(&mut self, block_fn: &BlockFnDeclaration) {
        let fn_id = block_fn.get_id();
        if fn_id.is_default() { return }
        if block_fn.get_return_type().get_id().is_default() { return }
        // We know the `identify/types/item` check has already added a concrete
        // type to the block fn.

        let fn_type = match self.builder.get_type(&fn_id) {
            Some(fn_type) => fn_type,
            // If it's not defined, there's already an error.
            None => return
        };

        // We've already set up information about `fn_ty` and the params.
        // Now, we're gonna require that the block returns the same thing
        // as the function, and check the block.
        // We're also going to save the block type id so it can be used for
        // inferring returns.

        let need_ret_value =
            block_fn.get_return_type().get_id().deref() !=
            self.builder.get_named_type_id("()").expect("Primitive");

        self.current_fn_return_type = self.graph.get_type(
            &block_fn.get_return_type().get_id())
            .expect("Could not determine fn return type");

        let fn_ix = self.graph.add_variable(fn_id.clone());
        let fn_ty_ix = self.graph.add_type(fn_id.clone());

        // var_f: ty_f
        self.graph.add_inference(fn_ix, fn_ty_ix,
            InferenceSource::FnSignature(block_fn.get_ident().clone()));

        // Add in connections to the parameter variables.
        for &(ref param_ident, ref param_expr) in block_fn.get_params() {
            let param_id = param_ident.get_id();
            let param_ty_id = param_expr.get_id();

            // x: <param type>
            let param_ix = self.graph.add_variable(param_id.clone());
            let param_ty_ix = self.graph.add_type(param_ty_id.clone());

            // var_param: ty_param
            self.graph.add_inference(param_ix, param_ty_ix,
            InferenceSource::FnParameter(param_ident.clone()));
        }
        self.visit_block(block_fn.get_block());
        // Add inference for implicit return value.

        // If the function needs a return expression, add type inference from
        // the last statement of the function's block.
        if need_ret_value {
            let fn_ret_type = block_fn.get_return_type();
            let fn_ret_id = fn_ret_type.get_id();
            let fn_ret_type = self.builder.get_type(&fn_ret_id)
                .expect("Function checked out but return type didn't");
            // expr_inret: ty_fn_ret
            self.graph.add_inference(self.current_type, fn_ix,
                InferenceSource::FnReturnType(block_fn.get_ident().clone()));
        }
    }
}

impl<'err, 'builder, 'graph> BlockVisitor
    for ExprTypeChecker<'err, 'builder, 'graph> {

    fn visit_block(&mut self, block: &Block) {
        if block.get_id().is_default() { return }
        // If the block ends with an expression, that expression 's type
        // should be `self.curent_type`
        visit::walk_block(self, block);
    }
}

impl<'err, 'builder, 'graph> StatementVisitor
    for ExprTypeChecker<'err, 'builder, 'graph> {

    // Use standard block handling.
    fn visit_do_block(&mut self, block: &DoBlock) {
        visit::walk_do_block(self, block);
    }

    fn visit_if_block(&mut self, if_block: &IfBlock) {
        trace!("Visiting if block");
        if if_block.get_id().is_default() { return }
        // Get the type ID and start figuring out whether the block itself
        // needs to be typed.
        let block_id = if_block.get_id();

        // If block has its own expression, may not be used.
        let expr_id = self.graph.add_expression();

        let bool_ty_ix = self.primitive_type_ix("bool");

        for conditional in if_block.get_conditionals() {
            trace!("Checking conditional");
            self.visit_expression(conditional.get_condition());
            let cond_ty_id = self.current_type;
            // tcond = tbool
            self.graph.add_inference(cond_ty_id, bool_ty_ix,
                InferenceSource::IfConditionalBool);

            self.current_type = NodeIndex::default();
            trace!("Checking conditional block");
            self.visit_block(conditional.get_block());
            if self.current_expr_rvalue {
                self.graph.add_inference(self.current_type, expr_id,
                    InferenceSource::IfBranchesSame);
            }
        }

        if let Some(&(_, ref block)) = if_block.get_else() {
            trace!("Checking block else");
            self.visit_block(block);
            if self.current_expr_rvalue {
                self.graph.add_inference(self.current_type, expr_id,
                    InferenceSource::IfBranchesSame);
            }
        }
        // Update the current type, only if asked and we could theoretically
        // match.
        if self.current_expr_rvalue && if_block.has_else() {
            self.current_type = expr_id;
        }
        // Otherwise always set `()`
        else {
            self.current_type = self.primitive_type_ix("()");
        }
    }

    fn visit_return_stmt(&mut self, return_: &Return) {
        // Expr matches block's return.
        // t_ret_expr = tfn
        self.current_type = self.primitive_type_ix("()");
        // return <expr>
        if let Some(expr) = return_.get_value() {
            self.visit_expression(expr);
            // So we're not enforcing anything about return expressions
            // if the function is not declared to return anything.
            // So that's kind of off.
            // On the other hand, we need to make sure not to implicitly
            // try to return anything if the fn's return type is undeclared.
            // Maybe we should be using more `InferredType` vars than ids?
            // OTOH operating on those will become more expensive over time.
            if self.current_type != self.primitive_type_ix("()") {
                self.graph.add_inference(self.current_type,
                                         self.current_fn_return_type,
                                         InferenceSource::ExplicitReturn);
            }
        }
        // return
        else {
            // Need to make sure this is a type error.
            // ty_fn : ty_()
            let unary_type = self.primitive_type_ix("())");
            self.graph.add_inference(self.current_fn_return_type, unary_type,
                InferenceSource::ExplicitReturn);
        }
    }
}

impl<'err, 'builder, 'graph> ExpressionVisitor
    for ExprTypeChecker<'err, 'builder, 'graph> {

    fn visit_var_ref(&mut self, ident: &Identifier) {
        // Set the type id to be the ident's type.
        // tx
        if ident.get_id().is_default() { return }
        self.current_type = self.graph.add_variable(ident.get_id().clone());
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

        self.visit_expression(if_expr.get_condition());
        let bool_ty_ix = self.primitive_type_ix("bool");

        self.graph.add_inference(self.current_type, bool_ty_ix,
            InferenceSource::IfConditionalBool);

        self.visit_expression(if_expr.get_true_expr());
        let left_ty_id = self.current_type;

        self.visit_expression(if_expr.get_else());
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
        match unary_op.get_operator() {
            Operator::Subtraction | Operator::Addition => {
                self.visit_expression(unary_op.get_inner());
                // t_expr = tint
                self.graph.add_inference(self.current_type, float_type,
                    InferenceSource::NumericOperator);
                let unary_op_expr_ty = self.graph.add_expression();
                self.graph.add_inference(unary_op_expr_ty, float_type,
                    InferenceSource::NumericOperator);
            },
            // This match should be exhaustive.
            // https://github.com/immington-industries/protosnirk/issues/29
            _ => {
                unreachable!("Unexpected unary operation {:?}", unary_op);
            }
        }
    }

    fn visit_binary_op(&mut self, bin_op: &BinaryOperation) {
        use ast::Operator::*;
        // Depending on the binary operation, we can infer types of each side.
        // Get the left and right TypeIds.
        self.visit_expression(bin_op.get_left());
        let left_type_id = self.current_type;

        self.visit_expression(bin_op.get_right());
        let right_type_id = self.current_type;

        let binop_type = self.graph.add_expression();

        match bin_op.get_operator() {
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
            Custom => {
                unreachable!("Unexpected binary operation {:?}", bin_op)
            }
        }
        self.current_type = binop_type;
    }

    fn visit_assignment(&mut self, assign: &Assignment) {
        // var matches assignment type.
        let lvalue_id = assign.get_lvalue().get_id();
        if lvalue_id.is_default() { return }

        let lvalue_type = self.graph.get_variable(&lvalue_id)
            .expect("Did not know type of existing variable");

        self.visit_expression(assign.get_rvalue());

        // tleft = tright
        self.graph.add_inference(self.current_type, lvalue_type,
            InferenceSource::Assignment);

        self.current_type = self.primitive_type_ix("()");
    }

    fn visit_declaration(&mut self, decl: &Declaration) {
        // var matches declaration and declared type.
        let var_ix = self.graph.add_variable(decl.get_id().clone());

        if let Some(ty_expr) = decl.get_type_decl() {
            let ty_ix = self.graph.get_type(&ty_expr.get_id())
                .expect("Did not have type for existing type");

            // t_var: ty_expr
            self.graph.add_inference(var_ix, ty_ix,
                InferenceSource::ExplicitDecl(decl.get_ident().clone()));
        }
        self.visit_expression(decl.get_value());
        // tvar = texpr
        self.graph.add_inference(var_ix, self.current_type,
            InferenceSource::Declaration(decl.get_ident().clone()));

        self.current_type = self.primitive_type_ix("()");
    }

    fn visit_literal_expr(&mut self, literal: &Literal) {
        // We create a new ID with the literal's type.
        let literal_type_id =
            match *literal.get_value() {
                LiteralValue::Bool(_) => self.primitive_type_ix("bool"),
                LiteralValue::Float(_) => self.primitive_type_ix("float"),
                LiteralValue::Unit => self.primitive_type_ix("()")
            };
        self.current_type = literal_type_id;
    }

    fn visit_fn_call(&mut self, fn_call: &FnCall) {
        use ast::CallArgument;

        // fn foo() => ...
        // let x = foo
        // let y = x(1, 2)

        let fn_id = fn_call.get_id();
        if fn_id.is_default() { return }

        // Attempt to find the function, either through top-level declaration or
        // through local binding.
        let fn_ix = self.graph.get_type(&fn_id)
            .or_else(|| self.graph.get_variable(&fn_id));

        if fn_ix.is_none() {
            self.errors.add_error(CheckerError::new(
                fn_call.get_token().clone(),
                vec![],
                format!("Unknown function {}", fn_call.get_text())
            ));
            return
        }
        let fn_ix = fn_ix.expect("Checked");

        // We create an indirect node between call arguments and the function
        // type which the graph will simplify later.
        for (arg_num, arg) in fn_call.get_args().iter().enumerate() {
            // t_arg: fnArg(arg, fn)
            self.visit_expression(arg.get_expression());
            let expr_ty = self.current_type;
            let arg_infer = if let Some(ident) = arg.get_name() {
                self.graph.add_named_call_arg(ident.get_name().into(), fn_ix)
            }
            else {
                self.graph.add_call_arg(arg_num, fn_ix)
            };
            self.graph.add_inference(expr_ty, arg_infer,
                InferenceSource::CallArgument(fn_call.get_ident().clone()));
        }

        // t_current = t_return(fn)
        let fn_return_type = self.graph.add_call_return_type(fn_ix);
        self.current_type = fn_return_type;
    }
}
