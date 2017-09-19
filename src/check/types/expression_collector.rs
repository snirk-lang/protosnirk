use parse::ScopedId;
use parse::ast::*;
use parse::ast::types::*;

use check::ErrorCollector;
use check::types::environment::{TypeEnvironment,
                                TypeConstraint,
                                ConstraintSource};
use check::visitor::*;

use check::types::{TYPE_ID_INT, TYPE_ID_BOOL};

/// Collects type equations in expressions
///
/// We can consider the `ItemTypeCollector` to have already visited the AST,
/// so the invariants described in its struct docs are assumed.
///
/// # Invariants
/// After the expression collector has visited an expression it will hopefully
/// have enough type information to pass checking. We ask the environment to
/// resolve rules one function at a time by unifying rules under the function's
/// top-level `ScopedId`.
#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionTypeCollector<'err, 'env> {
    errors: &'err mut ErrorCollector,
    environment: &'env mut TypeEnvironment,
    /// TypeId of expression rvalues
    expr_type: TypeId,
    /// Keep a stack of the IDs of which block we're in.
    /// The top of the stack can be used for the return statement of a block.
    return_blocks: Vec<TypeId>,
    /// Stack of blocks which require a return stmt.
    ///
    /// AST nodes which cause a need for subnodes to return a value, such as
    /// lvalues in expressions, push a `ScopedId` to this stack. `return`
    /// expressions will probably target the top of the stack, whereas the last
    /// expression in a block will return a value for that block.
    return_require_stack: Vec<ScopedId>,
    /// ScopeId of the enclosing block or fn.
    ///
    /// This corresponds to the fn's ident. For the top-level block, use the
    /// first of `return_blocks`.
    enclosing_fn_id: ScopedId,
    /// Whether we've returned from the fn yet.
    /// I guess this needs to take branching into account?
    return_complete: bool
}

impl<'err, 'env> ExpressionTypeCollector<'err, 'env> {
    pub fn new(errors: &'err mut ErrorCollector,
               environment: &'env mut TypeEnvironment)
               -> ExpressionTypeCollector<'err, 'env> {
        ExpressionTypeCollector {
            errors,
            environment,
            enclosing_fn_id: ScopedId::default(),
            return_blocks: Vec::new(),
            expr_type: TypeId::default(),
            return_complete: false
        }
    }

    fn add_constraint(&mut self,
                      constraint: TypeConstraint,
                      source: ConstraintSource) {
        self.environment.add_constraint(self.current_scope, constraint, source);
    }
}

impl<'err, 'env> DefaultUnitVisitor for ExpressionTypeCollector<'err, 'env> { }

impl<'err, 'env> ItemVisitor for ExpressionTypeCollector<'err, 'env> {
    fn visit_block_fn_decl(&mut self, block_fn: &BlockFnDeclaration) {
        let fn_id = block_fn.get_ident().get_id();
        if fn_id.is_default() {
            return // redundant fn is not checked.
        }

        self.enclosing_fn_id = fn_id.clone();

        self.return_blocks.push(block_fn.get_block().get_id().clone());
        self.return_value_set = false;

        // If the fn's return type isn't empty, we want to set up a return type.
        if let Some(return_ty_expr) = block_fn.get_type_expr().get_return_type() {
            self.return_require_stack.push(
                block_fn.get_block().get_id().clone());
            let mut visitor =
                ExpressionTypeVisitor::new(&self.errors, &mut self.environment);
            visitor.visit_type_expr(return_ty_expr);
            let return_ty_id = visitor.into();
            self.curr_type_id = return_ty_id;
        }
        else {
            // fn returns ().
            self.add_constraint(
                TypeConstraint::FnReturnsUnit(fn_id.clone()),
                ConstraintSource::FnSignature
            );
        }
        visit::walk_block(block_fn.get_block());

        self.return_blocks.clear();
        self.return_require_stack.clear();

    }
}

impl<'err, 'env> BlockVisitor for ExpressionTypeCollector<'err, 'env> {
    fn visit_block(&mut self, block: &Block) {
        // declare block return type???

        // This is entirely possible if using a `// pass` comment.
        if block.statements().len() == 0 { return }

        // Correlate the last expr type with the block ID.
        // This information only matters if the block is being used as a value.
        let last_ix = block.statements.len() - 1usize;
        for (ix, stmt) in block.statements().iter().enumerate() {
            self.current_id = TypeId::default();
            self.visit_statement(stmt);
            if ix == last_ix {
                let last_stmt_ty_id = self.current_id;
                // Could still get the defaut `TypeId` if the last
                // stmt is an assignment or possibly if it's an unvalued block.
                if !last_stmt_ty_id.is_default() {
                    self.environment.add_constraint(
                        block.get_scope_id().clone(),
                        TypeConstraint::BlockHasType(block.get_scope_id.clone(),
                        ConstraintSource::ImplicitReturnMatch)
                    );
                }
            }
        }
    }
}

impl<'err, 'env> StatementVisitor for ExpressionTypeCollector<'err, 'env> {
    fn visit_do_block(&mut self, do_block: &DoBlock) {
        // No special handling here, it's just a regular block.
        visit::walk_do_block(self, do_block);
    }

    fn visit_if_block(&mut self, if_block: &IfBlock) {
        trace!("Visiting an if block");
        // The whole block is given a type id anyway and it's ignored/invalidated
        // if needed.
        let if_block_ty = self.environment.declare_new_type();
        // If there's no else, we can't return a value anyway.
        let block_has_else = if_block.has_else();
        // branches must match up
        for conditional in if_block.get_conditionals() {
            trace!("Checking if conditional expr");
            self.visit_expression(conditional.get_condition());
            if self.current_id.is_default() { return }
            let cond_ty_id = self.current_id;
            TYPE_ID_BOOL.with(|bool_type_id| {
                self.add_constraint(
                    TypeConstraint::TypesAreSame(cond_ty_id, bool_type_id),
                    ConstraintSource::IfConditionalBool
                );
            });
            self.current_id = TypeId::default();
            trace!("Checking if conditional block");
            self.visit_block(conditional.get_block());
            let this_arm_ty = self.current_id;

            // Require the arms to match only if there's an else.
            if block_has_else {
                self.add_constraint(
                    TypeConstraint::TypesAreSame(if_block_ty, this_arm_ty),
                    ConstraintSource::IfBranchesSame
                );
            }
        }
        if let Some((_, block)) = if_block.get_else() {
            trace!("Checking if conditional else");
            self.current_id = TypeId::default();
            self.visit_block(block);
            if self.current_id.is_default() { return }
            let else_block_ty = self.current_id;
            self.add_constraint(
                TypeConstraint::TypesAreSame(if_block_ty, else_block_ty),
                ConstraintSource::IfBranchesSame
            );
        }
        self.current_id = if_block_ty;
    }

    fn visit_return_stmt(&mut self, return_: &Return) {
        // expr matches block's return
        if let Some(expr) = return_.get_value() {
            self.visit_expression(expr);
            let ret_expr_ty = self.current_id;
            if ret_expr_ty.is_default() { return }
            let ret_block_id = self.return_blocks.first()
                .expect("Visiting return stmt without return blocks");
            self.add_constraint(
                TypeConstraint::FnReturnType(ret_expr_ty, ret_block_id.clone())
                ConstraintSource::ExplicitReturnMatch
            );
        }
    }
}

impl<'err, 'env> ExpressionVisitor for ExpressionTypeCollector<'err, 'env> {
    fn visit_var_ref(&mut self, ident: &Identifier) {
        // Keep track of referred-to ident?
        if let Some(known_type_id) = self.environment.get_type_id_of_var(ident.get_id()) {
            trace!("Found {} ({:?}) to have type ID {:?}",
                ident.get_name(), ident.get_id(), known_type_id);
            self.expr_type = known_type_id;
        }
        else {
            trace!("Declaring new type id for var {} ({:?})",
                ident.get_name(), ident.get_id());
            let type_id = self.environment.declare_var_new_type(ident.get_id().clone());
            self.expr_type = type_id;
        }
    }

    fn visit_if_expr(&mut self, if_expr: &IfExpression) {
        // condition must be boolean, exprs must match

        self.visit_expression(if_expr.get_condition());
        let cond_ty = self.expr_type;
        TYPE_ID_BOOL.with(|bool_type_id| {
            self.add_constraint(
                TypeConstraint::TypesAreSame(cond_ty, bool_type_id),
                ConstraintSource::IfConditionalBool
            );
        });

        self.visit_expression(if_expr.get_true_expr());
        let true_type_id = self.expr_type;
        self.visit_expression(if_expr.get_else());
        let else_type_id = self.expr_type;

        self.add_constraint(
            TypeConstraint::TypesAreSame(true_type_id, else_type_id),
            ConstraintSource::IfBranchesSame
        );
    }

    fn visit_unary_op(&mut self, un_op: &UnaryOperation) {
        // expr must be numeric
        match un_op.get_operator() {
            Operator::Subtraction | Operator::Addition => {
                // We have to constrain the interior id to be TYPE_ID_INT.
                self.visit_expression(un_op.get_inner());
                let inner_type_id = self.expr_type;
                TYPE_ID_INT.with(|type_id_int| {
                    self.add_constraint(
                        TypeConstraint::TypesAreSame(
                            inner_type_id, type_id_int),
                        ConstraintSource::NumericOperator);
                    self.inner_type_id = type_id_int;
                });
            },
            // This match should be exhaustive.
            // https://github.com/immington-industries/protosnirk/issues/29
            _ => {
                unreachable!("Unexpected unary operation {:?}", un_op);
            }
        }
    }

    fn visit_binary_op(&mut self, bin_op: &BinaryOperation) {
        use parse::ast::Operator::*;

        self.visit_expression(bin_op.get_left());
        let left_type_id = self.expr_type;
        self.visit_expression(bin_op.get_right());
        let right_type_id = self.expr_type;

        match bin_op.get_operator() {
            Equality | NonEquality => {
                // lhs and rhs are same, result is bool.
                TYPE_ID_BOOL.with(|type_id_bool| {
                    self.add_constraint(
                        TypeConstraint::TypesAreSame(left_type_id, right_type_id),
                        ConstraintSource::EqualityOperator
                    );
                    self.expr_type = type_id_bool;
                });
            },
            LessThan | GreaterThan | GreaterThanEquals | LessThanEquals => {
                // lhs and rhs are numeric, result is bool.
                let type_id_bool = TYPE_ID_BOOL.with(|ty_bool| ty_bool);
                TYPE_ID_INT.with(|type_id_int| {
                    self.add_constraint(
                        TypeConstraint::TypesAreSame(left_type_id, type_id_int),
                        ConstraintSource::NumericOperator
                    );
                    self.add_constraint(
                        TypeConstraint::TypesAreSame(right_type_id, type_id_int),
                        ConstraintSource::NumericOperator
                    );
                    self.expr_type = type_id_bool;
                });
            },
            Addition | Subtraction | Multiplication | Division | Modulus => {
                // lhs and rhs are numeric, result is numeric.
                TYPE_ID_INT.with(|type_id_int| {
                    self.add_constraint(
                        TypeConstraint::TypesAreSame(left_type_id, type_id_int),
                        ConstraintSource::NumericOperator
                    );
                    self.add_constraint(
                        TypeConstraint::TypesAreSame(right_type_id, type_id_int),
                        ConstraintSource::NumericOperator
                    );
                    self.expr_type = type_id_int;
                });
            },
            Custom => unreachable!("Unexpected binary operation {:?}", bin_op)
        }
    }

    fn visit_fn_call(&mut self, fn_call: &FnCall) {
        let fn_id = fn_call.get_name().get_id().clone();
        // TODO: need to check for a lot more errors.
        // Need type information about the fn in order to constrain the args.
        // Should we add a specific map<scopedid, fnDefinition> to the env?

        // This section is kind of painful to deal with.
        // Perhaps we should have a better definition of fn calling.

        match fn_call.get_args() {
            FnCallArgs::SingleExpr(call_expr) => {
                // This is what the previous pass would identify the first param as.
                let mut first_expr_scoped_id = fn_call.get-name().get_id().pushed();
                first_expr_scoped_id.push();
                first_expr_scoped_id.increment();
                self.visit_expression(call_expr);
                let expr_type = self.expr_type;
                self.add_constraint(
                    TypeConstraint::ValueForSingleFnArg(fn_id, expr_type),
                    ConstraintSource::FnSignature
                );
            },
            FnCallArgs::Arguments(call_args) => {
                for call_arg in call_args {
                    let param_ident = call_arg.get_name().get_scoped_id().clone();
                    if let Some(ref call_expr) = call_arg.get_expr() {
                        self.visit_expression(call_expr);
                        let arg_expr_type = self.expr_type;
                        self.add_constraint(
                            TypeConstraint::ValueForFnArg(
                                fn_id.clone(), param_ident, arg_expr_type
                            ),
                            ConstraintSource::FnSignature
                        );
                    }
                }
            }
        }

        // Create a new `TypeId` for a fn return type?
        // Note that right now we have a few extra vars. We could be a little simpler,
        // given the simplicity of the language right now.
        let ret_type_id = self.environment.declare_new_type();
        self.add_constraint(
            TypeConstraint::TypeIsFnReturned(ret_type_id, fn_id),
            ConstraintSource::FnSignature
        );
        self.expr_type = ret_type_id;
    }

    fn visit_assignment(&mut self, assignment: &Assignment) {
        // var must match assignment type
        let lvalue_scoped_id = assignment.get_lvalue().get_id();
        let lvalue_type_id = self.environment.get_type_id_of_var(lvalue_scoped_id)
            .expect("Could not find type ID of assignment, lvalue undeclared");
        self.visit_expression(assignment.get_rvalue());
        let rvalue_type_id = self.expr_type;
        self.add_constraint(
            TypeConstraint::TypesAreSame(lvalue_type_id, rvalue_type_id),
            ConstraintSource::VarAssignment
        );
        self.expr_type = TypeId::default();
    }

    fn visit_declaration(&mut self, decl: &Declaration) {
        // var must match decl type.
        // decl must match var declared type.
        let var_scope_id = decl.get_ident().get_id();
        let var_type_id = self.environment.declare_var_new_type(var_scope_id);

        // also restrict var_scope_id to the type decl.
        if let Some(type_decl) = decl.get_type_decl() {
            // This is going to require more work with type visiting in the future.
            let declared_type_id = ExpressionTypeVisitor::new(&self.environment)
                .visit_type_expr(type_decl)
                .into();
            self.add_constraint(
                TypeConstraint::TypesAreSame(var_type_id, declared_type_id),
                ConstraintSource::ExplicitVarDecl
            );
        }
        self.visit_expression(decl.get_value());
        let expr_type_id = self.expr_type;
        debug_assert!(!expr_type_id.is_default(),
            "No type ID from visiting an expression in declaration rvalue");
        self.adadd_constraint(
            TypeConstraint::TypesAreSame(var_type_id, expr_type_id),
            ConstraintSource::VarDeclValue
        );
    }
}

/// Simple visitor which uses the `TypeEnvironment` to place a `TypeId` for a given
/// `TypeExpression`.
///
/// This is used in places where a type is explicitly declared within a statement,
/// such as a declaration. In the future this wlll be used when generics are
/// explicily declared, or to resolve the `TypeId` of expressions like `Vector<String>`.
struct ExpressionTypeVisitor<'env, 'err> {
    errors: &'err ErrorCollector,
    environment: &'env mut TypeEnvironment,
    current_id: TypeId,
    current_scope: ScopedId
}

impl<'env, 'err> ExpressionTypeVisitor<'err, 'env> {
    pub fn new(errors: &'err ErrorCollector,
           environment: &'env mut TypeEnvironment)
           -> ExpressionTypeVisitor<'err, 'env> {
        ExpressionTypeVisitor {
            errors,
            environment,
            current_id: TypeId::default(),
            current_scope: ScopedId::default()
        }
    }
}

impl <'err, 'env> Into<TypeId> for ExpressionTypeVisitor<'err, 'env> {
    fn into(self) -> TypeId {
        self.current_id
    }
}

impl<'err, 'env> TypeVisitor for ExpressionTypeVisitor<'err, 'env> {
    fn visit_named_type_expr(&mut self, named_ty: &NamedTypeExpression) {
        if let Some(type_id) =
                self.environment.get_type_id_of_var(named_ty.get_ident()) {
            self.current_id = type_id;
        }
        else {
            // Unknown type here - need to figure out how to handle errors!
        }
    }
    fn visit_fn_type_expr(fn_ty: &FnTypeExpression) {
        unreachable!("ExpressionTypeVisitor should not be visiting fn types");
    }
}
