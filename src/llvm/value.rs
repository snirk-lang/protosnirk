//! Bindings to LLVM value objects

use llvm_sys::core::*;
use llvm_sys::prelude::*;

llvm_wrapped! {
    pub struct Value {
        value: LLVMValueRef,
        dispose: drop // Value is entirely owned by Context
    }
}
