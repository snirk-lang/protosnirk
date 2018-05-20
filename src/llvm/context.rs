//! Bindings to LLVM context objects

use std::rc::Rc;
use std::ffi::CString;
use libc::{c_char};

use llvm_sys::core::*;
use llvm_sys::prelude::*;

use llvm::{Value, Type, Module, BasicBlock, Builder};

pub struct Context {
    ptr: LLVMContextRef
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            LLVMContextDispose(self.ptr());
        }
    }
}

impl Context {
    // Can't use `llvm_methods` due to no lifetime needs :)

    pub unsafe fn from_ref(ptr: LLVMContextRef) -> Context {
        Context { ptr }
    }

    pub fn ptr(&self) -> LLVMContextRef {
        self.ptr
    }

    // From Core / Contexts

    pub fn new() -> Context {
        unsafe {
            Context::from_ref(LLVMContextCreate())
        }
    }

    pub fn append_basic_block<'ctx>(&'ctx self,
                                  func: &Value<'ctx>,
                                  name: &str) -> BasicBlock<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            BasicBlock::from_ref(LLVMAppendBasicBlockInContext(self.ptr(),
                                          func.ptr(),
                                          name.as_ptr() as *const c_char))
        }
    }

    pub fn new_builder<'ctx>(&'ctx self) -> Builder<'ctx> {
        unsafe {
            Builder::from_ref(
                LLVMCreateBuilderInContext(self.ptr())
            )
        }
    }

    pub fn new_module<'ctx>(&'ctx self, name: &str) -> Module<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Module::from_ref(
                LLVMModuleCreateWithNameInContext(name.as_ptr() as *const c_char,
                                                  self.ptr()))
        }
    }
}
