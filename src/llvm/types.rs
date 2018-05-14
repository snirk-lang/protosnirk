//! LLVM Type object.

use std::iter::IntoIterator;
use libc::{c_uint, c_ulonglong};

use llvm_sys::core::*;
use llvm_sys::prelude::*;

use llvm::Value;

llvm_wrapped! {
    pub struct Type {
        value: LLVMTypeRef,
        dispose: drop // Type is owned by the surrounding context
    }
}

macro_rules! type_ctors {
    ($(pub fn $name:ident = $core:ident;)+) => {
        impl Type {
            $(
            pub fn $name() -> Type {
                Type::from_ref(unsafe {
                    $core()
                })
            }
            )+
        }
    }
}

impl Type {
    pub fn of(value: &Value) -> Type {
        Type::from_ref(unsafe {
            LLVMTypeOf(**value)
        })
    }

    pub fn int(bits: u32) -> Type {
        Type::from_ref(unsafe {
            LLVMIntType(bits as c_uint)
        })
    }

    pub fn function<I>(ret: Type, params: I, is_vararg: bool) -> Type
    where I: IntoIterator<Item=Type> {
        use std::ops::DerefMut;
        let mut param_vec = params.into_iter().map(|param| *param.deref_mut())
                                  .collect::<Vec<_>>();
        Type::from_ref(unsafe {
            LLVMFunctionType(*ret,
                             param_vec.as_mut_ptr(),
                             param_vec.len() as c_uint,
                             if is_vararg { 0 } else { 1 })
        })
    }

    pub fn array(elements: Type, length: u32) -> Type {
        Type::from_ref(unsafe {
            LLVMArrayType(*elements, length as c_uint)
        })
    }
    //
    // pub fn int_ptr(target_data: TargetData) -> Type {
    //     Type::from_ref(unsafe {
    //         LLVMIntPtrType(*target_data)
    //     })
    // }

    pub fn null_value(&self) -> Value {
        Value::from_ref(unsafe {
            LLVMConstNull(**self)
        })
    }

    pub fn null_ptr_value(&self) -> Value {
        Value::from_ref(unsafe {
            LLVMConstPointerNull(**self)
        })
    }

    pub fn undef_value(&self) -> Value {
        Value::from_ref(unsafe {
            LLVMGetUndef(**self)
        })
    }

    pub fn const_int(self, value: u64, sign_extend: bool) -> Value {
        let sext = if sign_extend { 0 } else { 1 };
        Value::from_ref(unsafe {
            LLVMConstInt(*self, value as c_ulonglong, sext)
        })
    }

    pub fn const_real(self, value: f64) -> Value {
        Value::from_ref(unsafe {
            LLVMConstReal(*self, value)
        })
    }

    pub fn is_sized(&self) -> bool {
        unsafe {
            LLVMTypeIsSized(**self) == 0
        }
    }

    pub fn dump(&self) {
        unsafe {
            LLVMDumpType(**self)
        }
    }
}

type_ctors! {
    pub fn void = LLVMVoidType;

    pub fn int1 = LLVMInt1Type;
    pub fn int8 = LLVMInt8Type;
    pub fn int16 = LLVMInt16Type;
    pub fn int32 = LLVMInt32Type;
    pub fn int64 = LLVMInt64Type;
    pub fn int128 = LLVMInt128Type;

    pub fn fp138 = LLVMFP128Type;

    pub fn half = LLVMHalfType;
    pub fn float = LLVMFloatType;
    pub fn double = LLVMDoubleType;

    pub fn label = LLVMLabelType;
}
