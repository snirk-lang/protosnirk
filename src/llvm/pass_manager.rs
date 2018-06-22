//! Bindings to LLVM pass manager objects

use llvm_sys::prelude::*;
use llvm_sys::core::*;
use llvm_sys::transforms::scalar::*;

use llvm::{Module, Value};

pub struct PassManager {
    ptr: LLVMPassManagerRef
}

impl_llvm_ptr_fmt!(PassManager);

impl Drop for PassManager {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposePassManager(self.ptr())
        }
    }
}

impl PassManager {
    pub unsafe fn from_ref(ptr: LLVMPassManagerRef) -> PassManager {
        PassManager { ptr }
    }

    pub fn ptr(&self) -> LLVMPassManagerRef {
        self.ptr
    }

    pub fn new() -> PassManager {
        unsafe {
            PassManager::from_ref(LLVMCreatePassManager())
        }
    }

    pub fn run<'ctx>(&self, module: &Module<'ctx>) -> bool {
        unsafe {
            LLVMRunPassManager(self.ptr(), module.ptr()) > 0
        }
    }
}

macro_rules! pass_methods {
    (impl $struct_name:ident { $(pub fn $wrapped:ident = $name:ident;)+ }) => {
        impl $struct_name {
            $(
                pub fn $wrapped(&self) {
                    unsafe {
                        $name(self.ptr());
                    }
                }
            )+
        }
    }
}

pub struct FunctionPassManager {
    ptr: LLVMPassManagerRef,
}

impl_llvm_ptr_fmt!(FunctionPassManager);

impl Drop for FunctionPassManager {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposePassManager(self.ptr())
        }
    }
}

impl FunctionPassManager {
    pub unsafe fn from_ref(ptr: LLVMPassManagerRef) -> FunctionPassManager {
        FunctionPassManager { ptr }
    }

    pub fn ptr(&self) -> LLVMPassManagerRef {
        self.ptr
    }

    pub fn new<'ctx>(module: &Module<'ctx>) -> FunctionPassManager {
        unsafe {
            FunctionPassManager::from_ref(
                LLVMCreateFunctionPassManagerForModule(module.ptr()))
        }
    }

    pub fn initialize(&self) -> bool {
        unsafe {
            LLVMInitializeFunctionPassManager(self.ptr()) > 0
        }
    }

    pub fn run<'ctx>(&self, func: &Value<'ctx>) -> bool {
        unsafe {
            LLVMRunFunctionPassManager(self.ptr(), func.ptr()) > 0
        }
    }
}

pass_methods! {
    impl FunctionPassManager {
        pub fn add_instruction_combining_pass = LLVMAddInstructionCombiningPass;
        pub fn add_gvn_pass = LLVMAddGVNPass;
        pub fn add_cfg_simplification_pass = LLVMAddCFGSimplificationPass;
        pub fn add_basic_alias_analysis_pass = LLVMAddBasicAliasAnalysisPass;
        pub fn add_reassociate_pass = LLVMAddReassociatePass;
    }
}
