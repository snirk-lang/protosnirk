//! Bindings to LLVM value objects

use std::ffi::CString;
use libc::{c_char, c_uint};

use llvm_sys::core::*;
use llvm_sys::prelude::*;
use llvm_sys::analysis::{LLVMVerifierFailureAction, LLVMVerifyFunction};

use llvm::BasicBlock;

llvm_wrapped! {
    pub struct Value {
        value: LLVMValueRef,
        dispose: drop // Value is entirely owned by Context
    }
}

impl Value {
    // general methods

    pub fn is_null_value(&self) -> bool {
        unsafe {
            LLVMIsNull(**self) == 0
        }
    }

    // methods on FunctionValue

    pub fn count_params(&self) -> usize {
        unsafe {
            LLVMCountParams(**self) as usize
        }
    }

    pub fn get_params(&self) -> Vec<Value> {
        let param_count = self.count_params();
        if param_count == 0 {
            return vec![]
        }
        let mut params = Vec::with_capacity(param_count);
        unsafe {
            LLVMGetParams(**self, params.as_mut_ptr());
        }
        // Could be done with a transmute, but this way we get null checks too.
        params.iter().map(|value| Value::from_ref(*value)).collect()
    }

    pub fn set_name(&self, name: &str) {
        let c_name = CString::new(name).unwrap();
        unsafe {
            LLVMSetValueName(**self, c_name.as_ptr() as *const c_char);
        }
    }

    pub fn verify(&self, action: LLVMVerifierFailureAction) -> bool {
        unsafe {
            LLVMVerifyFunction(**self, action) == 0
        }
    }

    // methods on PhiNode

    pub fn add_incoming<V, B>(&self, values: V, blocks: B)
    where V: IntoIterator<Item=Value>, B: IntoIterator<Item=BasicBlock> {
        use std::ops::DerefMut;
        let in_vals: Vec<_> = values.into_iter().map(|val| *val.deref_mut()).collect();
        let in_blocks: Vec<_> = blocks.into_iter().map(|block| *block.deref_mut()).collect();

        debug_assert_eq!(in_vals.len(), in_blocks.len());

        unsafe {
            LLVMAddIncoming(**self,
                            in_vals.as_mut_ptr(),
                            in_blocks.as_mut_ptr(),
                            in_vals.len() as c_uint)
        }
    }

}
