//! Identifies `TypeId`s in expressions.

use ast::*;
use ast::types::*;
use visit;
use visit::visitor::*;
use identify::{ConcreteType, TypeBuilder};
use check::{CheckerError, ErrorCollector};
use check::types::{TypeGraph, TypeExprIdentifier, InferenceSource};

use petgraph::graph::NodeIndex;

use std::collections::HashMap;

/// Creates type equations for functions.
#[derive(Debug, PartialEq)]
pub struct ExprTypeIdentifier<'err, 'builder, 'graph> {
    builder: &'builder TypeBuilder,
    errors: &'err mut ErrorCollector,
    graph: &'graph  mut TypeGraph,
    current_type: NodeIndex,
    fn_ret_expr: NodeIndex,
    fn_ret_explicit: bool,
}

impl<'err, 'builder, 'graph> ExprTypeIdentifier<'err, 'builder, 'graph> {
    pub fn new(builder: &'builder TypeBuilder,
               graph: &'graph mut TypeGraph,
               errors: &'err mut ErrorCollector)
               -> ExprTypeIdentifier<'err, 'builder, 'graph> {
        ExprTypeIdentifier {
            builder,
            errors,
            current_type: NodeIndex::default(),
            fn_ret_type: NodeIndex::default()
        }
    }
}

impl<'err, 'builder, 'graph> DefaultUnitVisitor
    for ExprTypeIdentifier<'err, 'builder, 'graph> { }

impl<'err, 'builder, 'graph> ItemVisitor
    for ExprTypeIdentifier<'err, 'builder, 'graph> {

    fn visit_block_fn_decl(&mut self, block_fn: &BlockFnDeclaration) {
        let fn_id = block_fn.get_id();
        if fn_id.is_default() { return }
        // We know the `identify/types/item` check has already added a concrete
        // type to the block fn.
        let fn_type_id = block_fn.get_ident().get_type_id();
        if fn_type_id == TypeId::default() {
            self.errors.add_error(CheckerError::new(
                block_fn.get_ident().get_token(),
                vec![],
                format!("Unable to infer type of function {}",
                    block_fn.get_ident().get_name())
            ));
            return
        }

        let fn_type = self.builder.get(fn_type_id)
                                  .expect("fn type already defined");

        // We've already set up information about `fn_ty` and the params.
        // Now, we're gonna require that the block return the same thing
        // as the function, and check the block.
        // We're also going to save the block type id so it can be used for
        // inferring returns.

        let fn_ret_type = match fn_type {
            ConcreteType::Function(ref func) => func.get_return_ty.clone(),
            _ => unreachable!("A function was inferred to not be a function")
        };

        let need_ret_value = fn_ret_type != ConcreteType::Primitive(Primitive::Unary);

        let fn_ix = self.graph.add_variable(fn_id.clone());
        let fn_ty_ix = self.graph.add_concrete(fn_type_id);

        // f: c
        self.graph.add_inference(fn_ix, fn_ty_ix,
            InferenceSource::FunctionSignature(block_fn.get_ident().clone()));

        // Add in connections to the parameter variables.
        for &(ref param_ident, ref param_expr) in block_fn.get_params() {
            // x: int
            let param_id = param_ident.get_id();
            match param_expr {
                TypeExpression::Primitive(prim) => {
                    let param_ty_id = self.builder
                                          .get(ConcreteType::Primitive(prim))
                                          .expect("Didn't have check primitive");
                    // x: <param type>
                    let param_ix = self.graph.add_variable(param_id);
                    let param_ty_ix = self.graph.add_concrete(param_ty_id);
                    self.graph.add_inference(param_ix, param_ty_ix,
                        InferenceSource::FnParameter(param_ident.clone()));
                },
                _ => unreachable!("Only primitive types are parsed")
            }
        }
        self.visit_block(block_fn.get_block());
        // Add inference for implicit return value.
        if need_ret_value && self.fn_ret_expr != NodeIndex::default() {
            let fn_ret_ty_ix = self.builder.get(fn_ret_type)
                                            .expect("Registered type");
            let inference = if self.fn_ret_explicit {
                InferenceSource::ImplicitReturn
            }
            else {
                InferenceSource::ExplicitReturn
            };
            // tf: t<ret expr>
            self.graph.add_inference(self.fn_ret_expr, fn_ret_ty_ix, inference);
        }
    }
}

impl<'err, 'builder, 'graph> BlockVisitor
    for ExprTypeIdentifier<'err, 'builder, 'graph> {

    fn visit_block(&mut self, block: &Block) {
        if block.get_id().is_default() { return }
        // If the block ends with an expression, that expression 's type
        // should be `self.curent_type`
        visit::walk_block(self, block);
    }
}

impl<'err, 'builder, 'graph> StatementVisitor
    for ExprTypeIdentifier<'err, 'builder, 'graph> {

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
        let block_is_typed =
            if_block.has_else() && !self.lvalue_ty_id.is_empty();
        if block_is_typed {
            let block_ty_id = self.builder.get_id(block_id.clone());
            if_block.set_type_id(block_ty_id);
        }

        let bool_type_ix = self.graph.add_type(
            self.builder.define_type(ConcreteType::Primitive(Primitive::Bool)));

        for conditional in if_block.get_conditionals() {
            trace!("Checking conditional");
            self.visit_expression(conditional.get_condition());
            let cond_ty_id = self.current_type;
            // tcond = tbool
            self.graph.add_inference(cond_ty_id, bool_type_ix,
                InferenceSource::IfConditionalBool);
            // tcond: if conditional
            self.builder.add_source(cond_ty_id,
                InferenceSource::IfConditionalBool(
                    conditional.get_token().clone()));
            self.var_type_id = TypeId::default();
            trace!("Checking conditional block");
            if block_is_typed {
                // If we're typed, ask the cond block to
                // type itself with us.
                self.lvalue_ty_id.push(if_block.get_type_id());
                self.var_type_id = TypeId::default();
            }
            self.visit_block(conditional.get_block());
        }

        if let Some(&(_, ref block)) = if_block.get_else() {
            trace!("Checking if else");
            if block_is_typed {
                self.lvalue_ty_id.push(if_block.get_type_id());
                self.var_type_id = TypeId::default();
            }
            self.visit_block(block);
        }
        if block_is_typed {
            self.var_type_id = if_block.get_type_id();
        }
    }

    fn visit_return_stmt(&mut self, return_: &Return) {
        // Expr matches block's return.
        // t_ret_expr = tfn
        self.var_type_id = TypeId::default();
        if let Some(expr) = return_.get_value() {
            self.visit_expression(expr);
            // So we're not enforcing anything about return expressions
            // if the function is not declared to return anything.
            // So that's kind of off.
            // On the other hand, we need to make sure not to implicitly
            // try to return anything if the fn's return type is undeclared.
            // Maybe we should be using more `InferredType` vars than ids?
            // OTOH operating on those will become more expensive over time.
            if !self.return_ty_id.is_default() {
                let ret_expr_ty = self.var_type_id;
                self.builder.add_equation(TypeEquation {
                    lhs: self.return_ty_id,
                    rhs: InferredType::Variable(ret_expr_ty)
                });
            }
        }
        else {
            self.builder.add_equation(TypeEquation {
                lhs: self.return_ty_id,
                rhs: InferredType::Known(
                    ConcreteType::Primitive(Primitive::Unary))
            });
        }
        // tfn: from this return stmt
        // This will help pin down mismatched return/return expr from -> ()
        self.builder.add_source(self.return_ty_id,
            InferenceSource::ExplicitReturn(return_.get_token().clone()));
    }
}

impl<'err, 'builder> ExpressionVisitor for ExprTypeIdentifier<'err, 'builder> {
    fn visit_var_ref(&mut self, ident: &Identifier) {
        // Set the type id to be the ident's type.
        // tx
        if ident.get_id().is_default() { return }
        let ident_type_id = self.builder.get_id(ident.get_id().clone());
        ident.set_type_id(ident_type_id);
        self.var_type_id = ident_type_id;
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

        self.var_type_id = TypeId::default();
        self.visit_expression(if_expr.get_condition());
        let cond_ty = self.var_type_id;

        // tcond = tbool
        self.builder.add_equation(TypeEquation {
            lhs: cond_ty,
            rhs: InferredType::Known(ConcreteType::Primitive(Primitive::Bool))
        });
        // tcond: if conditional bool
        self.builder.add_source(cond_ty,
            InferenceSource::IfConditionalBool(if_expr.get_token().clone()));

        self.visit_expression(if_expr.get_true_expr());
        let left_ty_id = self.var_type_id;

        self.visit_expression(if_expr.get_else());
        let right_ty_id = self.var_type_id;

        // if_left = if_right
        self.builder.add_equation(TypeEquation {
            lhs: left_ty_id,
            rhs: InferredType::Variable(right_ty_id)
        });

        // We're just gonna add both types to the if expression. This seems to be
        // what Rust does sometimes.

        // {left, right}: matches if expression
        self.builder.add_source(left_ty_id,
            InferenceSource::IfBranchesSame(if_expr.get_token().clone()));
        self.builder.add_source(right_ty_id,
            InferenceSource::IfBranchesSame(if_expr.get_token().clone()));
    }

    fn visit_unary_op(&mut self, unary_op: &UnaryOperation) {
        // Require a numeric value for `-expr`
        match unary_op.get_operator() {
            Operator::Subtraction | Operator::Addition => {
                self.visit_expression(unary_op.get_inner());
                let inner_type_id = self.var_type_id;
                // t_expr = tint
                self.builder.add_equation(TypeEquation {
                    lhs: inner_type_id,
                    rhs: InferredType::Known(ConcreteType::Primitive(Primitive::Int))
                });
                self.builder.add_source(inner_type_id,
                    InferenceSource::NumericOperator);
                // t_op
                let expr_type_id = self.builder.fresh_id();
                // t_op = t_int
                self.builder.add_equation(TypeEquation {
                    lhs: expr_type_id,
                    rhs: InferredType::Known(ConcreteType::Primitive(Primitive::Int))
                });
                // t_op: numeric operation
                self.builder.add_source(expr_type_id,
                    InferenceSource::NumericOperator);
                self.var_type_id = expr_type_id;
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
        let left_type_id = self.var_type_id;

        self.visit_expression(bin_op.get_right());
        let right_type_id = self.var_type_id;

        match bin_op.get_operator() {
            Equality | NonEquality => {
                // lhs and rhs must be the same type, result is bool.
                // tleft = tright
                self.builder.add_equation(TypeEquation {
                    lhs: left_type_id,
                    rhs: InferredType::Variable(right_type_id)
                });
                // tleft: in equality
                self.builder.add_source(left_type_id,
                    InferenceSource::EqualityOperator);
                // tright = tleft
                self.builder.add_equation(TypeEquation {
                    lhs: right_type_id,
                    rhs: InferredType::Variable(left_type_id)
                });
                // tright: in equality
                self.builder.add_source(right_type_id,
                    InferenceSource::EqualityOperator);
                // t_binop
                let expr_type_id = self.builder.fresh_id();
                // t_binop = t_bool
                self.builder.add_equation(TypeEquation {
                    lhs: expr_type_id,
                    rhs: InferredType::Known(ConcreteType::Primitive(Primitive::Int))
                });
                // tpinop: equality operator
                self.builder.add_source(expr_type_id,
                    InferenceSource::EqualityOperator);
                self.var_type_id = expr_type_id;

            },
            LessThan | GreaterThan | GreaterThanEquals | LessThanEquals => {
                // lhs and rhs are numeric, result is bool
                // lhs = number
                self.builder.add_equation(TypeEquation {
                    lhs: left_type_id,
                    rhs: InferredType::Known(
                        ConcreteType::Primitive(Primitive::Int))
                });
                self.builder.add_source(left_type_id,
                    InferenceSource::NumericOperator);
                // rhs = number
                self.builder.add_equation(TypeEquation {
                    lhs: right_type_id,
                    rhs: InferredType::Known(
                        ConcreteType::Primitive(Primitive::Int))
                });
                // rhs: numeric operation
                self.builder.add_source(right_type_id,
                    InferenceSource::NumericOperator);
                // tresult
                let expr_type_id = self.builder.fresh_id();
                // tresult = tbool
                self.builder.add_equation(TypeEquation {
                    lhs: expr_type_id,
                    rhs: InferredType::Known(
                        ConcreteType::Primitive(Primitive::Bool))
                });
                // tresult: boolean operation
                self.builder.add_source(expr_type_id,
                    InferenceSource::BooleanOperator);
                self.var_type_id = expr_type_id;
            },
            Addition | Subtraction | Multiplication | Division | Modulus => {
                // lhs and rhs are numeric, result is numeric
                // lhs = number
                self.builder.add_equation(TypeEquation {
                    lhs: left_type_id,
                    rhs: InferredType::Known(
                        ConcreteType::Primitive(Primitive::Int))
                });
                self.builder.add_source(left_type_id,
                    InferenceSource::NumericOperator);
                // rhs = number
                self.builder.add_equation(TypeEquation {
                    lhs: right_type_id,
                    rhs: InferredType::Known(
                        ConcreteType::Primitive(Primitive::Int))
                });
                // rhs: numeric operation
                self.builder.add_source(right_type_id,
                    InferenceSource::NumericOperator);
                // tresult
                let expr_type_id = self.builder.fresh_id();
                // tresult = tint
                self.builder.add_equation(TypeEquation {
                    lhs: expr_type_id,
                    rhs: InferredType::Known(
                        ConcreteType::Primitive(Primitive::Int))
                });
                // tresult: numeric operation
                self.builder.add_source(expr_type_id,
                    InferenceSource::NumericOperator);
                self.var_type_id = expr_type_id;
            },
            Custom => {
                unreachable!("Unexpected binary operation {:?}", bin_op)
            }
        }
    }

    fn visit_assignment(&mut self, assign: &Assignment) {
        // var matches assignment type.
        let lvalue_id = assign.get_lvalue().get_id();
        if lvalue_id.is_default() { return }
        let lvalue_type = self.builder.get_id(lvalue_id.clone());
        self.visit_expression(assign.get_rvalue());
        let rvalue_type = self.var_type_id;
        // tleft = tright
        self.builder.add_equation(TypeEquation {
            lhs: lvalue_type,
            rhs: InferredType::Variable(rvalue_type)
        });
        // tleft: from assignment
        self.builder.add_source(lvalue_type,
            InferenceSource::Assignment);
        // Can't get a value from an assignment (assign should be a statement)
        // https://github.com/immington-industries/protosnirk/issues/30
        self.var_type_id = TypeId::default();
    }

    fn visit_declaration(&mut self, decl: &Declaration) {
        // var matches declaration and declared type.
        let var_id = decl.get_ident().get_id();
        let var_type_id = self.builder.get_id(var_id.clone());

        if let Some(type_decl_expr) = decl.get_type_decl() {
            let declared_type = {
                let mut visitor = TypeExprIdentifier::new(self.builder, self.errors);
                visitor.visit_type_expr(type_decl_expr);
                visitor.get_type()
            };
            // tvar = tdeclared
            self.builder.add_equation(TypeEquation {
                lhs: var_type_id,
                rhs: declared_type
            });
            // tvar: explicily declared
            self.builder.add_source(var_type_id,
                InferenceSource::ExplicitDecl(decl.get_ident().clone()));
        }
        self.visit_expression(decl.get_value());
        let expr_type_id = self.var_type_id;
        // tvar = texpr
        self.builder.add_equation(TypeEquation {
            lhs: var_type_id,
            rhs: InferredType::Variable(expr_type_id)
        });
        // tvar: declared
        self.builder.add_source(var_type_id,
            InferenceSource::Declaration(decl.get_ident().clone()));
        self.var_type_id = TypeId::default();
    }

    fn visit_literal_expr(&mut self, literal: &Literal) {
        // We create a new ID with the literal's type.
        let literal_type_id = self.builder.fresh_id();
        match *literal.get_value() {
            LiteralValue::Bool(_) => {
                // tliteral = tbool
                self.builder.add_equation(TypeEquation {
                    lhs: literal_type_id,
                    rhs: InferredType::Known(ConcreteType::Primitive(Primitive::Bool))
                });
            },
            LiteralValue::Float(_) => {
                // tliteral = tfloat
                self.builder.add_equation(TypeEquation {
                    lhs: literal_type_id,
                    rhs: InferredType::Known(ConcreteType::Primitive(Primitive::Int))
                });
            },
            LiteralValue::Unit => {
                // tliteral = tunit
                self.builder.add_equation(TypeEquation {
                    lhs: literal_type_id,
                    rhs: InferredType::Known(ConcreteType::Primitive(Primitive::Unary))
                });
            }
        }
        // tliteral: it's a literal
        self.builder.add_source(literal_type_id,
            InferenceSource::LiteralValue(literal.clone()));
        self.var_type_id = literal_type_id;
    }

    fn visit_fn_call(&mut self, fn_call: &FnCall) {
        // tfn = (targ...) -> <expr_id>
        if fn_call.get_id().is_default() { return }
        let result_id = self.builder.fresh_id();
        let mut param_ids = HashMap::with_capacity(fn_call.get_args().len());
        match *fn_call.get_args() {
            FnCallArgs::SingleExpr(ref call_expr) => {
                self.visit_expression(call_expr);
                let arg_type = self.var_type_id;
                // targ: called by fn.
                self.builder.add_source(arg_type,
                    InferenceSource::CallArgument(fn_call.get_ident().clone()));
                // This is a hack for single-arg fns.
                param_ids.insert("".to_string(),
                    InferredType::Variable(arg_type));
            },
            FnCallArgs::Arguments(ref call_args) => {
                for call_arg in call_args {
                    let arg_id = match *call_arg.get_value() {
                        CallArgumentValue::Expression(ref expr) => {
                            self.visit_expression(expr);
                            self.var_type_id
                        },
                        CallArgumentValue::LocalVar(ref var) => {
                            self.visit_var_ref(var);
                            self.var_type_id
                        }
                    };
                    // targ: called by fn
                    self.builder.add_source(arg_id,
                        InferenceSource::CallArgument(fn_call.get_ident().clone()));
                    param_ids.insert(call_arg.get_name().to_string(),
                        InferredType::Variable(arg_id));
                }
            }
        }
        let fn_id = self.builder.get_id(fn_call.get_id().clone());
        // tfn = (targ... -> tresult)
        self.builder.add_equation(TypeEquation {
            lhs: fn_id,
            rhs: InferredType::Fn {
                params: param_ids,
                return_type: Box::new(InferredType::Variable(result_id))
            }
        });
        // tresult: fn call result
        self.builder.add_source(result_id,
            InferenceSource::CallReturnType(fn_call.get_ident().clone()));

        self.var_type_id = result_id;
    }
}
