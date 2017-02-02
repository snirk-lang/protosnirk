use std::collections::HashMap;

use parse::{Operator, ASTVisitor, SymbolTable};
use parse::ast::*;
use compile::llvm::{LLVMContext, ModuleProvider, LexicalScopeManager};

use llvm_sys::{self, LLVMOpcode};
use llvm_sys::prelude::*;
use llvm_sys::analysis::LLVMVerifierFailureAction;
use iron_llvm::LLVMRef;
use iron_llvm::core::{Function, Builder};
use iron_llvm::core::value::{RealConstRef, FunctionRef, Value};
use iron_llvm::core::types::{RealTypeRef, FunctionTypeRef, FunctionTypeCtor, RealTypeCtor};
use iron_llvm::core::value::{RealConstCtor, ConstCtor, FunctionCtor};

pub struct ModuleCompiler<M: ModuleProvider> {
    module_provider: M,
    optimizations: bool,
    context: LLVMContext,
    symbols: SymbolTable,
    ir_code: Vec<LLVMValueRef>,
    scope_manager: LexicalScopeManager<LLVMValueRef>
}
impl<M: ModuleProvider> ModuleCompiler<M> {
    pub fn new(symbols: SymbolTable, provider: M, optimizations: bool) -> ModuleCompiler<M> {
        ModuleCompiler {
            module_provider: provider,
            context: LLVMContext::new(),
            symbols: symbols,
            ir_code: Vec::with_capacity(1),
            scope_manager: LexicalScopeManager::new(),
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
        let float_type = RealTypeRef::get_float();
        debug_assert!(!float_type.to_ref().is_null());
        let float_value = literal.get_value();
        let literal_value = RealConstRef::get(&float_type, float_value);
        debug_assert!(!literal_value.to_ref().is_null());
        self.ir_code.push(literal_value.to_ref());
    }

    fn check_var_ref(&mut self, ident_ref: &Identifier) {
        trace!("Checking variable ref {}", ident_ref.get_name());
        let (var_alloca, ix) = self.scope_manager.get(ident_ref.get_name()).expect(
            "Attempted to check var ref but had no alloca");
        let load_name = if ix == 1 {
            format!("load_{}", ident_ref.get_name())
        }
        else {
            format!("load_{}_{}", ident_ref.get_name(), ix - 1)
        };
        let mut builder = self.context.get_ir_builder_mut();
        let var_load = builder.build_load(*var_alloca, &load_name);
        self.ir_code.push(var_load);
    }

    fn check_declaration(&mut self, decl: &Declaration) {
        trace!("Checking declaration for {}", decl.get_name());
        self.check_expression(decl.get_value());
        let decl_value = self.ir_code.pop()
            .expect("Did not have rvalue of declaration");
        let mut builder = self.context.get_ir_builder_mut();
        let float_type = RealTypeRef::get_float();
        let alloca = builder.build_alloca(float_type.to_ref(), decl.get_name());
        self.scope_manager.define_local(decl.get_name().to_string(), alloca.to_ref());
        builder.build_store(decl_value, alloca);
    }

    fn check_assignment(&mut self, assign: &Assignment) {
        trace!("Checking assignment of {}", assign.lvalue.get_name());
        self.check_expression(&*assign.rvalue);
        let rvalue = self.ir_code.pop()
            .expect("Could not generate rvalue of assignment");
        let (var_alloca, _ix) = self.scope_manager.get(assign.lvalue.get_name())
            .expect("Could not find existing var for assignment!");
        let mut builder = self.context.get_ir_builder_mut();
        builder.build_store(rvalue, *var_alloca);
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
        trace!("Checking binary operation {:?}", binary_op.get_operator());
        trace!("Checking {:?} lvalue", binary_op.get_operator());
        self.check_expression(&*binary_op.left);
        let left_register = self.ir_code.pop()
            .expect("Could not generate lvalue of binary op");
        trace!("Checking {:?} rvalue", binary_op.get_operator());
        self.check_expression(&*binary_op.right);
        let right_register = self.ir_code.pop()
            .expect("Could not generate rvalue of binary op");
        let mut builder = self.context.get_ir_builder_mut();
        trace!("Appending binary operation");
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

    fn check_return(&mut self, return_: &Return) {
        trace!("Checking return statement");
        if let Some(ref return_expr) = return_.value {
            self.check_expression(&*return_expr);
            let return_val = self.ir_code.pop()
                .expect("Could not generate value of return");
            let mut builder = self.context.get_ir_builder_mut();
            builder.build_ret(&return_val);
        }
        else {
            warn!("Empty return statement, appending ret void");
            let mut builder = self.context.get_ir_builder_mut();
            // Hopefully doesn't happen, protosnirk doesn't support void types
            builder.build_ret_void();
        }
    }

    fn check_unit(&mut self, unit: &Unit) {
        trace!("Checking unit");
        let fn_ret_double = RealTypeRef::get_float().to_ref();
        let block_fn_type = FunctionTypeRef::get(&fn_ret_double, &mut [], false);
        trace!("Creating `fn main` definition");
        let mut fn_ref = FunctionRef::new(&mut self.module_provider.get_module_mut(),
            "main", &block_fn_type);
        let mut basic_block = fn_ref.append_basic_block_in_context(
            self.context.get_global_context_mut(), "entry");
        self.context.get_ir_builder_mut().position_at_end(&mut basic_block);
        trace!("Positioned IR builder at the end of entry block, checking unit block");
        self.check_block(&unit.block);

        let mut builder = self.context.get_ir_builder_mut();
        // We can auto-issue a return stmt if the ir_code hasn't been
        // consumed. Otherwise, we return 0f64.
        // If the last statement _was_ a return, it's just a redundant
        // return that llvm should optimize out.
        // This will also need to be fixed when allowing nested blocks.
        // This will be redone in the parsing stages.
        if let Some(remaining_expr) = self.ir_code.pop() {
            trace!("Found final expression, appending a return");
            builder.build_ret(&remaining_expr);
        }

        // Returns true if verification failed
        assert!(!fn_ref.verify(LLVMVerifierFailureAction::LLVMPrintMessageAction));
        if self.optimizations {
            trace!("Running optimizations");
            self.module_provider.get_pass_manager().run(&mut fn_ref);
        }
        // The final ir_code value should be a reference to the function
        self.module_provider.get_module()
            .verify(LLVMVerifierFailureAction::LLVMPrintMessageAction)
            .unwrap();
    }

    fn check_block(&mut self, block: &Block) {
        trace!("Checking block");
        self.scope_manager.new_scope();
        for stmt in &block.statements {
            self.check_statement(stmt)
        }
        self.scope_manager.pop();
    }
}
