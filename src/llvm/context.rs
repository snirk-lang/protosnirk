//! Bindings to LLVM context objects

use llvm::{Value, BasicBlock};

use std::rc::Rc;
use std::ffi::CString;
use libc::{c_char};

use llvm_sys::core::*;
use llvm_sys::prelude::*;

llvm_wrapped! {
    pub struct Context {
        value: LLVMContextRef,
        dispose: LLVMContextDispose
    }
}

impl Context {
    pub fn new() -> Context {
        let context_ref = unsafe {
            LLVMContextCreate()
        };
        Context::from_ref(context_ref)
    }

    pub fn global() -> Context {
        let context_ref = unsafe {
            LLVMGetGlobalContext()
        };
        Context::from_ref(context_ref)
    }

    pub fn append_basic_block(&self, func: &Value, name: &str) -> BasicBlock {
        let name = CString::new(name).unwrap();
        let block_ref = unsafe {
            LLVMAppendBasicBlockInContext(**self,
                                          **func,
                                          name.as_ptr() as *const c_char)
        };
        BasicBlock::from_ref(block_ref)
    }
}
