//! Bindings to LLVM pass manager objects

use llvm::{Module, Value};

use llvm_sys::prelude::*;
use llvm_sys::core::*;

llvm_wrapped! {
    pub struct PassManager {
        inner: PassManagerInner,
        value: LLVMPassManagerRef,
        dispose: LLVMDisposePassManager
    }
}

impl PassManager {
    pub fn new() -> PassManager {
        let pass_ref = unsafe {
            LLVMCreatePassManager()
        };
        PassManager::from_ref(pass_ref)
    }

    pub fn run(&self, module: &Module) -> bool {
        unsafe {
            LLVMRunPassManager(self.get(), module.get())
        }
    }
}


llvm_wrapped! {
    pub struct FunctionPassManager {
        inner: FnPassManagerInner,
        value: LLVMPassManagerRef,
        dispose: LLVMFinalizeFunctionPassManager
    }
}

impl FunctionPassManager {
    pub fn new(module: &Module) -> FunctionPassManager {
        let pass_ref = unsafe {
            LLVMCreateFunctionPassManager(module.get())
        };
        FunctionPassManager::from_ref(pass_ref)
    }

    pub fn run(f: &Value) -> bool {
        unsafe {
            LLVMRunFunctionPassManager(self.get(), f.get())
        }
    }
}
