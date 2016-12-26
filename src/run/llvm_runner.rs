use compile::llvm::ModuleProvider;

use llvm_sys::prelude::LLVMValueRef;

pub trait LLVMJIT : ModuleProvider {
    fn run_function(&mut self, func: LLVMValueRef) -> f64;
}
