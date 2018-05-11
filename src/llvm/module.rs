//! Bindings to LLVM module objects

use std::rc::Rc;
use libc::{c_char};

use llvm_sys::core::*;
use llvm_sys::prelude::*;

use llvm::context::Context;

llvm_wrapped! {
    pub struct Module {
        value: LLVMModuleRef,
        dispose: LLVMDisposeModule
    }
}

impl Module {
    /// Create a new Module with the given name
    pub fn with_name(name: &str) -> Module {
        let c_name = name.as_ptr() as *const c_char;
        let module_ref = unsafe {
            LLVMModuleCreateWithName(c_name)
        };
        Module::from_ref(module_ref)
    }

    /// Dump the contents of the module to stdout.
    ///
    /// See `LLVMDumpModule`.
    pub fn dump(&self) {
        unsafe {
            LLVMDumpModule(**self);
        }
    }

    pub fn get_context(&self) -> Context {
        let context = unsafe {
            LLVMGetModuleContext(**self)
        };
        Context::from_ref(context)
    }


}
