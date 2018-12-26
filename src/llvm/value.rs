//! Bindings to LLVM value objects

use std::ffi::CString;
use std::mem;

use libc::{c_char, c_uint};

use llvm_sys::core::*;
use llvm_sys::prelude::*;
use llvm_sys::analysis::{LLVMVerifierFailureAction, LLVMVerifyFunction};

use llvm::BasicBlock;
use llvm::types::Type;

/// Represents many LLVM value types.
///
/// Currently incomplete. I only need floating type stuff right now,
/// so a lot of things are not included.
#[derive(Clone)]
pub struct Value<'ctx> {
    ptr: LLVMValueRef,
    _lt: ::std::marker::PhantomData<&'ctx ()>
}

impl_llvm_ptr_fmt!(<'ctx> Value);

impl<'ctx> Value<'ctx> {
    llvm_methods! { Value<'ctx> => LLVMValueRef }

    llvm_passthrough! {
        pub fn get_type() -> Type<'ctx> => LLVMTypeOf;
    }

    pub fn is_null_value(&self) -> bool {
        unsafe {
            LLVMIsNull(self.ptr()) == 0
        }
    }

    // methods on FunctionValue

    pub fn count_params(&self) -> u32 {
        unsafe {
            LLVMCountParams(self.ptr()) as u32
        }
    }

    pub fn get_params(&self) -> Vec<Value<'ctx>> {
        let params_count = self.count_params();
        let mut buf : Vec<LLVMValueRef> = Vec::with_capacity(params_count as usize);
        let p = buf.as_mut_ptr();
        unsafe {
            mem::forget(buf);
            LLVMGetParams(self.ptr(), p);
            let raw = Vec::from_raw_parts(p, params_count as usize, params_count as usize);
            mem::transmute::<Vec<LLVMValueRef>, Vec<Value<'ctx>>>(raw)
        }
    }

    pub fn set_name(&self, name: &str) {
        let c_name = CString::new(name).unwrap();
        unsafe {
            LLVMSetValueName(self.ptr(), c_name.as_ptr() as *const c_char);
        }
    }

    pub fn verify(&self, action: LLVMVerifierFailureAction) -> bool {
        unsafe {
            LLVMVerifyFunction(self.ptr(), action) == 0
        }
    }

    pub fn dump(&self) {
        unsafe {
            LLVMDumpValue(self.ptr());
        }
    }

    // From Core / BasicBlock

    // methods on PhiNode

    pub fn add_incoming<V, B>(&self, values: V, blocks: B)
    where V: IntoIterator<Item=Value<'ctx>>,
          B: IntoIterator<Item=BasicBlock<'ctx>> {

        let mut values_vec: Vec<_> = values.into_iter().collect::<Vec<_>>();
        let values_count = values_vec.len() as c_uint;
        let values_ref = values_vec.as_mut_slice();
        let values_ptrs = unsafe {
            mem::transmute::<&mut [Value<'ctx>], &mut [LLVMValueRef]>(values_ref)
        };

        let mut blocks_vec: Vec<_> = blocks.into_iter().collect::<Vec<_>>();
        let blocks_count = blocks_vec.len() as c_uint;
        let blocks_ref = blocks_vec.as_mut_slice();
        let blocks_ptrs = unsafe {
            mem::transmute::<&mut [BasicBlock<'ctx>], &mut [LLVMBasicBlockRef]>(blocks_ref)
        };

        debug_assert_eq!(blocks_count, values_count);

        unsafe {
            LLVMAddIncoming(self.ptr(),
                            values_ptrs.as_mut_ptr(),
                            blocks_ptrs.as_mut_ptr(),
                            values_count);
        }
    }

}
