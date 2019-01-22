//! Bindings to LLVM's ORC API

use std::ffi::{CString, CStr};
use std::{mem, ptr};
use libc::{c_char, c_void};

use llvm_sys::prelude::*;
use llvm_sys::orc::*;

use llvm::{Module, TargetMachine};

pub type SymbolResolver<C> = fn(name: &str, ctx: &C) -> u64;

type SymbolResolverData<C> = (SymbolResolver<C>, C);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum OrcErrorCode {
    Generic
}

pub type OrcResult<T> = Result<T, OrcErrorCode>;

#[inline(never)]
extern "C" fn default_symbol_resolver<C>(name: *const c_char,
                                         data: *mut c_void) -> u64 {
    let res_ctx_box: Box<SymbolResolverData<C>> = unsafe {
        Box::from_raw(data as *mut SymbolResolverData<C>)
    };

    let name = unsafe {
        CStr::from_ptr(name).to_str().expect("Bad symbol name")
    };

    let &(ref resolver, ref resolver_ctx) = res_ctx_box.as_ref();

    resolver(name, resolver_ctx)
}

extern "C" fn empty_symbol_resolver(name: *const c_char,
                                    data: *mut c_void) -> u64 {
    0
}

pub type LazyCompileCallback<C> = fn(jit: &JitStack, ctx: &C);

type LazyCompileCallbackData<C> = (LazyCompileCallback<C>, C);

#[inline(never)]
extern "C" fn default_compile_callback<C>(jit: LLVMOrcJITStackRef,
                                         data: *mut c_void) {
    let callback_ctx_box: Box<LazyCompileCallbackData<C>> = unsafe {
        Box::from_raw(data as *mut LazyCompileCallbackData<C>)
    };

    let jit = unsafe { JitStack::from_ref(jit) };

    let &(ref callback, ref callback_ctx) = callback_ctx_box.as_ref();

    callback(&jit, callback_ctx);

    mem::forget(jit); // Do not double free
}

pub struct JitStack {
    ptr: LLVMOrcJITStackRef
}

impl_llvm_ptr_fmt!(JitStack);

impl Drop for JitStack {
    fn drop(&mut self) {
        unsafe {
            LLVMOrcDisposeInstance(self.ptr());
        }
    }
}

impl JitStack {
    pub unsafe fn from_ref(ptr: LLVMOrcJITStackRef) -> JitStack {
        JitStack { ptr }
    }

    pub fn ptr(&self) -> LLVMOrcJITStackRef {
        self.ptr
    }

    pub fn new(target_machine: TargetMachine) -> JitStack {
        unsafe {
            llvm_sys::execution_engine::LLVMLinkInMCJIT();
        }
        let ptr = unsafe {
            LLVMOrcCreateInstance(target_machine.ptr())
        };
        mem::forget(target_machine);
        unsafe { JitStack::from_ref(ptr) }
    }

    pub fn add_object_file(&self, buffer: LLVMMemoryBufferRef) -> OrcResult<LLVMOrcModuleHandle> {
        let ret_addr = &mut 0u64 as *mut LLVMOrcModuleHandle;
        let result = unsafe {
            LLVMOrcAddObjectFile(self.ptr(),
                                 ret_addr,
                                 buffer,
                                 None,
                                 ptr::null_mut() as *mut c_void)
        };
        if result == LLVMOrcErrorCode::LLVMOrcErrSuccess {
            if ret_addr.is_null() {
                Err(OrcErrorCode::Generic)
            }
            else {
                Ok(ret_addr as u64)
            }
        }
        else {
            println!("error: {}", unsafe { CStr::from_ptr(LLVMOrcGetErrorMsg(self.ptr())).to_string_lossy() });
            Err(OrcErrorCode::Generic)
        }
    }

    pub fn add_eagerly_compiled_ir(&self, mod_: LLVMModuleRef) -> OrcResult<LLVMOrcModuleHandle> {
        let ret_addr = Box::into_raw(Box::new(0u64));
        let result = unsafe {
            LLVMOrcAddEagerlyCompiledIR(self.ptr(),
                                        ret_addr,
                                        mod_,
                                        None,
                                        ptr::null_mut() as *mut c_void)
        };
        if result == LLVMOrcErrorCode::LLVMOrcErrSuccess {
            if ret_addr.is_null() {
                println!("ret addr is null?");
                Err(OrcErrorCode::Generic)
            }
            else {
                Ok(ret_addr as u64)
            }
        }
        else {
            println!("error: {}", unsafe { CStr::from_ptr(LLVMOrcGetErrorMsg(self.ptr())).to_string_lossy() });
            Err(OrcErrorCode::Generic)
        }
    }

    pub fn add_eagerly_compiled_ir_with<C>(&self,
                                      module: Module,
                                      resolver: SymbolResolver<C>,
                                      data: C) -> OrcResult<LLVMOrcModuleHandle> {
        let ret_addr = 0 as *mut LLVMOrcTargetAddress;
        let initial_data: Box<SymbolResolverData<C>> = Box::new((resolver, data));
        let initial_data_ptr = Box::into_raw(initial_data);
        let callback: LLVMOrcSymbolResolverFn = unsafe {
            // better hope this works
            std::mem::transmute_copy(&Some(default_symbol_resolver::<C>))
        };


        let result = unsafe {
            LLVMOrcAddEagerlyCompiledIR(self.ptr(),
                                        ret_addr,
                                        module.ptr(),
                                        callback as LLVMOrcSymbolResolverFn,
                                        initial_data_ptr as *mut c_void)
        };
        if result == LLVMOrcErrorCode::LLVMOrcErrSuccess {
            Ok(unsafe { *ret_addr })
        }
        else {
            Err(OrcErrorCode::Generic)
        }
    }

    pub fn get_symbol_address(&self, name: &str)
                              -> OrcResult<LLVMOrcTargetAddress> {
        let ret_addr = 0 as *mut LLVMOrcTargetAddress;
        let c_name = CString::new(name).unwrap();
        let result = unsafe {
            LLVMOrcGetSymbolAddress(self.ptr(),
                                    ret_addr,
                                    c_name.as_ptr())
        };
        if result == LLVMOrcErrorCode::LLVMOrcErrSuccess {
            Ok(unsafe {*ret_addr })
        }
        else {
            Err(OrcErrorCode::Generic)
        }
    }

    pub unsafe fn get_symbol_as<T>(&self, name: &str) -> OrcResult<*mut T> {
        self.get_symbol_address(name)
            .map(|addr| mem::transmute(addr as *mut T))
    }
}
