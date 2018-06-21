//! Bindings to LLVM basic block type.

use std::marker::PhantomData;

use llvm_sys::prelude::*;
use llvm_sys::core::*;

use llvm::Value;

#[derive(Clone)]
pub struct BasicBlock<'ctx> {
    ptr: LLVMBasicBlockRef,
    _lt: PhantomData<&'ctx ()>
}

impl_llvm_ptr_fmt!(<'ctx> BasicBlock);

impl<'ctx> BasicBlock<'ctx> {
    llvm_methods! { BasicBlock<'ctx> => LLVMBasicBlockRef }

    // From Core / BasicBlock

    llvm_passthrough! {
        pub fn as_value() -> Value<'ctx> => LLVMBasicBlockAsValue;
    }

    pub fn from_value(value: &Value<'ctx>) -> BasicBlock<'ctx> {
        unsafe {
            BasicBlock::from_ref(LLVMValueAsBasicBlock(value.ptr()))
        }
    }

    pub fn get_parent(&self) -> Option<Value<'ctx>> {
        let value_ref = unsafe {
            LLVMGetBasicBlockParent(self.ptr())
        };
        if value_ref.is_null() {
            None
        }
        else {
            unsafe { Some(Value::from_ref(value_ref)) }
        }
    }

    pub fn get_terminator(&self) -> Option<Value<'ctx>> {
        let value_ref = unsafe {
            LLVMGetBasicBlockTerminator(self.ptr())
        };
        if value_ref.is_null() {
            None
        }
        else {
            unsafe { Some(Value::from_ref(value_ref)) }
        }
    }
}
