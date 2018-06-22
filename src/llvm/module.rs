//! Bindings to LLVM module objects

use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use libc::{c_char};

use llvm_sys::core::*;
use llvm_sys::prelude::*;
use llvm_sys::analysis::{LLVMVerifierFailureAction, LLVMVerifyModule};

use llvm::{Context, Type, Value};

/// Handle to an LLVM Module. Owned by an LLVM Context.
#[derive(Debug, Clone)]
pub struct Module<'ctx> {
    ptr: LLVMModuleRef,
    _lt: PhantomData<&'ctx ()>
}

impl_llvm_ptr_fmt!(<'ctx> Module);

impl<'ctx> Drop for Module<'ctx> {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeModule(self.ptr())
        }
    }
}

impl<'ctx> Module<'ctx> {
    llvm_methods! { Module<'ctx> => LLVMModuleRef }

    // From Core / Modules

    /// Dump the contents of the module to stderr.
    ///
    /// See `LLVMDumpModule`.
    pub fn dump(&self) {
        unsafe {
            LLVMDumpModule(self.ptr());
        }
    }

    pub fn print_to_string(&self) -> String {
        unsafe {
            let buf = LLVMPrintModuleToString(self.ptr());
            let cstr_buf = CStr::from_ptr(buf);
            let result = String::from_utf8_lossy(cstr_buf.to_bytes()).into_owned();
            LLVMDisposeMessage(buf);
            result
        }
    }

    pub fn add_function(&self, name: &str, ty: &Type<'ctx>) -> Value<'ctx> {
        let c_name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(
                LLVMAddFunction(self.ptr(),
                                c_name.as_ptr() as *const c_char,
                                ty.ptr())
            )
        }
    }

    pub fn get_function(&self, name: &str) -> Option<Value<'ctx>> {
        let name = CString::new(name).unwrap();
        let fn_ptr = unsafe {
            LLVMGetNamedFunction(self.ptr(), name.as_ptr() as *const c_char)
        };
        if fn_ptr.is_null() {
            None
        }
        else {
            unsafe {
                Some(Value::from_ref(fn_ptr))
            }
        }
    }


    pub fn get_type_by_name(&self, name: &str) -> Option<Type<'ctx>> {
        let c_name = CString::new(name).unwrap();
        let ty_ref = unsafe {
            LLVMGetTypeByName(self.ptr(), c_name.as_ptr() as *const c_char)
        };
        if ty_ref.is_null() {
            None
        }
        else {
            Some(unsafe { Type::from_ref(ty_ref) })
        }
    }


    pub fn verify(&self,
                  action: LLVMVerifierFailureAction) -> Result<(), String> {
        let mut error = 0 as *mut c_char;
        unsafe {
            if LLVMVerifyModule(self.ptr(), action, &mut error) > 0 {
                let cstr_buf = CStr::from_ptr(error);
                let result = String::from_utf8_lossy(cstr_buf.to_bytes())
                                     .into_owned();
                LLVMDisposeMessage(error);
                Err(result)
            } else {
                Ok(())
            }
        }
    }
}
