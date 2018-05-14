use std::collections::{HashMap, BTreeMap};

use parse::{ASTVisitor, ScopeIndex, SymbolTable};
use parse::ast::*;
use compile::{LLVMContext, ModuleProvider};

use llvm_sys::{self, LLVMOpcode, LLVMRealPredicate};
use llvm_sys::prelude::*;
use llvm_sys::analysis::LLVMVerifierFailureAction;

use llvm::{Value, Type};

pub struct ModuleCompiler<M: ModuleProvider> {
    module_provider: M,
    optimizations: bool,
    context: LLVMContext,
    ir_code: Vec<Value>,
    symbols: SymbolTable,
    scope_manager: HashMap<ScopeIndex, Value>
}
impl<M: ModuleProvider> ModuleCompiler<M> {
    pub fn new(symbols: SymbolTable, provider: M, optimizations: bool) -> ModuleCompiler<M> {
        ModuleCompiler {
            module_provider: provider,
            context: LLVMContext::new(),
            symbols: symbols,
            ir_code: Vec::with_capacity(1),
            scope_manager: HashMap::new(),
            optimizations: optimizations
        }
    }
    pub fn decompose(self) -> (M, LLVMContext, SymbolTable) {
        (self.module_provider, self.context, self.symbols)
    }
}
impl<M:ModuleProvider> ASTVisitor for ModuleCompiler<M> {
    fn check_literal(&mut self, literal: &Literal) {
        trace!("Checking literal {}", literal.token);
        let float_value = literal.get_value();
        let literal_value = Type::double().const_real(float_value as f64);
        self.ir_code.push(literal_value);
    }

    fn check_var_ref(&mut self, ident_ref: &Identifier) {
        trace!("Checking variable ref {}", ident_ref.get_name());
        let var_alloca = self.scope_manager.get(&ident_ref.get_index())
            .expect("Attempted to check var ref but had no alloca");
        let load_name = format!("load_{}", ident_ref.get_name());
        let mut builder = self.context.builder_mut();
        let var_load = builder.build_load(*var_alloca, &load_name);
        self.ir_code.push(var_load);
    }

    fn check_declaration(&mut self, decl: &Declaration) {
        trace!("Checking declaration for {}", decl.get_name());
        self.check_expression(decl.get_value());
        let decl_value = self.ir_code.pop()
            .expect("Did not have rvalue of declaration");
        let mut builder = self.context.builder_mut();
        let float_type = Type::double();
        let alloca = builder.build_alloca(float_type, decl.get_name());
        self.scope_manager.insert(decl.ident.get_index(), alloca);
        builder.build_store(decl_value, alloca);
    }

    fn check_assignment(&mut self, assign: &Assignment) {
        trace!("Checking assignment of {}", assign.lvalue.get_name());
        self.check_expression(&*assign.rvalue);
        let rvalue = self.ir_code.pop()
            .expect("Could not generate rvalue of assignment");
        let var_alloca = self.scope_manager.get(&assign.lvalue.get_index())
            .expect("Could not find existing var for assignment!");
        let mut builder = self.context.builder_mut();
        builder.build_store(rvalue, *var_alloca);
    }

    fn check_unary_op(&mut self, unary_op: &UnaryOperation) {
        debug_assert!(unary_op.operator == Operator::Subtraction,
            "Invalid unary operator {:?}", unary_op.operator);
        self.check_expression(&*unary_op.expression);
        let inner_value = self.ir_code.pop()
            .expect("Did not generate value inside unary op");
        let mut builder = self.context.builder_mut();
        let value = match unary_op.operator {
            Operator::Subtraction =>
                builder.build_neg(inner_value, "negate"),
            other => panic!("Invalid unary operator {:?}", other)
        };
        self.ir_code.push(value);
    }

    fn check_binary_op(&mut self, binary_op: &BinaryOperation) {
        trace!("Checking binary operation {:?}", binary_op.get_operator());
        trace!("Checking {:?} lvalue", binary_op.get_operator());
        self.check_expression(&*binary_op.left);
        let left_register = self.ir_code.pop()
            .expect("Could not generate lvalue of binary op");
        trace!("Checking {:?} rvalue", binary_op.get_operator());
        self.check_expression(&*binary_op.right);
        let right_register = self.ir_code.pop()
            .expect("Could not generate rvalue of binary op");
        let mut builder = self.context.builder_mut();
        trace!("Appending binary operation");
        use llvm_sys::LLVMRealPredicate::*;
        let bin_op_value = match binary_op.get_operator() {
            Operator::Addition =>
                builder.build_fadd(left_register, right_register, "add"),
            Operator::Subtraction =>
                builder.build_fsub(left_register, right_register, "sub"),
            Operator::Multiplication =>
                builder.build_fmul(left_register, right_register, "mul"),
            Operator::Division =>
                builder.build_binop(LLVMOpcode::LLVMFDiv, left_register, right_register, "div"),
            Operator::Modulus =>
                builder.build_frem(left_register, right_register, "rem"),
            // TODO binary operations should be handled seperately
            // when types are added
            Operator::Equality => {
                let eq = builder.build_fcmp(LLVMRealOEQ, left_register, right_register, "eqtmp");
                builder.build_ui_to_fp(eq, Type::double(), "eqcast")
            },
            Operator::NonEquality => {
                let neq = builder.build_fcmp(LLVMRealONE, left_register, right_register, "neqtmp");
                builder.build_ui_to_fp(neq, Type::double(), "neqcast")
            },
            Operator::LessThan => {
                let lt = builder.build_fcmp(LLVMRealOLT, left_register, right_register, "lttmp");
                builder.build_ui_to_fp(lt, Type::double(), "ltcast")
            },
            Operator::LessThanEquals => {
                let le = builder.build_fcmp(LLVMRealOLE, left_register, right_register, "letmp");
                builder.build_ui_to_fp(le, Type::double(), "lecast")
            },
            Operator::GreaterThan => {
                let gt = builder.build_fcmp(LLVMRealOGT, left_register, right_register, "gttmp");
                builder.build_ui_to_fp(gt, Type::double(), "gtcast")
            },
            Operator::GreaterThanEquals => {
                let ge = builder.build_fcmp(LLVMRealOGE, left_register, right_register, "getmp");
                builder.build_ui_to_fp(ge, Type::double(), "gecast")
            }
            Operator::Custom => panic!("Cannot handle custom operator")
        };
        self.ir_code.push(bin_op_value);
    }

    fn check_fn_call(&mut self, fn_call: &FnCall) {
        trace!("Checking call to {}", fn_call.get_text());
        let mut arg_map = BTreeMap::new();
        let fn_type = self.symbols[&fn_call.get_name().get_index()]
                        .get_type()
                        .clone()
                        .expect_fn();
        trace!("Found function type {:?}", fn_type);

        match *fn_call.get_args() {
            FnCallArgs::SingleExpr(ref inner) => {
                self.check_expression(inner);
                let arg_val = self.ir_code.pop()
                    .expect("Could not generate value of function arg");
                trace!("Insearting default value at index 0");
                arg_map.insert(0usize, arg_val);
            },
            FnCallArgs::Arguments(ref args) => {
                // TODO just use a hashmap in fncall
                // Also it's important to emut code in the order that
                // the arguments are given to the function rather than
                // sort the arguments by how the callee does.
                for arg in args {
                    let (ix, _declared_type) = fn_type.get_arg(arg.get_text())
                        .expect("Function arg check did not pass");
                    // TODO type check the param types
                    // No value so must be a ref
                    if !arg.has_value() {
                        self.check_var_ref(arg.get_name());
                        let arg_ref = self.ir_code.pop()
                            .expect("Could not get alloca for implicit var for fn arg");
                        arg_map.insert(ix, arg_ref);
                    }
                    else {
                        self.check_expression(arg.get_expr().expect("Checked expect"));
                        let value_ref = self.ir_code.pop()
                            .expect("Could not get alloca for named var of fn arg");
                        arg_map.insert(ix, value_ref);
                    }
                }
            }
        }
        trace!("Preparing arguments: {:?}", arg_map);
        let mut arg_values: Vec<Value> =
            Vec::with_capacity(fn_call.get_args().len());
        for (ix, value) in arg_map.into_iter() {
            trace!("Pushing arg {}: {:?}", ix, value);
            arg_values.push(value);
            trace!("Arg {} pushed", ix);
        }
        trace!("Finished pushing args");
        debug_assert_eq!(arg_values.len(), fn_type.get_args().len());
        let name = format!("call_{}", fn_call.get_text());
        trace!("Scope manager: {:?}", self.scope_manager);
        trace!("Fn call index: {:?}", fn_call.get_name().get_index());
        let fn_ref = self.scope_manager[&fn_call.get_name().get_index()];
        trace!("Got a function ref to call");
        let call = self.context.builder_mut()
                               .build_call(fn_ref, arg_values, &name);
        self.ir_code.push(call);
    }

    fn check_return(&mut self, return_: &Return) {
        trace!("Checking return statement");
        if let Some(ref return_expr) = return_.value {
            self.check_expression(&*return_expr);
            let return_val = self.ir_code.pop()
                .expect("Could not generate value of return");
            let mut builder = self.context.builder_mut();
            builder.build_ret(return_val);
        }
        else {
            warn!("Empty return statement, appending ret void");
            let mut builder = self.context.builder_mut();
            // Hopefully doesn't happen, protosnirk doesn't support void types
            builder.build_ret_void();
        }
    }

    fn check_fn_declaration(&mut self, fn_declaration: &FnDeclaration) {
        trace!("Checking declaration of {}", fn_declaration.get_name().get_name());

        let float_type = Type::double();
        let mut arg_types = vec![float_type; fn_declaration.get_args().len()];
        let fn_type = Type::function(float_type, arg_types, false);
        let mut fn_ref = self.module_provider.get_module().add_function(
            fn_declaration.get_name().get_name(), fn_type);

        // Gotta insert the fn ref first so it can be called recursively
        self.scope_manager.insert(fn_declaration.get_name().get_index(), fn_ref);
        trace!("Inserted {} into the scope manager: {:?}",
            fn_declaration.get_name().get_name(), self.scope_manager);

        // Gonna be fancy and have a separate basic block for parameters
        let mut entry_block = self.context.global_context().append_basic_block(&fn_ref, "entry");
        let mut start_block = self.context.global_context().append_basic_block(&fn_ref, "start");
        self.context.builder_mut().position_at_end(&mut entry_block);
        trace!("Ready to build {}", fn_declaration.get_name().get_name());

        // Rename args to %argname, create+remember allocas and store the function values there.
        // This allows LLVM to mutate function params even if we don't allow it right now.
        for (ir_param, ast_param) in fn_ref.get_params().iter().zip(fn_declaration.get_args()) {
            trace!("Adding fn param {} (ix {:?})", ast_param.get_name(), ast_param.get_index());
            ir_param.set_name(ast_param.get_name());
            let alloca = self.context.builder_mut().build_alloca(float_type, ast_param.get_name());
            self.scope_manager.insert(ast_param.get_index(), alloca);
            self.context.builder_mut().build_store(*ir_param, alloca);
        }
        self.context.builder_mut().build_br(&mut start_block);
        self.context.builder_mut().position_at_end(&mut start_block);

        // Compile the function
        self.check_block(&fn_declaration.get_block());

        if let Some(remaining_expr) = self.ir_code.pop() {
            trace!("Found final expression, appending a return");
            self.context.builder_mut().build_ret(remaining_expr);
            //self.module_provider.get_module().dump();
        }

        assert!(!fn_ref.verify(LLVMVerifierFailureAction::LLVMPrintMessageAction));
        if self.optimizations {
            trace!("Running optimizations on a function");
            self.module_provider.get_pass_manager().run(&mut fn_ref);
        }
    }

    fn check_unit(&mut self, unit: &Unit) {
        trace!("Checking a unit");

        for fn_declaration in unit.get_items() {
            self.check_item(fn_declaration);
        }

        // The final ir_code value should be a reference to the function
        self.module_provider.get_module()
            .verify(LLVMVerifierFailureAction::LLVMPrintMessageAction)
            .unwrap();
    }

    fn check_if_expr(&mut self, if_expr: &IfExpression) {
        // Build conditional expr
        self.check_expression(if_expr.get_condition());
        let condition_expr = self.ir_code.pop()
            .expect("Did not get value from if conditional");
        let const_zero = Type::double().const_real(0f64);
        // hack: compare it to 0, due to lack of booleans right now
        let condition = self.context.builder_mut()
            .build_fcmp(LLVMRealPredicate::LLVMRealOEQ, condition_expr, const_zero, "ife_cond");
        // Create basic blocks in the function
        let mut function = self.context.builder().get_insert_block().get_parent();
        let then_block = self.context.global_context().append_basic_block(function, "ife_then");
        let else_block = self.context.global_context().append_basic_block(function, "ife_else");
        let mut end_block = self.context.global_context().append_basic_block(function, "ife_end");
        // Branch off of the `== 0` comparison
        self.context.builder_mut().build_cond_br(condition, &then_block, &else_block);

        // Emit the then code
        self.context.builder_mut().position_at_end(&mut then_block);
        self.check_expression(if_expr.get_true_expr());
        let then_value = self.ir_code.pop()
            .expect("Did not get IR value from visiting `then` clause of if expression");
        self.context.builder_mut().build_br(&end_block);
        let then_end_block = self.context.builder_mut().get_insert_block();

        // Emit the else code
        self.context.builder_mut().position_at_end(&mut else_block);
        self.check_expression(if_expr.get_else());
        let else_value = self.ir_code.pop()
            .expect("Did not get IR value from visiting `else` clause of if expression");
        self.context.builder_mut().build_br(&end_block);
        let else_end_block = self.context.builder_mut().get_insert_block();

        self.context.builder_mut().position_at_end(&mut end_block);
        let phi = self.context.builder().build_phi(Type::double(), "ifephi");

        phi.add_incoming(vec![then_value], vec![then_end_block]);
        phi.add_incoming(vec![else_value], vec![else_end_block]);
        self.ir_code.push(phi);
    }

    fn check_if_block(&mut self, if_block: &IfBlock) {
        trace!("Checking if block: has_value={}", if_block.has_value());
        // Create some lists of values to use later
        let condition_count = if_block.get_conditionals().len();
        let valued_if = if_block.has_value();
        let mut function = self.context.builder().get_insert_block().get_parent();

        let mut condition_blocks = Vec::with_capacity(condition_count);
        let mut incoming_values =
            Vec::with_capacity(if if_block.has_value() { condition_count} else {0});

        trace!("Preparing to emit {} conditionals", condition_count);
        // Populate a list of the future blocks to have
        for (ix, _conditional) in if_block.get_conditionals().iter().enumerate() {
            trace!("Creating condition block {}", ix);
            if ix != 0usize {
                let name = format!("if_{}_cond", ix + 1);
                condition_blocks.push(
                    self.context.global_context_mut().append_basic_block(function, &name)
                )
            }
            let name = format!("if_{}_then", ix + 1);
            condition_blocks.push(
                self.context.global_context_mut().append_basic_block(function, &name)
            );
        }
        // If there's an else it needs a block
        if if_block.has_else() {
            trace!("Creating else block");
            condition_blocks.push(
                self.context.global_context().append_basic_block(function, "else_block")
            );
        }

        let const_zero = Type::double().const_real(0.0);

        trace!("Creating end block");
        condition_blocks.push(self.context.global_context().append_basic_block(function,
                                                                     "if_end"));

        let mut ix = 0;
        for conditional in if_block.get_conditionals() {
            trace!("Checking expr for condition {}", ix);
            self.check_expression(conditional.get_condition());
            let cond_value = self.ir_code.pop()
                .expect("Did not get IR value from if block condition");
            let cond_cmp_name = format!("if_{}_cmp", ix);
            let cond_cmp = self.context.builder_mut()
                .build_fcmp(LLVMRealPredicate::LLVMRealOEQ, cond_value, const_zero, &cond_cmp_name);

            trace!("Building a break to next blocks {} -> {}, {}", cond_cmp_name, ix, ix + 1);
            self.context.builder_mut().build_cond_br(cond_cmp,
                                                     &condition_blocks[ix],
                                                     &condition_blocks[ix + 1]);

            // Go to the `if_true` block of this conditional
            trace!("Positioning at end of cond block {}", ix);
            self.context.builder_mut().position_at_end(&mut condition_blocks[ix]);
            trace!("Checking conditional block");
            self.check_block(conditional.get_block());
            // If this is a valued if, save the value ref for this branch of the condition
            if valued_if {
                let value = self.ir_code.pop()
                    .expect("Did not get value from valued if block");
                incoming_values.push(value);
            }

            // After block, go to done
            trace!("Adding branch to cond end block");
            let last_ix = condition_blocks.len() - 1;
            self.context.builder_mut().build_br(&mut condition_blocks[last_ix]);

            // Position at the beginning of the next block
            trace!("Moving onto block {}", ix + 1);
            self.context.builder_mut().position_at_end(&mut condition_blocks[ix + 1]);
            ix += 2;
        }

        trace!("Finished checking conditions");
        // If there's an else, check that too
        if let Some(&(_, ref else_block)) = if_block.get_else() {
            trace!("Checking else block");
            self.check_block(else_block);
            if valued_if {
                let value = self.ir_code.pop()
                    .expect("Did not get value from else of valued if block");
                incoming_values.push(value);
            }
            // Branch to end after else
            let last_ix = condition_blocks.len() - 1;
            self.context.builder_mut().build_br(&mut condition_blocks[last_ix]);
        }

        // Remove the end block from condition blocks for borrowck + phi reasons
        let mut cond_end_block = condition_blocks.pop()
            .expect("Somehow there were 0 conditional blocks");
        // Position at end block - this lets us get on with the function
        self.context.builder_mut().position_at_end(&mut cond_end_block);

        // If we need to push a value, create a phi
        if valued_if {
            let mut incoming_conditions = condition_blocks
                .chunks(2)
                .map(|c| c[0])
                .collect::<Vec<_>>();
            incoming_conditions.push(*condition_blocks.last().expect("No condition blocks"));
            self.module_provider.get_module().dump();
            trace!("Generating phi node with {} values and {} edges",
                incoming_values.len(), incoming_conditions.len());
            let phi = self.context.builder().build_phi(Type::double(), "if_phi");
            phi.add_incoming(incoming_values, incoming_conditions);
            self.ir_code.push(phi);
        }
    }

    fn check_block(&mut self, block: &Block) {
        trace!("Checking block");
        for stmt in block.statements.iter() {
            self.check_statement(stmt)
        }
    }
}
