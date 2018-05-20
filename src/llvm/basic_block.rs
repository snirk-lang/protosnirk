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

impl<'ctx> BasicBlock<'ctx> {
    llvm_methods! { BasicBlock<'ctx> => LLVMBasicBlockRef }

    // From Core / BasicBlock

    pub fn as_value(&self) -> Value<'ctx> {
        unsafe {
            Value::from_ref(LLVMBasicBlockAsValue(self.ptr()))
        }
    }

    pub fn from_value(value: &Value<'ctx>) -> BasicBlock<'ctx> {
        unsafe {
            BasicBlock::from_ref(LLVMValueAsBasicBlock(value.ptr()))
        }
    }

    pub fn get_parent(&self) -> Value<'ctx> {
        unsafe {
            Value::from_ref(LLVMGetBasicBlockParent(self.ptr()))
        }
    }

    pub fn get_terminator(&self) -> Value<'ctx> {
        unimplemented!()
    }

    pub fn insert_before(&self, name: &str) -> BasicBlock<'ctx> {
        unimplemented!()
    }
}
