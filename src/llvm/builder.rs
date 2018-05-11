//! Represent an LLVM builder

use std::iter::IntoIterator;
use std::ffi::{CStr, CString};

use libc::{c_char, c_int, c_uint};

use llvm::{BasicBlock, Context, Value, Type};
use llvm::util;

use llvm_sys::*;
use llvm_sys::prelude::*;
use llvm_sys::core::*;

llvm_wrapped! {
    pub struct Builder {
        value: LLVMBuilderRef,
        dispose: LLVMDisposeBuilder
    }
}

impl Builder {
    pub fn new() -> Builder {
        let builder_ref = unsafe {
            LLVMCreateBuilder()
        };
        Builder::from_ref(builder_ref)
    }

    pub fn in_context(context: &Context) -> Builder {
        let builder_ref = unsafe {
            LLVMCreateBuilderInContext(**context)
        };
        Builder::from_ref(builder_ref)
    }

    pub fn position_at(&self, block: &BasicBlock, instr: &Value) {
        unsafe {
            LLVMPositionBuilder(**self, **block, **instr)
        }
    }

    pub fn position_before(&self, instr: &Value) {
        unsafe {
            LLVMPositionBuilderBefore(**self, **instr)
        }
    }

    pub fn position_at_end(&self, block: &BasicBlock) {
        unsafe {
            LLVMPositionBuilderAtEnd(**self, **block)
        }
    }

    pub fn get_insert_block(&self) -> BasicBlock {
        let block_ref = unsafe {
            LLVMGetInsertBlock(**self)
        };
        BasicBlock::from_ref(block_ref)
    }

    pub fn clear_insertion_position(&self) {
        unsafe {
            LLVMClearInsertionPosition(**self)
        }
    }

    pub fn insert(&self, instr: &Value) {
        unsafe {
            LLVMInsertIntoBuilder(**self, **instr)
        }
    }

    pub fn insert_with_name(&self, instr: &Value, name: &str) {
        let name = CString::new(name).unwrap();
        unsafe {
            LLVMInsertIntoBuilderWithName(**self,
                                          **instr,
                                          name.as_ptr() as *const c_char)
        }
    }

    pub fn build_ret_void(&self) -> Value {
        let value_ref = unsafe {
            LLVMBuildRetVoid(**self)
        };
        Value::from_ref(value_ref)
    }

    pub fn build_ret(&self, value: Value) -> Value {
        let value_ref = unsafe {
            LLVMBuildRet(**self, *value)
        };
        Value::from_ref(value_ref)
    }

    pub fn build_br(&self, dest: &BasicBlock) -> Value {
        let value_ref = unsafe {
            LLVMBuildBr(**self, **dest)
        };
        Value::from_ref(value_ref)
    }

    pub fn build_cond_br(&self, cond: Value,
                                then_block: &BasicBlock,
                                else_block: &BasicBlock) -> Value {
        let value_ref = unsafe {
            LLVMBuildCondBr(**self, *cond, **then_block, **else_block)
        };
        Value::from_ref(value_ref)
    }

    pub fn build_switch(&self, value: Value,
                               else_block: &BasicBlock,
                               num_cases: u32) -> Value {
        let value_ref = unsafe {
            LLVMBuildSwitch(**self, *value, **else_block, num_cases)
        };
        Value::from_ref(value_ref)
    }

    pub fn build_indirect_br(&self, addr: Value, num_dests: u32) -> Value {
        let value_ref = unsafe {
            LLVMBuildIndirectBr(**self, *addr, num_dests)
        };
        Value::from_ref(value_ref)
    }

    pub fn build_invoke<I: IntoIterator<Item=Value>>(&self, func: Value,
                               args: I,
                               then_block: &BasicBlock,
                               catch_block: &BasicBlock,
                               name: &str) -> Value {
        use std::ops::DerefMut;
        let name = CString::new(name).unwrap();
        let mut mapped_args = args.into_iter().map(|arg| *arg.deref_mut())
                                  .collect::<Vec<_>>();
        let value_ref = unsafe {
            LLVMBuildInvoke(**self,
                            *func,
                            mapped_args.as_mut_ptr(),
                            mapped_args.len() as c_uint,
                            **then_block,
                            **catch_block,
                            name.as_ptr() as *const c_char)
        };
        Value::from_ref(value_ref)
    }

    pub fn build_landing_pad<I>(&self, func: Value,
                                    args: I,
                                    then_block: &BasicBlock,
                                    catch_block: &BasicBlock,
                                    name: &str) -> Value
    where I: IntoIterator<Item=Value> {
        use std::ops::DerefMut;
        let name = CString::new(name).unwrap();
        let mut mapped_args = args.into_iter().map(|arg| *arg.deref_mut())
                                  .collect::<Vec<_>>();
        let value_ref = unsafe {
            LLVMBuildInvoke(**self,
                            *func,
                            mapped_args.as_mut_ptr(),
                            mapped_args.len() as c_uint,
                            **then_block,
                            **catch_block,
                            name.as_ptr() as *const c_char)
        };
        Value::from_ref(value_ref)
    }

    pub fn build_resume(&self, exn: Value) -> Value {
        let value_ref = unsafe {
            LLVMBuildResume(**self, *exn)
        };
        Value::from_ref(value_ref)
    }

    pub fn build_unreachable(&self) -> Value {
        let value_ref = unsafe {
            LLVMBuildUnreachable(**self)
        };
        Value::from_ref(value_ref)
    }

    pub fn add_case(switch: Value, on_val: Value, dest: &BasicBlock) {
        unsafe {
            LLVMAddCase(*switch, *on_val, **dest)
        }
    }

    pub fn add_destination(indirect_br: Value, dest: &BasicBlock) {
        unsafe {
            LLVMAddDestination(*indirect_br, **dest)
        }
    }

    pub fn add_clause(landing_pad: Value, clause_val: Value) {
        unsafe {
            LLVMAddClause(*landing_pad, *clause_val)
        }
    }

    pub fn set_cleanup(landing_pad: Value, val: bool) {
        unsafe {
            LLVMSetCleanup(*landing_pad, val as LLVMBool)
        }
    }

    pub fn build_add(&self, lhs: Value, rhs: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildAdd(**self,
                         *lhs,
                         *rhs,
                         name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_nsw_add(&self, lhs: Value, rhs: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildNSWAdd(**self,
                            *lhs,
                            *rhs,
                            name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_nuw_add(&self, lhs: Value, rhs: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildNUWAdd(**self,
                            *lhs,
                            *rhs,
                            name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_fadd(&self, lhs: Value, rhs: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildFAdd(**self,
                          *lhs,
                          *rhs,
                          name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_sub(&self, lhs: Value, rhs: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildSub(**self,
                         *lhs,
                         *rhs,
                         name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_nsw_sub(&self, lhs: Value, rhs: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildNSWSub(**self,
                            *lhs,
                            *rhs,
                            name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_nuw_sub(&self, lhs: Value, rhs: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildNUWSub(**self,
                            *lhs,
                            *rhs,
                            name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_fsub(&self, lhs: Value, rhs: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildFSub(**self,
                          *lhs,
                          *rhs,
                          name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_mul(&self, lhs: Value, rhs: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildMul(**self,
                         *lhs,
                         *rhs,
                         name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_nsw_mul(&self, lhs: Value, rhs: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildNSWMul(**self,
                            *lhs,
                            *rhs,
                            name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)    }

    pub fn build_nuw_mul(&self, lhs: Value, rhs: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildNUWMul(**self,
                            *lhs,
                            *rhs,
                            name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_fmul(&self, lhs: Value, rhs: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildFMul(**self,
                          *lhs,
                          *rhs,
                          name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_udiv(&self, lhs: Value, rhs: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildUDiv(**self,
                          *lhs,
                          *rhs,
                          name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_sdiv(&self, lhs: Value, rhs: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildSDiv(**self,
                          *lhs,
                          *rhs,
                          name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_exact_sdiv(&self, lhs: Value, rhs: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildExactSDiv(**self,
                               *lhs,
                               *rhs,
                               name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_urem(&self, lhs: Value, rhs: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildURem(**self,
                          *lhs,
                          *rhs,
                          name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_srem(&self, lhs: Value, rhs: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildSRem(**self,
                          *lhs,
                          *rhs,
                          name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_frem(&self, lhs: Value, rhs: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildFRem(**self,
                          *lhs,
                          *rhs,
                          name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_shl(&self, lhs: Value, rhs: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildShl(**self,
                         *lhs,
                         *rhs,
                         name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_lshr(&self, lhs: Value, rhs: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildLShr(**self,
                          *lhs,
                          *rhs,
                          name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_ashr(&self, lhs: Value, rhs: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildAShr(**self,
                          *lhs,
                          *rhs,
                          name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_and(&self, lhs: Value, rhs: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildAnd(**self,
                         *lhs,
                         *rhs,
                         name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_or(&self, lhs: Value, rhs: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildOr(**self,
                        *lhs,
                        *rhs,
                        name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_xor(&self, lhs: Value, rhs: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildXor(**self,
                         *lhs,
                         *rhs,
                         name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_binop(&self, op: LLVMOpcode,
                              lhs: Value,
                              rhs: Value,
                              name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildBinOp(**self,
                           op,
                           *lhs,
                           *rhs,
                           name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_neg(&self, value: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildNeg(**self, *value, name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_nsw_neg(&self, value: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildNSWNeg(**self,
                            *value,
                            name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_nuw_neg(&self, value: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildNUWNeg(**self,
                            *value,
                            name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_fneg(&self, val: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildFNeg(**self, *val, name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_not(&self, val: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildNot(**self, *val, name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_malloc(&self, ty: Type, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildMalloc(**self, *ty, name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_array_malloc(&self, ty: Type, val: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildArrayMalloc(**self,
                                 *ty,
                                 *val,
                                 name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_alloca(&self, ty: Type, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildAlloca(**self, *ty, name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_array_alloca(&self, ty: Type, val: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildArrayAlloca(**self,
                                 *ty,
                                 *val,
                                 name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_free(&self, pointer: Value) -> Value {
        let val_ref = unsafe {
            LLVMBuildFree(**self, *pointer)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_load(&self, pointer: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildLoad(**self,
                          *pointer,
                          name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_store(&self, val: Value, pointer: Value) -> Value {
        let val_ref = unsafe {
            LLVMBuildStore(**self, *val, *pointer)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_gep<I>(&self,
                        pointer: Value,
                        indices: I,
                        name: &str) -> Value
    where I: IntoIterator<Item=Value> {
        use std::ops::Deref;
        let name = CString::new(name).unwrap();
        let mapped_indices = indices.into_iter().map(|ix| *ix.deref())
                                    .collect::<Vec<_>>();
        let val_ref = unsafe {
            LLVMBuildGEP(**self,
                         *pointer,
                         mapped_indices.as_mut_ptr(),
                         mapped_indices.len() as c_uint,
                         name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_in_bounds_gep<I>(&self,
                                  pointer: Value,
                                  indices: I,
                                  name: &str) -> Value
    where I: IntoIterator<Item=Value> {
        use std::ops::Deref;
        let name = CString::new(name).unwrap();
        let mapped_indices = indices.into_iter().map(|ix| *ix.deref())
                                    .collect::<Vec<_>>();
        let val_ref = unsafe {
            LLVMBuildInBoundsGEP(**self,
                         *pointer,
                         mapped_indices.as_mut_ptr(),
                         mapped_indices.len() as c_uint,
                         name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_struct_gep(&self,
                            pointer: Value,
                            ix: u32,
                            name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildStructGEP(**self,
                               *pointer,
                               ix,
                               name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_global_string(&self, string: &str, name: &str) -> Value {
        let string = CString::new(string).unwrap();
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildGlobalString(**self,
                                  string.as_ptr() as *const c_char,
                                  name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_global_string_ptr(&self, string: &str, name: &str) -> Value {
        let string = CString::new(string).unwrap();
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildGlobalStringPtr(**self,
                                     string.as_ptr() as *const c_char,
                                     name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn get_volatile(memory_access_inst: Value) -> bool {
        unsafe {
            LLVMGetVolatile(*memory_access_inst) > 0
        }
    }

    pub fn set_volatile(memory_access_inst: Value, is_volatile: bool) {
        unsafe {
            LLVMSetVolatile(*memory_access_inst, is_volatile as LLVMBool)
        }
    }

    pub fn build_trunc(&self, v: Value, ty: LLVMTypeRef, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildTrunc(**self, *v, ty, name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_zext(&self, v: Value, ty: LLVMTypeRef, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildZExt(**self, *v, ty, name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_sext(&self, v: Value, ty: LLVMTypeRef, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildSExt(**self, *v, ty, name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_fp_to_ui(&self, v: Value, ty: LLVMTypeRef, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildFPToUI(**self, *v, ty, name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_fp_to_si(&self, v: Value, ty: LLVMTypeRef, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildFPToSI(**self, *v, ty, name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_ui_to_fp(&self, v: Value, ty: LLVMTypeRef, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildUIToFP(**self, *v, ty, name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_si_to_fp(&self, v: Value, ty: LLVMTypeRef, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildSIToFP(**self, *v, ty, name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_fp_trunc(&self, v: Value, ty: LLVMTypeRef, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildFPTrunc(**self, *v, ty, name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_fp_ext(&self, v: Value, ty: LLVMTypeRef, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildFPExt(**self, *v, ty, name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_ptr_to_int(&self, v: Value, ty: LLVMTypeRef, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildPtrToInt(**self, *v, ty, name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_int_to_ptr(&self, v: Value, ty: LLVMTypeRef, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildIntToPtr(**self, *v, ty, name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_bit_cast(&self, v: Value, ty: LLVMTypeRef, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildBitCast(**self, *v, ty, name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_addr_space_cast(&self, v: Value, ty: LLVMTypeRef, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildAddrSpaceCast(**self, *v, ty, name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_zext_or_bit_cast(&self, v: Value, ty: LLVMTypeRef, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildZExtOrBitCast(**self, *v, ty, name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_sext_or_bit_cast(&self, v: Value, ty: LLVMTypeRef, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildSExtOrBitCast(**self, *v, ty, name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_trunc_or_bit_cast(&self, v: Value, ty: LLVMTypeRef, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildTruncOrBitCast(**self, *v, ty, name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_cast(&self, op: LLVMOpcode, v: Value, ty: LLVMTypeRef, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildCast(**self, op, *v, ty, name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_pointer_cast(&self, v: Value, ty: LLVMTypeRef, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildPointerCast(**self, *v, ty, name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_int_cast(&self, v: Value, ty: LLVMTypeRef, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildIntCast(**self, *v, ty, name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_fp_cast(&self, v: Value, ty: LLVMTypeRef, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildFPCast(**self, *v, ty, name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_icmp(&self, op: LLVMIntPredicate, lhs: Value, rhs: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildICmp(**self, op, *lhs, *rhs, name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_fcmp(&self, op: LLVMRealPredicate, lhs: Value, rhs: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildFCmp(**self, op, *lhs, *rhs, name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_phi(&self, ty: LLVMTypeRef, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildPhi(**self, ty, name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_call<I>(&self,
                         func: Value,
                         args: I,
                         name: &str) -> Value
    where I: IntoIterator<Item=Value> {
        use std::ops::DerefMut;
        let name = CString::new(name).unwrap();
        let mapped_args = args.into_iter().map(|arg| *arg.deref_mut()).collect::<Vec<_>>();
        let val_ref = unsafe {
            LLVMBuildCall(**self,
                          *func,
                          mapped_args.as_mut_ptr(),
                          mapped_args.len() as c_uint,
                          name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_select(&self, cond: Value,
                               then_bl: &BasicBlock,
                               else_bl: &BasicBlock,
                               name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildSelect(**self,
                            *cond,
                            **then_bl.as_value(),
                            **else_bl.as_value(),
                            name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_va_arg(&self, list: Value, ty: Type, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildVAArg(**self, *list, *ty, name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_extract_element(&self, vec_val: Value, idx: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildExtractElement(**self, *vec_val, *idx, name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_insert_element(&self, vec_val: Value, v: Value, idx: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildInsertElement(**self, *vec_val, *v, *idx, name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_shuffle_vector(&self, v1: Value, v2: Value, mask: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildShuffleVector(**self, *v1, *v2, *mask, name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_extract_value(&self, agg_val: Value, idx: u32, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildExtractValue(**self, *agg_val, idx, name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_insert_value(&self, agg_val: Value, v: Value, idx: u32, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildInsertValue(**self, *agg_val, *v, idx, name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_is_null(&self, v: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildIsNull(**self, *v, name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_is_not_null(&self, value: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildIsNotNull(**self,
                               *value,
                               name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_ptr_diff(&self, lhs: Value, rhs: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildPtrDiff(**self, *lhs, *rhs, name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_fence(&self, ordering: LLVMAtomicOrdering, single_thread: bool, name: &str) -> Value {
        let name = CString::new(name).unwrap();
        let val_ref = unsafe {
            LLVMBuildFence(**self, ordering, single_thread as LLVMBool, name.as_ptr() as *const c_char)
        };
        Value::from_ref(val_ref)
    }

    pub fn build_atomic_rmw(&self, op: LLVMAtomicRMWBinOp, ptr: Value, v: Value, ordering: LLVMAtomicOrdering, single_thread: bool) -> Value {
        let val_ref = unsafe {
            LLVMBuildAtomicRMW(**self, op, *ptr, *v, ordering, single_thread as LLVMBool)
        };
        Value::from_ref(val_ref)
    }
}
