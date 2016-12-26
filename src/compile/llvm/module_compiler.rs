use std::collections::HashMap;

use parse::{Operator, ExpressionChecker, SymbolTable};
use parse::expression::*;
use compile::llvm::{LLVMContext, ModuleProvider};

use llvm_sys::prelude::*;
use llvm_sys::LLVMOpcode;
use llvm_sys::analysis::LLVMVerifierFailureAction;
use iron_llvm::LLVMRef;
use iron_llvm::core::Function;
use iron_llvm::core::value::{RealConstRef, FunctionRef, Value};
use iron_llvm::core::types::{RealTypeRef, FunctionTypeRef, FunctionTypeCtor, RealTypeCtor};
use iron_llvm::core::value::{RealConstCtor, ConstCtor, FunctionCtor};

pub struct ModuleCompiler<M: ModuleProvider> {
    module_provider: M,
    context: LLVMContext,
    symbols: SymbolTable,
    ir_code: Vec<LLVMValueRef>,
    scopes: Vec<HashMap<String, LLVMValueRef>>,
}
impl<M: ModuleProvider> ModuleCompiler<M> {
    pub fn new(symbols: SymbolTable, provider: M) -> ModuleCompiler<M> {
        ModuleCompiler {
            module_provider: provider,
            context: LLVMContext::new(),
            symbols: symbols,
            ir_code: Vec::with_capacity(1),
            scopes: vec![HashMap::new()],
        }
    }
    pub fn decompose(self) -> (M, LLVMContext, SymbolTable) {
        (self.module_provider, self.context, self.symbols)
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new())
    }
    fn pop_scope(&mut self) {
        self.scopes.pop();
    }
    fn get_named_var(&self, name: &str) -> LLVMValueRef {
        for scope in self.scopes.iter().rev() {
            if let Some(matched) = scope.get(name) {
                return *matched
            }
        }
        panic!("Could not find {} in any scops", name)
    }
    fn push_local_var_def(&mut self, name: String) -> LLVMValueRef {
        debug_assert!(!self.scopes.is_empty(),
            "Didn't have a local scope to define a new variable");
        let last_ix = self.scopes.len() -1;
        let mut local_scope = &mut self.scopes[last_ix];
        let computed_value = self.ir_code.pop();
        let var_value = match computed_value {
            Some(val) => val,
            None => {
                // We don't allow uninitialized variables,
                // might need some static analysis there.
                RealConstRef::get_undef(&RealTypeRef::get_float()).to_ref()
            }
        };
        local_scope.insert(name, var_value.to_ref()); // whee cloning
        return var_value
    }
}
impl<M:ModuleProvider> ExpressionChecker for ModuleCompiler<M> {
    fn check_literal(&mut self, literal: &Literal) {
        let float_type = RealTypeRef::get_float();
        let float_value = literal.get_value();
        let literal_value = RealConstRef::get(&float_type, float_value);
        self.ir_code.push(literal_value.to_ref());
    }

    fn check_var_ref(&mut self, var_ref: &Identifier) {
        let var_ref = self.get_named_var(var_ref.get_name()).to_ref();
        let mut builder = self.context.get_ir_builder_mut();
        let load_name = format!("load_{}", var_ref.get_name());
        let ref_value = builder.build_load(var_ref, &load_name);
        self.ir_code.push(ref_value);
    }

    fn check_declaration(&mut self, decl: &Declaration) {
        self.check_expression(decl.get_value());
        let decl_value = self.ir_code.pop()
            .expect("Did not generate value from declaration");
        self.push_local_var_def(decl.get_name().into());
        let local_name = format!("local_{}", decl.get_name());
        let decl_type = RealTypeRef::get_float().to_ref();
        let mut builder = self.context.get_ir_builder_mut();
        let alloca = builder.build_alloca(decl_type, &local_name);
        let _ref_stored_value = builder.build_store(decl_value, alloca);
    }

    fn check_unary_op(&mut self, unary_op: &UnaryOperation) {
        debug_assert!(unary_op.operator == Operator::Subtraction,
            "Invalid unary operator {:?}", unary_op.operator);
        self.check_expression(&*unary_op.expression);
        let inner_value = self.ir_code.pop()
            .expect("Did not generate value inside unary op");
        let mut builder = self.context.get_ir_builder_mut();
        let ref_value = builder.build_neg(inner_value, "negate");
        self.ir_code.push(ref_value);
    }

    fn check_binary_op(&mut self, binary_op: &BinaryOperation) {
        self.check_expression(&*binary_op.left);
        let left_register = self.ir_code.pop()
            .expect("Could not generate lvalue of binary op");
        self.check_expression(&*binary_op.right);
        let right_register = self.ir_code.pop()
            .expect("Could not generate rvalue of binary op");
        let mut builder = self.context.get_ir_builder_mut();
        let bin_op_value = match binary_op.get_operator() {
            Operator::Addition =>
                builder.build_fadd(left_register, right_register, "add"),
            Operator::Subtraction =>
                builder.build_fsub(left_register, right_register, "sub"),
            Operator::Multiplication =>
                builder.build_fmul(left_register, right_register, "mul"),
            Operator::Division =>
                // build_fdiv is missing...
                // this code needs to be redone (get llvm op from op, etc.)
                // and also deal with casting/etc.
                builder.build_binop(LLVMOpcode::LLVMFDiv, left_register, right_register, "div"),
            Operator::Modulus => {
                builder.build_frem(left_register, right_register, "rem")
            },
            Operator::Custom => panic!("Cannot handle custom operator")
        };
        self.ir_code.push(bin_op_value);
    }

    fn check_assignment(&mut self, assign: &Assignment) {
        self.check_expression(&*assign.rvalue);
        let rvalue = self.ir_code.pop()
            .expect("Could not generate rvalue of assignment");
        let var_alloca = self.get_named_var(assign.lvalue.get_name());
        let mut builder = self.context.get_ir_builder_mut();
        let _assigned_var = builder.build_store(var_alloca, rvalue);
    }

    fn check_return(&mut self, return_: &Return) {
        if let Some(ref return_expr) = return_.value {
            self.check_expression(&*return_expr);
            let return_val = self.ir_code.pop()
                .expect("Could not generate value of return");
            let mut builder = self.context.get_ir_builder_mut();
            builder.build_ret(&return_val);
        }
        else {
            let mut builder = self.context.get_ir_builder_mut();
            // Hopefully doesn't happen, protosnirk doesn't support void types
            builder.build_ret_void();
        }
    }

    fn check_block(&mut self, block: &Vec<Expression>) {

        let fn_ret_double = RealTypeRef::get_float().to_ref();
        let block_fn_type = FunctionTypeRef::get(&fn_ret_double, &mut [], false);
        let mut fn_ref = FunctionRef::new(&mut self.module_provider.get_module_mut(), "main", &block_fn_type);
        let mut basic_block = fn_ref.append_basic_block_in_context(self.context.get_global_context_mut(), "entry");
        self.context.get_ir_builder_mut().position_at_end(&mut basic_block);

        self.scopes.push(HashMap::new());
        for expr in block {
            self.check_expression(expr)
        }
        self.scopes.pop();

        let mut builder = self.context.get_ir_builder_mut();
        // We can auto-issue a return stmt if the ir_code hasn't been
        // consumed. Otherwise, we return 0f64.
        // If the last statement _was_ a return, it's just a redundant
        // return that llvm should optimize out.
        // This will also need to be fixed when allowing nested blocks.
        if let Some(remaining_expr) = self.ir_code.pop() {
            builder.build_ret(&remaining_expr);
        }
        else {
            builder.build_ret_void();
        }

        fn_ref.verify(LLVMVerifierFailureAction::LLVMAbortProcessAction);
        self.module_provider.get_pass_manager().run(&mut fn_ref);
        // The final ir_code value should be a reference tothe function
        self.ir_code.push(fn_ref.to_ref());
    }
}
