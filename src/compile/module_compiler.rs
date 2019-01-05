use std::collections::HashMap;

use ast::{*, visit::*};
use identify::ConcreteType;
use check::TypeMapping;
use compile::ModuleProvider;

use llvm_sys::{LLVMIntPredicate, LLVMRealPredicate, LLVMTypeKind};
use llvm_sys::analysis::LLVMVerifierFailureAction;

use llvm::{Module, Value, Type, Builder, Context};

//#[derive(Debug)]
// https://github.com/immington-industries/protosnirk/issues/52
/// Produces LLVM modules for AST `Unit`s
pub struct ModuleCompiler<'ctx, 'b, M: ModuleProvider<'ctx>> where 'ctx: 'b {
    module_provider: M,
    optimizations: bool,
    context: &'ctx Context,
    builder: &'b Builder<'ctx>,
    ir_code: &'b mut Vec<Value<'ctx>>,
    current_type: Type<'ctx>,
    types: TypeMapping,
    scope_manager: &'b mut HashMap<ScopedId, Value<'ctx>>,
}

impl<'ctx, 'b, M: ModuleProvider<'ctx>> ModuleCompiler<'ctx, 'b, M> {
    pub fn new(types: TypeMapping,
               provider: M,
               context: &'ctx Context,
               builder: &'b Builder<'ctx>,
               ir_code: &'b mut Vec<Value<'ctx>>,
               scope_manager: &'b mut HashMap<ScopedId, Value<'ctx>>,
               optimizations: bool) -> ModuleCompiler<'ctx, 'b, M> {
        ModuleCompiler {
            module_provider: provider,
            builder,
            context,
            types,
            ir_code,
            scope_manager,
            optimizations,
            current_type: Type::void(&context),
        }
    }
    pub fn decompose(self) -> (M, TypeMapping) {
        (self.module_provider, self.types)
    }

    fn current_module(&self) -> &Module<'ctx> {
        self.module_provider.module()
    }

    fn llvm_type_of(&self, id: &ScopedId) -> Type<'ctx> {
        trace!("Finding type of ID {:?}", id);
        let concrete = self.types.get(id)
            .expect("Attempted to find unknown type");
        self.llvm_type_of_concrete(concrete)
    }

    fn llvm_type_of_concrete(&self, concrete: &ConcreteType) -> Type<'ctx> {
        match concrete {
            &ConcreteType::Named(ref name) => {
                match name.name() {
                    "()" => Type::void(&self.context),
                    "bool" => Type::int1(&self.context),
                    "float" => Type::double(&self.context),
                    other => panic!("Unexpected concrete type {}", other)
                }
            },
            &ConcreteType::Function(ref fn_ty) => {
                let mut params = Vec::new();
                for &(ref _name, ref param_ty) in fn_ty.params() {
                    params.push(self.llvm_type_of_concrete(param_ty));
                }
                Type::function(
                    &self.llvm_type_of_concrete(fn_ty.return_ty()),
                    params, false)
            }
        }
    }
}

impl<'ctx, 'b, M> UnitVisitor for ModuleCompiler<'ctx, 'b, M>
    where M: ModuleProvider<'ctx>, 'ctx: 'b {

    fn visit_unit(&mut self, unit: &Unit) {
        trace!("Checking a unit");

        visit::walk_unit(self, unit);

        // The final ir_code value should be a reference to the function
        match self.current_module()
                .verify(LLVMVerifierFailureAction::LLVMPrintMessageAction) {

            Ok(_) => (),
            Err(_) => {
                info!("Module:");
                self.current_module().dump();
            }
        }
    }
}

impl<'ctx, 'b, M> ItemVisitor for ModuleCompiler<'ctx, 'b, M>
    where M: ModuleProvider<'ctx>, 'ctx: 'b {

    fn visit_block_fn_decl(&mut self, block_fn: &BlockFnDeclaration) {
        trace!("Checking declaration of {}", block_fn.name());

        let fn_type = self.llvm_type_of(&block_fn.id());
        let fn_ret_type_kind = fn_type.return_type()
            .expect("Block fn's LLVM type did not have a return type")
            .get_kind();
        let fn_returns_void =
            fn_ret_type_kind == LLVMTypeKind::LLVMVoidTypeKind;
        let fn_ref = self.current_module().add_function(
            block_fn.name(), &fn_type);

        // Gotta insert the fn ref first so it can be called recursively
        self.scope_manager.insert(block_fn.id().clone(), fn_ref.clone());
        trace!("Inserted {} into the scope manager",
            block_fn.name());

        // Gonna be fancy and have a separate basic block for parameters
        let entry_block = self.context.append_basic_block(&fn_ref, "entry");
        let start_block = self.context.append_basic_block(&fn_ref, "start");
        self.builder.position_at_end(&entry_block);
        trace!("Ready to build {}", block_fn.name());

        let fn_params = fn_ref.get_params();
        trace!("fn has {} params", fn_params.len());

        // Rename args to %argname, create+remember allocas and store the
        // function values there. This allows LLVM to mutate function params
        // even if we don't allow it right now.
        for (&(ref ast_param, _), ref ir_param) in
                        block_fn.params().iter().zip(fn_ref.get_params()) {
            trace!("Adding fn param {} (ix {:?})",
                ast_param.name(), ast_param.id());
            ir_param.set_name(ast_param.name());
            let param_type = self.llvm_type_of(&ast_param.id());
            let alloca = self.builder
                .build_alloca(&param_type, ast_param.name());
            self.builder.build_store(&ir_param, &alloca);
            self.scope_manager.insert(ast_param.id().clone(), alloca);
        }
        self.builder.build_br(&start_block);
        self.builder.position_at_end(&start_block);

        trace!("Moving to check the block");

        // Compile the function
        self.visit_block(&block_fn.block());

        if !fn_returns_void {
            if let Some(remaining_expr) = self.ir_code.pop() {
                trace!("Found final expression, appending a return");
                self.builder.build_ret(&remaining_expr);
            }
            else {
                trace!(
                    "No IR code remaining, assuming the last stmt was return");
            }
        }
        else {
            trace!("block fn returns void, appending ret void");
            self.builder.build_ret_void();
        }


        if !fn_ref.verify(LLVMVerifierFailureAction::LLVMPrintMessageAction) {
            error!("Failed to verify {}", block_fn.name());
            error!("Current module IR:\n{}", self.current_module().print_to_string());
            panic!("Validation error for {}", block_fn.name());
        }

        if self.optimizations {
            trace!("Running optimizations on fn {}", block_fn.name());
            self.module_provider.pass_manager().run(&fn_ref);
        }
    }

    fn visit_typedef(&mut self, _typedef: &Typedef) {
        // skip, typedef is not compiled.
    }
}

impl<'ctx, 'b, M> BlockVisitor for ModuleCompiler<'ctx, 'b, M>
    where M: ModuleProvider<'ctx>, 'ctx: 'b {

    fn visit_block(&mut self, block: &Block) {
        trace!("Visiting block");
        // We know from typeck that the last block statement must be an
        // expression. So we just walk the block and assume that self.ir_code
        // will receive the last expression.
        visit::walk_block(self, block);
        if block.has_source() {
            trace!("Block has source, setting ID");
            self.current_type = self.llvm_type_of(&block.id());
        }
    }
}

impl<'ctx, 'b, M> StatementVisitor for ModuleCompiler<'ctx, 'b, M>
    where M: ModuleProvider<'ctx>, 'ctx: 'b {

    fn visit_do_block(&mut self, do_block: &DoBlock) {
        trace!("Visiting do block");
        visit::walk_do_block(self, do_block);
    }

    fn visit_if_block(&mut self, if_block: &IfBlock) {
        trace!("Checking if block");
        // Create some lists of values to use later
        let condition_count = if_block.conditionals().len();
        let valued_if = if_block.has_source();
        let function = self.builder.insert_block().get_parent()
            .expect("Just inserted a block");

        let mut condition_blocks = Vec::with_capacity(condition_count);
        let mut incoming_values =
            Vec::with_capacity(if valued_if { condition_count } else {0});

        trace!("Preparing to emit {} conditionals", condition_count);
        // Populate a list of the future blocks to have
        for (ix, _conditional) in if_block.conditionals().iter().enumerate() {
            trace!("Creating condition block {}", ix);
            // We skip adding the first one to this list because we know we
            // will have at least one later so we handle it separately.
            if ix != 0usize {
                let name = format!("if_{}_cond", ix + 1);
                condition_blocks.push(
                    self.context.append_basic_block(&function, &name)
                );
            }
            let name = format!("if_{}_then", ix + 1);
            condition_blocks.push(
                self.context.append_basic_block(&function, &name)
            );
        }
        // If there's an else it needs a block
        if if_block.has_else() {
            trace!("Creating else block");
            condition_blocks.push(
                self.context.append_basic_block(&function, "else_block")
            );
        }

        let int1_type = Type::int1(self.context);
        let int1_zero = int1_type.const_int(0u64, false);

        trace!("Creating end block");
        condition_blocks.push(self.context.append_basic_block(&function,
                                                                     "if_end"));

        let mut ix = 0;
        for conditional in if_block.conditionals() {
            trace!("Checking expr for condition {}", ix);
            self.visit_expression(conditional.condition());
            let cond_value = self.ir_code.pop()
                .expect("Did not get IR value from if block condition");
            let cond_cmp_name = format!("if_{}_cmp", ix);
            let cond_cmp = self.builder.build_icmp(LLVMIntPredicate::LLVMIntEQ,
                    &cond_value, &int1_zero, &cond_cmp_name);

            trace!("Building a break to next blocks {} -> {}, {}",
                cond_cmp_name, ix, ix + 1);
            self.builder.build_cond_br(&cond_cmp,
                                                 &condition_blocks[ix],
                                                 &condition_blocks[ix + 1]);

            // Go to the `if_true` block of this conditional
            trace!("Positioning at end of cond block {}", ix);
            self.builder.position_at_end(&condition_blocks[ix]);
            trace!("Checking conditional block");
            self.visit_block(conditional.block());
            // If this is a valued if, save the value ref for this branch of the condition
            if valued_if {
                let value = self.ir_code.pop()
                    .expect("Did not get value from valued if block");
                incoming_values.push(value);
            }

            // After block, go to done
            trace!("Adding branch to cond end block");
            let last_ix = condition_blocks.len() - 1;
            self.builder.build_br(&condition_blocks[last_ix]);

            // Position at the beginning of the next block
            trace!("Moving onto block {}", ix + 1);
            self.builder.position_at_end(&condition_blocks[ix + 1]);
            ix += 2;
        }

        trace!("Finished checking conditions");
        // If there's an else, check that too
        if let Some(&(_, ref else_block)) = if_block.else_block() {
            trace!("Checking else block");
            self.visit_block(else_block);
            if valued_if {
                let value = self.ir_code.pop()
                    .expect("Did not get value from else of valued if block");
                incoming_values.push(value);
            }
            // Branch to end after else
            let last_ix = condition_blocks.len() - 1;
            self.builder.build_br(&condition_blocks[last_ix]);
        }

        // Remove the end block from condition blocks for borrowck + phi reasons
        let cond_end_block = condition_blocks.pop()
            .expect("Somehow there were 0 conditional blocks");
        // Position at end block - this lets us get on with the function
        self.builder.position_at_end(&cond_end_block);

        // If we need to push a value, create a phi
        if valued_if {
            let mut incoming_conditions = condition_blocks
                .chunks(2)
                .map(|c| c[0].clone())
                .collect::<Vec<_>>();
            incoming_conditions.push(condition_blocks.pop()
                .expect("No condition blocks"));
            trace!("Generating phi node with {} values and {} edges",
                incoming_values.len(), incoming_conditions.len());
            let phi_type = self.llvm_type_of(&if_block.id());
            let phi = self.builder.build_phi(&phi_type, "if_phi");
            phi.add_incoming(incoming_values, incoming_conditions);
            self.ir_code.push(phi);
            self.current_type = phi_type;
        }
    }

    fn visit_return_stmt(&mut self, return_: &Return) {
        trace!("Checking return statement");
        if let Some(ref return_expr) = return_.value() {
            self.visit_expression(return_expr);
            let return_val = self.ir_code.pop()
                .expect("Could not generate value of return");
            self.builder.build_ret(&return_val);
        }
        else {
            self.builder.build_ret_void();
        }
        self.current_type = Type::void(&self.context);
    }
}

impl<'ctx, 'b, M> ExpressionVisitor for ModuleCompiler<'ctx, 'b, M>
    where M: ModuleProvider<'ctx>, 'ctx: 'b {

    fn visit_literal_expr(&mut self, literal: &Literal) {
        use ast::LiteralValue;
        trace!("Checking literal {}", literal.token().text());
        let (literal_value, literal_type) = match literal.value() {
            &LiteralValue::Bool(b) => {
                let bool_value = if b { 1u64 } else { 0u64 };
                (Type::int1(&self.context)
                     .const_int(bool_value, false),
                 Type::int1(&self.context))
            },
            &LiteralValue::Float(f) => {
                (Type::double(&self.context).const_real(f),
                Type::double(&self.context))
            },
            &LiteralValue::Unit => {
                // Not directly used.
                //Type::void(self.context).const_null()
                unimplemented!()
            }
        };
        self.current_type = literal_type;
        self.ir_code.push(literal_value);
    }

    fn visit_var_ref(&mut self, ident_ref: &Identifier) {
        trace!("Checking variable ref {} ({:?})",
            ident_ref.name(), ident_ref.id());
        let var_alloca = self.scope_manager.get(&ident_ref.id())
            .expect("Attempted to check var ref but had no alloca for it")
            .clone();
        let load_name = format!("load_{}", ident_ref.name());
        trace!("Creating {}", load_name);
        let builder = self.builder;
        let var_load = builder.build_load(&var_alloca, &load_name);
        self.current_type = self.llvm_type_of(&ident_ref.id());
        self.ir_code.push(var_load);
    }


    fn visit_declaration(&mut self, decl: &Declaration) {
        trace!("Checking declaration for {}", decl.name());
        self.visit_expression(decl.value());
        let decl_value = self.ir_code.pop()
            .expect("Did not have rvalue of declaration");
        let builder = self.builder;
        let alloca = builder.build_alloca(&self.current_type, decl.name());
        builder.build_store(&decl_value, &alloca);
        self.scope_manager.insert(decl.id().clone(), alloca);
    }

    fn visit_assignment(&mut self, assign: &Assignment) {
        trace!("Checking assignment of {}", assign.lvalue().name());
        self.visit_expression(assign.rvalue());
        let rvalue = self.ir_code.pop()
            .expect("Could not generate rvalue of assignment");
        let var_alloca = self.scope_manager.get(&assign.lvalue().id())
            .expect("Could not find existing var for assignment!")
            .clone();
        let builder = self.builder;
        builder.build_store(&rvalue, &var_alloca);
    }

    fn visit_unary_op(&mut self, unary_op: &UnaryOperation) {
        debug_assert!(unary_op.operator() == UnaryOperator::Negation,
            "Invalid unary operator {:?}", unary_op.operator());
        self.visit_expression(unary_op.inner());
        let inner_value = self.ir_code.pop()
            .expect("Did not generate value inside unary op");
        let builder = self.builder;
        let (value, type_) = match unary_op.operator() {
            UnaryOperator::Negation =>
                (builder.build_neg(&inner_value, "negate"),
                Type::double(&self.context)),
            // The unary + operator is always a no-op.
            UnaryOperator::Addition =>
                (inner_value, self.current_type.clone())
        };
        self.current_type = type_;
        self.ir_code.push(value);
    }

    fn visit_binary_op(&mut self, binary_op: &BinaryOperation) {
        trace!("Checking binary operation {:?}", binary_op.operator());
        trace!("Checking {:?} lvalue", binary_op.operator());
        self.visit_expression(binary_op.left());
        let left_register = self.ir_code.pop()
            .expect("Could not generate lvalue of binary op");
        trace!("Checking {:?} rvalue", binary_op.operator());
        self.visit_expression(binary_op.right());
        let right_register = self.ir_code.pop()
            .expect("Could not generate rvalue of binary op");
        let builder = self.builder;
        trace!("Appending binary operation");
        use llvm_sys::LLVMRealPredicate::*;
        let (bin_op_value, bin_op_type) = match binary_op.operator() {
            BinaryOperator::Addition => {
                (builder.build_fadd(&left_register, &right_register, "add"),
                Type::double(&self.context))
            },
            BinaryOperator::Subtraction => {
                (builder.build_fsub(&left_register, &right_register, "sub"),
                Type::double(&self.context))
            },
            BinaryOperator::Multiplication => {
                (builder.build_fmul(&left_register, &right_register, "mul"),
                Type::double(&self.context))
            },
            BinaryOperator::Division => {
                (builder.build_fdiv(&left_register, &right_register, "div"),
                Type::double(&self.context))
            },
            BinaryOperator::Modulus => {
                (builder.build_frem(&left_register, &right_register, "rem"),
                Type::double(&self.context))
            },
            BinaryOperator::Equality => {
                let eq_type_kind = left_register.get_type().get_kind();

                (if eq_type_kind == LLVMTypeKind::LLVMDoubleTypeKind {
                    // Not sure about NaN == NaN here.
                    self.builder.build_fcmp(LLVMRealPredicate::LLVMRealOEQ,
                        &left_register, &right_register, "eq_double")
                }
                else if eq_type_kind == LLVMTypeKind::LLVMIntegerTypeKind {
                    self.builder.build_icmp(LLVMIntPredicate::LLVMIntEQ,
                        &left_register, &right_register, "eq_int")
                }
                else {
                    panic!("Unexpected type for equality check");
                },
                Type::int1(&self.context))
            },
           BinaryOperator::NonEquality => {
                (builder.build_fcmp(LLVMRealONE, &left_register, &right_register, "neqtmp"),
                Type::int1(&self.context))
            },
           BinaryOperator::LessThan => {
                (builder.build_fcmp(LLVMRealOLT, &left_register, &right_register, "lttmp"),
                Type::int1(&self.context))
            },
           BinaryOperator::LessThanEquals => {
                (builder.build_fcmp(LLVMRealOLE, &left_register, &right_register, "letmp"),
                Type::int1(&self.context))
            },
           BinaryOperator::GreaterThan => {
                (builder.build_fcmp(LLVMRealOGT, &left_register, &right_register, "gttmp"),
                Type::int1(&self.context))
            },
           BinaryOperator::GreaterThanEquals => {
                (builder.build_fcmp(LLVMRealOGE, &left_register, &right_register, "getmp"),
                Type::int1(&self.context))
            }
        };
        self.current_type = bin_op_type;
        self.ir_code.push(bin_op_value);
    }

    fn visit_fn_call(&mut self, fn_call: &FnCall) {
        trace!("Checking call to {}", fn_call.text());
        let fn_type = match self.types[&fn_call.id()].clone() {
            ConcreteType::Function(fn_type) => fn_type,
            _other => panic!("Function call's ident had non-fn type")
        };

        trace!("Found function type {:?}", fn_type);

        let mut arg_values = Vec::with_capacity(fn_call.args().len());

        for (_ix, &(ref name, _)) in fn_type.params().iter().enumerate() {
            for arg in fn_call.args() {
                if arg.name().name() == name {
                    self.visit_expression(arg.expression());
                    arg_values.push(self.ir_code.pop()
                        .expect("Could not get alloca for named var of fn arg"));
                    break
                }
            }
        }

        let fn_ref = &self.scope_manager[&fn_call.id()];
        let fn_return_type = self.llvm_type_of_concrete(fn_type.return_ty());
        trace!("Got a function ref to call");
        if fn_return_type.get_kind() == LLVMTypeKind::LLVMVoidTypeKind {
            trace!("Building call void {}", fn_call.text());
            let call = self.builder.build_call(fn_ref, arg_values, "");
            call.set_name("");
        }
        else {
            let name = format!("call_{}", fn_call.text());
            trace!("Building call {}", name);
            let call = self.builder.build_call(fn_ref, arg_values, &name);
            self.ir_code.push(call);
        };
        self.current_type = fn_return_type;
    }

    fn visit_if_expr(&mut self, if_expr: &IfExpression) {
        // Build conditional expr
        self.visit_expression(if_expr.condition());
        let condition_expr = self.ir_code.pop()
            .expect("Did not get value from if conditional");
        // Create basic blocks in the function
        let function = self.builder.insert_block().get_parent()
            .expect("Just now inserted a block");
        let then_block = self.context.append_basic_block(&function, "ife_then");
        let else_block = self.context.append_basic_block(&function, "ife_else");
        let end_block = self.context.append_basic_block(&function, "ife_end");
        // Branch off of the `== 0` comparison
        self.builder.build_cond_br(&condition_expr, &then_block, &else_block);

        // Emit the then code
        self.builder.position_at_end(&then_block);
        self.visit_expression(if_expr.true_expr());
        let then_value = self.ir_code.pop()
            .expect("Did not get IR value from visiting `then` clause of if expression");
        self.builder.build_br(&end_block);
        let then_end_block = self.builder.insert_block();

        // Emit the else code
        self.builder.position_at_end(&else_block);
        self.visit_expression(if_expr.else_expr()); // self.current_type set
        let else_value = self.ir_code.pop()
            .expect("Did not get IR value from visiting `else` clause of if expression");
        self.builder.build_br(&end_block);
        let else_end_block = self.builder.insert_block();

        self.builder.position_at_end(&end_block);

        let phi = self.builder.build_phi(&self.current_type, "ifephi");

        phi.add_incoming(vec![then_value], vec![then_end_block]);
        phi.add_incoming(vec![else_value], vec![else_end_block]);
        self.ir_code.push(phi);
        // self.current_type stays the same.
    }

}
