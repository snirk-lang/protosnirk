//! Represent an LLVM builder

use std::ffi::CString;
use std::iter::IntoIterator;
use std::mem;
use std::ptr;

use libc::{c_char, c_uint};

use llvm::{BasicBlock, Context, Value, Type};

use llvm_sys::*;
use llvm_sys::prelude::*;
use llvm_sys::core::*;

pub struct Builder<'ctx> {
    ptr: LLVMBuilderRef,
    _lt: ::std::marker::PhantomData<&'ctx ()>
}

impl<'ctx> Drop for Builder<'ctx> {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeBuilder(self.ptr());
        }
    }
}

impl_llvm_ptr_fmt!(<'ctx> Builder);

impl<'ctx> Builder<'ctx> {
    llvm_methods! { Builder<'ctx> => LLVMBuilderRef }

    // From IRBuilder

    pub fn new(ctx: &'ctx Context) -> Builder<'ctx> {
        unsafe {
            Builder::from_ref(LLVMCreateBuilderInContext(ctx.ptr()))
        }
    }

    llvm_passthrough! {
        pub fn position_at(block: &BasicBlock<'ctx>, instr: &Value<'ctx>)
            => LLVMPositionBuilder;
        pub fn position_before(instr: &Value<'ctx>)
            => LLVMPositionBuilderBefore;
        pub fn position_at_end(block: &BasicBlock<'ctx>)
            => LLVMPositionBuilderAtEnd;
        pub fn insert_block() -> BasicBlock<'ctx> => LLVMGetInsertBlock;
        pub fn clear_insertion_position() => LLVMClearInsertionPosition;
        pub fn insert_unnamed(instr: &Value<'ctx>) => LLVMInsertIntoBuilder;
    }

    pub fn insert(&self, instr: &Value<'ctx>, name: &str) {
        let name = CString::new(name).unwrap();
        unsafe {
            LLVMInsertIntoBuilderWithName(self.ptr(),
                                          instr.ptr(),
                                          name.as_ptr() as *const c_char)
        }
    }

    llvm_passthrough! {
        pub fn build_ret_void() -> Value<'ctx> => LLVMBuildRetVoid;
        pub fn build_ret(value: &Value<'ctx>) -> Value<'ctx> => LLVMBuildRet;
        pub fn build_br(dest: &BasicBlock<'ctx>) -> Value<'ctx>
            => LLVMBuildBr;
        pub fn build_cond_br(cond: &Value<'ctx>,
                             then_block: &BasicBlock<'ctx>,
                             else_block: &BasicBlock<'ctx>) -> Value<'ctx>
            => LLVMBuildCondBr;
    }

    pub fn build_switch(&self, value: &Value<'ctx>,
                               else_block: &BasicBlock,
                               num_cases: u32) -> Value<'ctx> {
        unsafe {
            Value::from_ref(LLVMBuildSwitch(self.ptr(),
                                            value.ptr(),
                                            else_block.ptr(),
                                            num_cases as u32))
        }
    }

    pub fn build_indirect_br(&self, addr: &Value<'ctx>,
                                    num_dests: u32) -> Value<'ctx> {
        unsafe {
            Value::from_ref(LLVMBuildIndirectBr(self.ptr(),
                                                addr.ptr(),
                                                num_dests as u32))
        }
    }

    pub fn build_invoke<I>(&self, func: &Value<'ctx>,
                                  args: I,
                                  then_block: &BasicBlock<'ctx>,
                                  catch_block: &BasicBlock<'ctx>,
                                  name: &str) -> Value<'ctx>
    where I: IntoIterator<Item=Value<'ctx>> {
        let name = CString::new(name).unwrap();
        let mut args_vec: Vec<_> = args.into_iter().collect::<Vec<_>>();
        let args_count = args_vec.len() as c_uint;
        let args_ref = args_vec.as_mut_slice();
        let args_ptrs = unsafe {
            mem::transmute::<&mut [Value<'ctx>], &mut [LLVMValueRef]>(args_ref)
        };
        unsafe {
            Value::from_ref(LLVMBuildInvoke(self.ptr(),
                            func.ptr(),
                            args_ptrs.as_mut_ptr(),
                            args_count,
                            then_block.ptr(),
                            catch_block.ptr(),
                            name.as_ptr() as *const c_char))
        }
    }

    llvm_passthrough! {
        pub fn build_unreachable() -> Value<'ctx> => LLVMBuildUnreachable;
        pub fn build_resume(exn: &Value<'ctx>) -> Value<'ctx> => LLVMBuildResume;

    }

    pub fn build_landing_pad(&self, ty: &Type<'ctx>,
                                    pers_fn: &Value<'ctx>,
                                    clauses: u32,
                                    name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildLandingPad(self.ptr(),
                                                ty.ptr(),
                                                pers_fn.ptr(),
                                                clauses as c_uint,
                                                name.as_ptr() as *const c_char))
        }
    }

    llvm_passthrough! {
        /*
        pub fn build_cleanup_ret(catch_pad: &Value<'ctx>,
                                 block: &BasicBlock<'ctx>) -> Value<'ctx>
            => LLVMBuildCleanupRet;
        pub fn build_catch_ret(catch_pad: &Value<'ctx>,
                               block: &BasicBlock<'ctx>)
                               -> Value<'ctx>
            => LLVMBuildCatchRet;
            pub fn build_cleanup_pad() => LLVMBuildCleanupPad;
            pub fn build_catch_switch() => LLVMBuildCatchSwitch;
        */
    }

    pub fn add_clause(landing_pad: &Value<'ctx>, clause_val: &Value<'ctx>) {
        unsafe {
            LLVMAddClause(landing_pad.ptr(), clause_val.ptr())
        }
    }

    pub fn set_cleanup(landing_pad: &Value<'ctx>, val: bool) {
        unsafe {
            LLVMSetCleanup(landing_pad.ptr(), val as LLVMBool)
        }
    }

    pub fn build_add(&self, lhs: &Value<'ctx>, rhs: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildAdd(self.ptr(),
                         lhs.ptr(),
                         rhs.ptr(),
                         name.as_ptr() as *const c_char))
        }
    }

    pub fn build_nsw_add(&self, lhs: &Value<'ctx>, rhs: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildNSWAdd(self.ptr(),
                            lhs.ptr(),
                            rhs.ptr(),
                            name.as_ptr() as *const c_char))
        }
    }

    pub fn build_nuw_add(&self, lhs: &Value<'ctx>, rhs: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildNUWAdd(self.ptr(),
                            lhs.ptr(),
                            rhs.ptr(),
                            name.as_ptr() as *const c_char))
        }
    }

    pub fn build_fadd(&self, lhs: &Value<'ctx>, rhs: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildFAdd(self.ptr(),
                          lhs.ptr(),
                          rhs.ptr(),
                          name.as_ptr() as *const c_char))
        }
    }

    pub fn build_sub(&self, lhs: &Value<'ctx>, rhs: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildSub(self.ptr(),
                         lhs.ptr(),
                         rhs.ptr(),
                         name.as_ptr() as *const c_char))
        }
    }

    pub fn build_nsw_sub(&self, lhs: &Value<'ctx>, rhs: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildNSWSub(self.ptr(),
                            lhs.ptr(),
                            rhs.ptr(),
                            name.as_ptr() as *const c_char))
        }
    }

    pub fn build_nuw_sub(&self, lhs: &Value<'ctx>, rhs: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildNUWSub(self.ptr(),
                            lhs.ptr(),
                            rhs.ptr(),
                            name.as_ptr() as *const c_char))
        }
    }

    pub fn build_fsub(&self, lhs: &Value<'ctx>, rhs: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildFSub(self.ptr(),
                          lhs.ptr(),
                          rhs.ptr(),
                          name.as_ptr() as *const c_char))
        }
    }

    pub fn build_mul(&self, lhs: &Value<'ctx>, rhs: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildMul(self.ptr(),
                         lhs.ptr(),
                         rhs.ptr(),
                         name.as_ptr() as *const c_char))
        }
    }

    pub fn build_nsw_mul(&self, lhs: &Value<'ctx>, rhs: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildNSWMul(self.ptr(),
                            lhs.ptr(),
                            rhs.ptr(),
                            name.as_ptr() as *const c_char))
        }
    }

    pub fn build_nuw_mul(&self, lhs: &Value<'ctx>, rhs: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildNUWMul(self.ptr(),
                            lhs.ptr(),
                            rhs.ptr(),
                            name.as_ptr() as *const c_char))
        }
    }

    pub fn build_fmul(&self, lhs: &Value<'ctx>, rhs: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildFMul(self.ptr(),
                          lhs.ptr(),
                          rhs.ptr(),
                          name.as_ptr() as *const c_char))
        }
    }

    pub fn build_fdiv(&self, lhs: &Value<'ctx>, rhs: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildFDiv(self.ptr(),
                            lhs.ptr(),
                            rhs.ptr(),
                            name.as_ptr() as *const c_char))
        }
    }

    pub fn build_udiv(&self, lhs: &Value<'ctx>, rhs: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildUDiv(self.ptr(),
                          lhs.ptr(),
                          rhs.ptr(),
                          name.as_ptr() as *const c_char))
        }
    }

    pub fn build_sdiv(&self, lhs: &Value<'ctx>, rhs: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildSDiv(self.ptr(),
                          lhs.ptr(),
                          rhs.ptr(),
                          name.as_ptr() as *const c_char))
        }
    }

    pub fn build_exact_sdiv(&self, lhs: &Value<'ctx>, rhs: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildExactSDiv(self.ptr(),
                               lhs.ptr(),
                               rhs.ptr(),
                               name.as_ptr() as *const c_char))
        }
    }

    pub fn build_urem(&self, lhs: &Value<'ctx>, rhs: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildURem(self.ptr(),
                          lhs.ptr(),
                          rhs.ptr(),
                          name.as_ptr() as *const c_char))
        }
    }

    pub fn build_srem(&self, lhs: &Value<'ctx>, rhs: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildSRem(self.ptr(),
                          lhs.ptr(),
                          rhs.ptr(),
                          name.as_ptr() as *const c_char))
        }
    }

    pub fn build_frem(&self, lhs: &Value<'ctx>, rhs: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildFRem(self.ptr(),
                          lhs.ptr(),
                          rhs.ptr(),
                          name.as_ptr() as *const c_char))
        }
    }

    pub fn build_shl(&self, lhs: &Value<'ctx>, rhs: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildShl(self.ptr(),
                         lhs.ptr(),
                         rhs.ptr(),
                         name.as_ptr() as *const c_char))
        }
    }

    pub fn build_lshr(&self, lhs: &Value<'ctx>, rhs: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildLShr(self.ptr(),
                          lhs.ptr(),
                          rhs.ptr(),
                          name.as_ptr() as *const c_char))
        }
    }

    pub fn build_ashr(&self, lhs: &Value<'ctx>, rhs: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildAShr(self.ptr(),
                          lhs.ptr(),
                          rhs.ptr(),
                          name.as_ptr() as *const c_char))
        }
    }

    pub fn build_and(&self, lhs: &Value<'ctx>, rhs: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildAnd(self.ptr(),
                         lhs.ptr(),
                         rhs.ptr(),
                         name.as_ptr() as *const c_char))
        }
    }

    pub fn build_or(&self, lhs: &Value<'ctx>, rhs: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildOr(self.ptr(),
                        lhs.ptr(),
                        rhs.ptr(),
                        name.as_ptr() as *const c_char))
        }
    }

    pub fn build_xor(&self, lhs: &Value<'ctx>, rhs: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildXor(self.ptr(),
                         lhs.ptr(),
                         rhs.ptr(),
                         name.as_ptr() as *const c_char))
        }
    }

    pub fn build_binop(&self, op: LLVMOpcode,
                              lhs: &Value<'ctx>,
                              rhs: &Value<'ctx>,
                              name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildBinOp(self.ptr(),
                           op,
                           lhs.ptr(),
                           rhs.ptr(),
                           name.as_ptr() as *const c_char))
        }
    }

    pub fn build_neg(&self, value: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildNeg(self.ptr(), value.ptr(), name.as_ptr() as *const c_char))
        }
    }

    pub fn build_nsw_neg(&self, value: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildNSWNeg(self.ptr(),
                            value.ptr(),
                            name.as_ptr() as *const c_char))
        }
    }

    pub fn build_nuw_neg(&self, value: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildNUWNeg(self.ptr(),
                            value.ptr(),
                            name.as_ptr() as *const c_char))
        }
    }

    pub fn build_fneg(&self, val: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildFNeg(self.ptr(), val.ptr(), name.as_ptr() as *const c_char))
        }
    }

    pub fn build_not(&self, val: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildNot(self.ptr(), val.ptr(), name.as_ptr() as *const c_char))
        }
    }

    pub fn build_malloc(&self, ty: &Type<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildMalloc(self.ptr(), ty.ptr(), name.as_ptr() as *const c_char))
        }
    }

    pub fn build_array_malloc(&self, ty: &Type<'ctx>, val: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildArrayMalloc(self.ptr(),
                                 ty.ptr(),
                                 val.ptr(),
                                 name.as_ptr() as *const c_char))
        }
    }

    pub fn build_alloca(&self, ty: &Type<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildAlloca(self.ptr(), ty.ptr(), name.as_ptr() as *const c_char))
        }
    }

    pub fn build_array_alloca(&self, ty: &Type<'ctx>, val: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildArrayAlloca(self.ptr(),
                                 ty.ptr(),
                                 val.ptr(),
                                 name.as_ptr() as *const c_char))
        }
    }

    pub fn build_free(&self, pointer: &Value<'ctx>) -> Value<'ctx> {
        unsafe {
            Value::from_ref(LLVMBuildFree(self.ptr(), pointer.ptr()))
        }
    }

    pub fn build_load(&self, pointer: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildLoad(self.ptr(),
                          pointer.ptr(),
                          name.as_ptr() as *const c_char))
        }
    }

    pub fn build_store(&self, val: &Value<'ctx>, pointer: &Value<'ctx>) -> Value<'ctx> {
        unsafe {
            Value::from_ref(LLVMBuildStore(self.ptr(), val.ptr(), pointer.ptr()))
        }
    }

    pub fn build_gep<I>(&self,
                        pointer: &Value<'ctx>,
                        indices: I,
                        name: &str) -> Value<'ctx>
    where I: IntoIterator<Item=Value<'ctx>> {
        let name = CString::new(name).unwrap();
        let mut indices_vec: Vec<_> = indices.into_iter().collect::<Vec<_>>();
        let indices_count = indices_vec.len() as c_uint;
        let indices_ref = indices_vec.as_mut_slice();
        let indices_ptrs = unsafe {
            mem::transmute::<&mut [Value<'ctx>], &mut [LLVMValueRef]>(indices_ref)
        };
        unsafe {
            Value::from_ref(LLVMBuildGEP(self.ptr(),
                         pointer.ptr(),
                         indices_ptrs.as_mut_ptr(),
                         indices_count,
                         name.as_ptr() as *const c_char))
        }
    }

    pub fn build_in_bounds_gep<I>(&self,
                                  pointer: &Value<'ctx>,
                                  indices: I,
                                  name: &str) -> Value<'ctx>
    where I: IntoIterator<Item=Value<'ctx>> {
        let name = CString::new(name).unwrap();
        let mut indices_vec: Vec<_> = indices.into_iter().collect::<Vec<_>>();
        let indices_count = indices_vec.len() as c_uint;
        let indices_ref = indices_vec.as_mut_slice();
        let indices_ptrs = unsafe {
            mem::transmute::<&mut [Value<'ctx>], &mut [LLVMValueRef]>(indices_ref)
        };
        unsafe {
            Value::from_ref(LLVMBuildInBoundsGEP(self.ptr(),
                         pointer.ptr(),
                         indices_ptrs.as_mut_ptr(),
                         indices_count,
                         name.as_ptr() as *const c_char))
        }
    }

    pub fn build_struct_gep(&self,
                            pointer: &Value<'ctx>,
                            ix: u32,
                            name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildStructGEP(self.ptr(),
                               pointer.ptr(),
                               ix,
                               name.as_ptr() as *const c_char))
        }
    }

    pub fn build_global_string(&self, string: &str, name: &str) -> Value<'ctx> {
        let string = CString::new(string).unwrap();
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildGlobalString(self.ptr(),
                                  string.as_ptr() as *const c_char,
                                  name.as_ptr() as *const c_char))
        }
    }

    pub fn build_global_string_ptr(&self, string: &str, name: &str) -> Value<'ctx> {
        let string = CString::new(string).unwrap();
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildGlobalStringPtr(self.ptr(),
                                     string.as_ptr() as *const c_char,
                                     name.as_ptr() as *const c_char))
        }
    }

    pub fn get_volatile(memory_access_inst: &Value<'ctx>) -> bool {
        unsafe {
            LLVMGetVolatile(memory_access_inst.ptr()) > 0
        }
    }

    pub fn set_volatile(memory_access_inst: &Value<'ctx>, is_volatile: bool) {
        unsafe {
            LLVMSetVolatile(memory_access_inst.ptr(), is_volatile as LLVMBool)
        }
    }

    pub fn build_trunc(&self, val: &Value<'ctx>, ty: &Type<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildTrunc(self.ptr(), val.ptr(), ty.ptr(), name.as_ptr() as *const c_char))
        }
    }

    pub fn build_zext(&self, val: &Value<'ctx>, ty: &Type<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildZExt(self.ptr(), val.ptr(), ty.ptr(), name.as_ptr() as *const c_char))
        }
    }

    pub fn build_sext(&self, val: &Value<'ctx>, ty: &Type<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildSExt(self.ptr(), val.ptr(), ty.ptr(), name.as_ptr() as *const c_char))
        }
    }

    pub fn build_fp_to_ui(&self, val: &Value<'ctx>, ty: &Type<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildFPToUI(self.ptr(), val.ptr(), ty.ptr(), name.as_ptr() as *const c_char))
        }
    }

    pub fn build_fp_to_si(&self, val: &Value<'ctx>, ty: &Type<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildFPToSI(self.ptr(), val.ptr(), ty.ptr(), name.as_ptr() as *const c_char))
        }
    }

    pub fn build_ui_to_fp(&self, val: &Value<'ctx>, ty: &Type<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildUIToFP(self.ptr(), val.ptr(), ty.ptr(), name.as_ptr() as *const c_char))
        }
    }

    pub fn build_si_to_fp(&self, val: &Value<'ctx>, ty: &Type<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildSIToFP(self.ptr(), val.ptr(), ty.ptr(), name.as_ptr() as *const c_char))
        }
    }

    pub fn build_fp_trunc(&self, val: &Value<'ctx>, ty: &Type<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildFPTrunc(self.ptr(), val.ptr(), ty.ptr(), name.as_ptr() as *const c_char))
        }
    }

    pub fn build_fp_ext(&self, val: &Value<'ctx>, ty: &Type<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildFPExt(self.ptr(), val.ptr(), ty.ptr(), name.as_ptr() as *const c_char))
        }
    }

    pub fn build_ptr_to_int(&self, val: &Value<'ctx>, ty: &Type<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildPtrToInt(self.ptr(), val.ptr(), ty.ptr(), name.as_ptr() as *const c_char))
        }
    }

    pub fn build_int_to_ptr(&self, val: &Value<'ctx>, ty: &Type<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildIntToPtr(self.ptr(), val.ptr(), ty.ptr(), name.as_ptr() as *const c_char))
        }
    }

    pub fn build_bit_cast(&self, val: &Value<'ctx>, ty: &Type<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildBitCast(self.ptr(), val.ptr(), ty.ptr(), name.as_ptr() as *const c_char))
        }
    }

    pub fn build_addr_space_cast(&self, val: &Value<'ctx>, ty: &Type<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildAddrSpaceCast(self.ptr(), val.ptr(), ty.ptr(), name.as_ptr() as *const c_char))
        }
    }

    pub fn build_zext_or_bit_cast(&self, val: &Value<'ctx>, ty: &Type<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildZExtOrBitCast(self.ptr(), val.ptr(), ty.ptr(), name.as_ptr() as *const c_char))
        }
    }

    pub fn build_sext_or_bit_cast(&self, val: &Value<'ctx>, ty: &Type<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildSExtOrBitCast(self.ptr(), val.ptr(), ty.ptr(), name.as_ptr() as *const c_char))
        }
    }

    pub fn build_trunc_or_bit_cast(&self, val: &Value<'ctx>, ty: &Type<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildTruncOrBitCast(self.ptr(), val.ptr(), ty.ptr(), name.as_ptr() as *const c_char))
        }
    }

    pub fn build_cast(&self, op: LLVMOpcode, val: &Value<'ctx>, ty: &Type<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildCast(self.ptr(), op, val.ptr(), ty.ptr(), name.as_ptr() as *const c_char))
        }
    }

    pub fn build_pointer_cast(&self, val: &Value<'ctx>, ty: &Type<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildPointerCast(self.ptr(), val.ptr(), ty.ptr(), name.as_ptr() as *const c_char))
        }
    }

    pub fn build_int_cast(&self, val: &Value<'ctx>, ty: &Type<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildIntCast(self.ptr(), val.ptr(), ty.ptr(), name.as_ptr() as *const c_char))
        }
    }

    pub fn build_fp_cast(&self, val: &Value<'ctx>, ty: &Type<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildFPCast(self.ptr(), val.ptr(), ty.ptr(), name.as_ptr() as *const c_char))
        }
    }

    pub fn build_icmp(&self, op: LLVMIntPredicate, lhs: &Value<'ctx>, rhs: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildICmp(self.ptr(), op, lhs.ptr(), rhs.ptr(), name.as_ptr() as *const c_char))
        }
    }

    pub fn build_fcmp(&self, op: LLVMRealPredicate, lhs: &Value<'ctx>, rhs: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildFCmp(self.ptr(), op, lhs.ptr(), rhs.ptr(), name.as_ptr() as *const c_char))
        }
    }

    pub fn build_phi(&self, ty: &Type<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildPhi(self.ptr(), ty.ptr(), name.as_ptr() as *const c_char))
        }
    }

    pub fn build_unnamed_call<I>(&self,
                                 func: &Value<'ctx>,
                                 args: I) -> Value<'ctx>
                                 where I: IntoIterator<Item=Value<'ctx>> {
        let mut args_vec: Vec<_> = args.into_iter().collect::<Vec<_>>();
        let args_count = args_vec.len() as c_uint;
        let args_ref = args_vec.as_mut_slice();
        let args_ptrs = unsafe {
            mem::transmute::<&mut [Value<'ctx>], &mut [LLVMValueRef]>(args_ref)
        };
        unsafe {
            Value::from_ref(LLVMBuildCall(self.ptr(),
                                          func.ptr(),
                                          args_ptrs.as_mut_ptr(),
                                          args_count,
                                          ptr::null()))
        }
    }

    pub fn build_call<I>(&self,
                         func: &Value<'ctx>,
                         args: I,
                         name: &str) -> Value<'ctx>
    where I: IntoIterator<Item=Value<'ctx>> {
        let mut args_vec: Vec<_> = args.into_iter().collect::<Vec<_>>();
        let args_count = args_vec.len() as c_uint;
        let args_ref = args_vec.as_mut_slice();
        let args_ptrs = unsafe {
            mem::transmute::<&mut [Value<'ctx>], &mut [LLVMValueRef]>(args_ref)
        };
        unsafe {
            Value::from_ref(LLVMBuildCall(self.ptr(),
                          func.ptr(),
                          args_ptrs.as_mut_ptr(),
                          args_count,
                          name.as_ptr() as *const c_char))
        }
    }

    pub fn build_select(&self, cond: &Value<'ctx>,
                               then_bl: &BasicBlock<'ctx>,
                               else_bl: &BasicBlock<'ctx>,
                               name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildSelect(self.ptr(),
                            cond.ptr(),
                            then_bl.as_value().ptr(),
                            else_bl.as_value().ptr(),
                            name.as_ptr() as *const c_char))
        }
    }

    pub fn build_va_arg(&self, list: &Value<'ctx>, ty: &Type<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildVAArg(self.ptr(), list.ptr(), ty.ptr(), name.as_ptr() as *const c_char))
        }
    }

    pub fn build_extract_element(&self, vec_val: &Value<'ctx>, idx: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildExtractElement(self.ptr(), vec_val.ptr(), idx.ptr(), name.as_ptr() as *const c_char))
        }
    }

    pub fn build_insert_element(&self, vec_val: &Value<'ctx>, val: &Value<'ctx>, idx: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildInsertElement(self.ptr(), vec_val.ptr(), val.ptr(), idx.ptr(), name.as_ptr() as *const c_char))
        }
    }

    pub fn build_shuffle_vector(&self, v1: &Value<'ctx>, v2: &Value<'ctx>, mask: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildShuffleVector(self.ptr(), v1.ptr(), v2.ptr(), mask.ptr(), name.as_ptr() as *const c_char))
        }
    }

    pub fn build_extract_value(&self, agg_val: &Value<'ctx>, idx: u32, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildExtractValue(self.ptr(), agg_val.ptr(), idx, name.as_ptr() as *const c_char))
        }
    }

    pub fn build_insert_value(&self, agg_val: &Value<'ctx>, val: &Value<'ctx>, idx: u32, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildInsertValue(self.ptr(), agg_val.ptr(), val.ptr(), idx, name.as_ptr() as *const c_char))
        }
    }

    pub fn build_is_null(&self, val: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildIsNull(self.ptr(), val.ptr(), name.as_ptr() as *const c_char))
        }
    }

    pub fn build_is_not_null(&self, value: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildIsNotNull(self.ptr(),
                               value.ptr(),
                               name.as_ptr() as *const c_char))
        }
    }

    pub fn build_ptr_diff(&self, lhs: &Value<'ctx>, rhs: &Value<'ctx>, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildPtrDiff(self.ptr(), lhs.ptr(), rhs.ptr(), name.as_ptr() as *const c_char))
        }
    }

    pub fn build_fence(&self, ordering: LLVMAtomicOrdering, single_thread: bool, name: &str) -> Value<'ctx> {
        let name = CString::new(name).unwrap();
        unsafe {
            Value::from_ref(LLVMBuildFence(self.ptr(), ordering, single_thread as LLVMBool, name.as_ptr() as *const c_char))
        }
    }

    pub fn build_atomic_rmw(&self, op: LLVMAtomicRMWBinOp, ptr: &Value<'ctx>, val: &Value<'ctx>, ordering: LLVMAtomicOrdering, single_thread: bool) -> Value<'ctx> {
        unsafe {
            Value::from_ref(LLVMBuildAtomicRMW(self.ptr(), op, ptr.ptr(), val.ptr(), ordering, single_thread as LLVMBool))
        }
    }
}
