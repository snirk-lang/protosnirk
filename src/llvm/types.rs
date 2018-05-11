//! LLVM Type object.

use llvm_sys::core::*;
use llvm_sys::prelude::*;

llvm_wrapped! {
    pub struct Type {
        value: LLVMTypeRef,
        dispose: drop // Type is owned by the surrounding context
    }
}
