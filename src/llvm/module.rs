//! Bindings to LLVM module objects

use std::ffi::{CStr, CString};
use libc::{c_char};

use llvm_sys::core::*;
use llvm_sys::prelude::*;
use llvm_sys::analysis::{LLVMVerifierFailureAction, LLVMVerifyModule};

use llvm::{Context, Type, Value};

llvm_wrapped! {
    pub struct Module {
        value: LLVMModuleRef,
        dispose: LLVMDisposeModule
    }
}

impl Module {
    /// Create a new Module with the given name
    pub fn with_name(name: &str) -> Module {
        let c_name = CString::new(name).unwrap();
        let module_ref = unsafe {
            LLVMModuleCreateWithName(c_name.as_ptr() as *const c_char)
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

    pub fn get_type_by_name(&self, name: &str) -> Option<Type> {
        let c_name = CString::new(name).unwrap();
        let ty_ref = unsafe {
            LLVMGetTypeByName(**self, c_name.as_ptr() as *const c_char)
        };
        if ty_ref.is_null() {
            None
        }
        else {
            Some(Type::from_ref(ty_ref))
        }
    }

    pub fn add_function(&self, name: &str, ty: Type) -> Value {
        let c_name = CString::new(name).unwrap();
        Value::from_ref(unsafe {
            LLVMAddFunction(**self, c_name.as_ptr() as *const c_char, *ty)
        })
    }

    pub fn verify(&self, action: LLVMVerifierFailureAction) -> Result<(), String> {
        let mut error = 0 as *mut c_char;
        unsafe {
            if LLVMVerifyModule(**self, action, &mut error) > 0 {
                let cstr_buf = CStr::from_ptr(error);
                let result = String::from_utf8_lossy(cstr_buf.to_bytes()).into_owned();
                LLVMDisposeMessage(error);
                Err(result)
            } else {
                Ok(())
            }
        }
    }
}
