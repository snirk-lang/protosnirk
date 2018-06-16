//! LLVM Type object.

use std::mem;
use std::ffi::{CStr, CString};
use std::iter::IntoIterator;
use std::marker::PhantomData;

use libc::{c_uint, c_ulonglong};

use llvm_sys::prelude::*;
use llvm_sys::LLVMTypeKind;
use llvm_sys::core::*;

use llvm::{Context, Value};

#[derive(Clone)]
pub struct Type<'ctx> {
    ptr: LLVMTypeRef,
    _lt: PhantomData<&'ctx ()>
}

impl<'ctx> Type<'ctx> {
    llvm_methods!{ Type<'ctx> => LLVMTypeRef }

    // From Core / Type

    pub fn dump(&self) {
        unsafe {
            LLVMDumpType(self.ptr());
        }
    }

    pub fn get_kind(&self) -> LLVMTypeKind {
        unsafe {
            LLVMGetTypeKind(self.ptr())
        }
    }

    pub fn print_to_string(&self) -> String {
        unsafe {
            let buf = LLVMPrintTypeToString(self.ptr());
            let cstr_buf = CStr::from_ptr(buf);
            let result = String::from_utf8_lossy(cstr_buf.to_bytes()).into_owned();
            LLVMDisposeMessage(buf);
            result
        }
    }

    pub fn is_sized(&self) -> bool {
        unsafe {
            LLVMTypeIsSized(self.ptr()) > 0
        }
    }

    // From Core / Types / Floating Point Types
}

macro_rules! context_ctors {
    ($( $(#[$attr:meta])*
        pub fn $ctor_name:ident <$lt:tt> = $llvm_name:ident;
    )+) => {
        $(
            $(#[$attr])*
            pub fn $ctor_name(context: &$lt Context) -> Type<$lt> {
                unsafe {
                    Type::from_ref($llvm_name(context.ptr()))
                }
            }
        )+
    }
}

impl<'ctx> Type<'ctx> {
    // From Core / Types / Floating Point Types
    context_ctors! {
        pub fn double <'ctx> = LLVMDoubleTypeInContext;
        pub fn float <'ctx> = LLVMFloatTypeInContext;
        pub fn fp128 <'ctx> = LLVMFP128TypeInContext;
        pub fn half <'ctx> = LLVMHalfTypeInContext;
        pub fn ppcfp128 <'ctx> = LLVMPPCFP128TypeInContext;
        pub fn xf86_fp80 <'ctx> = LLVMX86FP80TypeInContext;
    }

    // From Core / Types / Function Types

    pub fn param_count(&self) -> u32 {
        unsafe {
            LLVMCountParamTypes(self.ptr()) as u32
        }
    }

    pub fn function<'a, I>(ret: &Type<'ctx>,
                          params: I,
                          is_vararg: bool) -> Type<'ctx>
    where I: IntoIterator<Item=Type<'ctx>> {
        let mut params_vec: Vec<_> = params.into_iter().collect::<Vec<_>>();
        let param_count = params_vec.len() as c_uint;
        let params_ref = params_vec.as_mut_slice();
        let params_ptrs = unsafe {
            mem::transmute::<&mut [Type<'ctx>], &mut [LLVMTypeRef]>(params_ref)
        };
        unsafe {
            Type::from_ref(LLVMFunctionType(ret.ptr(),
                             params_ptrs.as_mut_ptr(),
                             param_count,
                             is_vararg as LLVMBool))
        }
    }

    pub fn param_types(&self) -> Vec<Type<'ctx>> {
        let params_count = self.param_count();
        let mut buf : Vec<LLVMTypeRef> = Vec::with_capacity(params_count as usize);
        let p = buf.as_mut_ptr();
        unsafe {
            mem::forget(buf);
            LLVMGetParamTypes(self.ptr(), p);
            let raw = Vec::from_raw_parts(p, params_count as usize, params_count as usize);
            mem::transmute::<Vec<LLVMTypeRef>, Vec<Type<'ctx>>>(raw)
        }
    }

    pub fn return_type(&self) -> Option<Type<'ctx>> {
        let ret_ty_ref = unsafe {
            LLVMGetReturnType(self.ptr())
        };
        if ret_ty_ref.is_null() {
            None
        }
        else {
            unsafe { Some(Type::from_ref(ret_ty_ref)) }
        }
    }

    pub fn is_var_arg(&self) -> bool {
        unsafe {
            LLVMIsFunctionVarArg(self.ptr()) == 0
        }
    }

    // From Core / Types / Structure Types

    // From Core / Types / Sequential Types

    // From Core / Types / Integer Types
    context_ctors! {
        pub fn int1 <'ctx> = LLVMInt1TypeInContext;
        pub fn int8 <'ctx> = LLVMInt1TypeInContext;
        pub fn int16 <'ctx> = LLVMInt1TypeInContext;
        pub fn int32 <'ctx> = LLVMInt1TypeInContext;
        pub fn int64 <'ctx> = LLVMInt1TypeInContext;
        pub fn int128 <'ctx> = LLVMInt1TypeInContext;
    }

    pub fn int(ctx: &'ctx Context, num_bits: u32) -> Type<'ctx> {
        unsafe {
            Type::from_ref(LLVMIntTypeInContext(ctx.ptr(), num_bits as c_uint))
        }
    }

    // From Core / Types / Other Types

    context_ctors! {
        pub fn void <'ctx> = LLVMVoidTypeInContext;
        pub fn label <'ctx> = LLVMLabelTypeInContext;
        pub fn x86mmx <'ctx> = LLVMX86MMXTypeInContext;
        pub fn token <'ctx> = LLVMTokenTypeInContext;
        pub fn metadata <'ctx> = LLVMMetadataTypeInContext;
    }

    // From Core / Values / Constants
    llvm_passthrough! {
        pub fn const_null() -> Value<'ctx> => LLVMConstNull;
        pub fn const_all_ones() -> Value<'ctx> => LLVMConstAllOnes;
        pub fn get_undef() -> Value<'ctx> => LLVMGetUndef;
        pub fn const_ptr_null() -> Value<'ctx> => LLVMConstPointerNull;
    }

    // From Core / Values / Constants / Scalar

    pub fn const_int(&self, val: u64, sign_extend: bool) -> Value<'ctx> {
        unsafe {
            Value::from_ref(LLVMConstInt(self.ptr(),
                val as c_ulonglong, sign_extend as LLVMBool))
        }
    }

    pub fn const_real(&self, n: f64) -> Value<'ctx> {
        unsafe {
            Value::from_ref(LLVMConstReal(self.ptr(), n))
        }
    }
}
