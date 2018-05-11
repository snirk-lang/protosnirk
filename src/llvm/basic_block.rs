//! Bindings to LLVM basic block type.

use llvm_sys::prelude::*;
use llvm_sys::core::*;

use llvm::Value;

llvm_wrapped! {
    pub struct BasicBlock {
        value: LLVMBasicBlockRef,
        dispose: drop
    }
}

impl BasicBlock {
    pub fn as_value(&self) -> &Value {
        let value_ref = unsafe {
            LLVMBasicBlockAsValue(**self)
        };
        &Value::from_ref(value_ref)
    }

    pub fn get_parent(&self) -> &Value {
        let value_ref = unsafe {
            LLVMGetBasicBlockParent(**self)
        };
        &Value::from_ref(value_ref)
    }
}
