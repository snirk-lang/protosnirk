use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

use compile::llvm::ModuleProvider;
use super::llvm_state::LLVMState;

use llvm_sys::prelude::LLVMValueRef;
use iron_llvm::{LLVMRefCtor};
use iron_llvm::core::{Module, FunctionPassManager};
use iron_llvm::core::value::{FunctionRef};
use iron_llvm::core::types::{RealTypeRef, RealTypeCtor};
use iron_llvm::execution_engine::execution_engine::MCJITBuilder;
use iron_llvm::execution_engine::memory_manager::BindingSectionMemoryManagerBuilder;

pub trait LLVMJIT : ModuleProvider {
    fn run_function(&mut self, func: LLVMValueRef) -> f64;
}

fn default_pass_manager(module: &Module, optimize: bool) -> FunctionPassManager {
    let mut pass_manager = FunctionPassManager::new(module);
    pass_manager.add_basic_alias_analysis_pass();
    pass_manager.add_CFG_simplification_pass();
    if optimize {
        pass_manager.add_instruction_combining_pass();
        pass_manager.add_reassociate_pass();
        pass_manager.add_GVN_pass();
    }
    pass_manager.initialize();
    pass_manager
}

pub struct MCJIT {
    current_module_name: String,
    current_module: Module,
    pass_manager: FunctionPassManager,
    optimization: bool,

    shared_state: Rc<RefCell<LLVMState>>
}

impl MCJIT {
    pub fn new(name: String, optimization: bool) -> MCJIT {
        let module = Module::new(&name);
        let pass_manager = default_pass_manager(&module, optimization);

        MCJIT {
            current_module_name: name,
            current_module: module,
            optimization: optimization,
            pass_manager: pass_manager,
            shared_state: Rc::new(RefCell::new(LLVMState::new()))
        }
    }

    /// Close the current module and create a new one.
    ///
    /// iron_llvm tutorial dictates creating a new module per
    /// function written to the command line.
    fn close_current_module(&mut self) {
        let new_module = Module::new(&self.current_module_name);
        let mut pass_manager = FunctionPassManager::new(&new_module);
        pass_manager.add_basic_alias_analysis_pass();
        pass_manager.add_instruction_combining_pass();
        pass_manager.add_reassociate_pass();
        // TODO incorporate optimization levels
        pass_manager.add_GVN_pass();
        pass_manager.add_CFG_simplification_pass();
        pass_manager.initialize();
        self.pass_manager = pass_manager;
        let current_module = mem::replace(&mut self.current_module, new_module);

        let container = self.shared_state.clone();
        let memory_manager = BindingSectionMemoryManagerBuilder::new()
            .set_get_symbol_address(move |mut parent_state, name| {
                let addr = parent_state.get_symbol_address(name);
                if addr != 0 {
                    return addr;
                }

                container.borrow().get_fn_address(name)
            })
            .create();

        let (execution_engine, module) = match MCJITBuilder::new()
            .set_mcjit_memory_manager(Box::new(memory_manager))
            .create(current_module) {
                Ok((ee, module)) => (ee, module),
                Err(msg) => panic!(msg)
            };

        self.shared_state.borrow_mut().execution_engines.push(execution_engine);
        self.shared_state.borrow_mut().modules.push(module);
    }
}
impl ModuleProvider for MCJIT {
    fn get_module(&self) -> &Module {
        &self.current_module
    }
    fn get_module_mut(&mut self) -> &mut Module {
        &mut self.current_module
    }
    fn get_pass_manager(&mut self) -> &mut FunctionPassManager {
        &mut self.pass_manager
    }
}
impl LLVMJIT for MCJIT {
    fn run_function(&mut self, function: LLVMValueRef) -> f64 {
        self.close_current_module();
        let function_ref = unsafe { FunctionRef::from_ref(function) };
        let mut args = vec![];
        let res = self.shared_state.borrow()
            .execution_engines.last()
            .expect("MCJIT did not have JITs")
            .run_function(&function_ref, args.as_mut_slice());
        let ty = RealTypeRef::get_double();
        res.to_float(&ty)
    }
}
