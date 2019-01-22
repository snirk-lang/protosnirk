//! Bindings to LLVM target methods

use std::ffi::{CStr, CString};
use libc::c_char;

use llvm_sys::core::LLVMDisposeMessage;
use llvm_sys::target::*;
use llvm_sys::target_machine::*;

pub fn initialize_native_target() -> bool {
    unsafe {
        if LLVM_InitializeNativeTarget() == 1 {
            return false
        }
        return true
    }
}

pub fn initialize_native_target_asm() {
    unsafe {
        LLVM_InitializeNativeAsmPrinter();
        LLVM_InitializeNativeAsmParser();
    }
}

pub fn initialize_all_targets() {
    unsafe {
        LLVM_InitializeAllTargets()
    }
}

pub fn native_target_triple() -> String {
    unsafe {
        let buf = LLVMGetDefaultTargetTriple();
        let cstr_buf = CStr::from_ptr(buf);
        let result = String::from_utf8_lossy(cstr_buf.to_bytes()).into_owned();
        LLVMDisposeMessage(buf);
        result
    }
}

pub fn native_cpu_name() -> String {
    unsafe {
        let buf = LLVMGetHostCPUName();
        let cstr_buf = CStr::from_ptr(buf);
        let result = String::from_utf8_lossy(cstr_buf.to_bytes()).into_owned();
        LLVMDisposeMessage(buf);
        result
    }
}

pub fn native_cpu_features() -> String {
    unsafe {
        let buf = LLVMGetHostCPUFeatures();
        let cstr_buf = CStr::from_ptr(buf);
        let result = String::from_utf8_lossy(cstr_buf.to_bytes()).into_owned();
        LLVMDisposeMessage(buf);
        result
    }
}

pub struct Target {
    ptr: LLVMTargetRef
}

impl_llvm_ptr_fmt!(Target);

impl Target {
    pub unsafe fn from_ref(ptr: LLVMTargetRef) -> Target {
        Target { ptr }
    }
    pub fn ptr(&self) -> LLVMTargetRef {
        self.ptr
    }

    pub fn first() -> Target {
        unsafe {
            Target::from_ref(LLVMGetFirstTarget())
        }
    }

    pub fn next(&self) -> Option<Target> {
        let ptr = unsafe {
            LLVMGetNextTarget(self.ptr())
        };
        if ptr.is_null() {
            None
        }
        else {
            Some(unsafe { Target::from_ref(ptr) })
        }
    }

    pub fn from_triple(triple: &str) -> Result<Target, String> {
        let mut first_ptr = Target::first().ptr;
        let triple_str = CString::new(triple).unwrap();
        let mut error_ptr = 0 as *mut c_char;
        let result = unsafe {
            LLVMGetTargetFromTriple(triple_str.as_ptr(),
                                    &mut first_ptr as *mut LLVMTargetRef,
                                    &mut error_ptr)
        };
        if result != 0 {
            unsafe {
            let cstr_buf = CStr::from_ptr(error_ptr);
            let error = String::from_utf8_lossy(cstr_buf.to_bytes())
                                .into_owned();
            LLVMDisposeMessage(error_ptr);
            Err(error)
            }
        }
        else {
            Ok(unsafe { Target::from_ref(first_ptr) })
        }
    }

    pub fn native() -> Result<Target, String> {
        Target::from_triple(&native_target_triple())
    }

    pub fn get_name(&self) -> String {
        unsafe {
            let buf = LLVMGetTargetName(self.ptr());
            let cstr_buf = CStr::from_ptr(buf);
            let result = String::from_utf8_lossy(cstr_buf.to_bytes()).into_owned();
            result
        }
    }


    pub fn get_description(&self) -> String {
        unsafe {
            let buf = LLVMGetTargetDescription(self.ptr());
            let cstr_buf = CStr::from_ptr(buf);
            let result = String::from_utf8_lossy(cstr_buf.to_bytes()).into_owned();
            result
        }
    }
}

pub struct TargetData {
    ptr: LLVMTargetDataRef
}

impl_llvm_ptr_fmt!(TargetData);

impl Drop for TargetData {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeTargetData(self.ptr());
        }
    }
}

impl TargetData {
    pub unsafe fn from_ref(ptr: LLVMTargetDataRef) -> TargetData {
        TargetData { ptr }
    }

    pub fn ptr(&self) -> LLVMTargetDataRef {
        self.ptr
    }

    /// # Panics
    /// Will abort if given an unknown target layout
    pub fn from_target_layout(layout: &str) -> TargetData {
        let c_str = CString::new(layout).unwrap();
        unsafe {
            TargetData::from_ref(
                LLVMCreateTargetData(c_str.as_ptr() as *const c_char)
            )
        }
    }

    pub fn from_machine(machine: &TargetMachine) -> TargetData {
        unsafe {
            TargetData::from_ref(LLVMCreateTargetDataLayout(machine.ptr()))
        }
    }

    pub fn native(opt_level: LLVMCodeGenOptLevel,
                  reloc_mode: LLVMRelocMode,
                  code_model: LLVMCodeModel) -> Result<TargetData, String> {
        let machine = try!(TargetMachine::native(opt_level, reloc_mode, code_model));
        Ok(TargetData::from_machine(&machine))
    }

}

pub struct TargetMachine {
    ptr: LLVMTargetMachineRef
}

impl_llvm_ptr_fmt!(TargetMachine);

impl Drop for TargetMachine {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeTargetMachine(self.ptr());
        }
    }
}

impl TargetMachine {
    pub unsafe fn from_ref(ptr: LLVMTargetMachineRef) -> TargetMachine {
        TargetMachine { ptr }
    }
    pub fn ptr(&self) -> LLVMTargetMachineRef {
        self.ptr
    }

    pub fn new(target: &Target,
               triple: &str,
               cpu: &str,
               features: &str,
               opt_level: LLVMCodeGenOptLevel,
               reloc_mode: LLVMRelocMode,
               code_model: LLVMCodeModel) -> TargetMachine {
        let triple_str = CString::new(triple).unwrap();
        let cpu_str = CString::new(cpu).unwrap();
        let features_str = CString::new(features).unwrap();
        unsafe {
            TargetMachine::from_ref(
                LLVMCreateTargetMachine(target.ptr(),
                                        triple_str.as_ptr(),
                                        cpu_str.as_ptr(),
                                        features_str.as_ptr(),
                                        opt_level,
                                        reloc_mode,
                                        code_model))
        }
    }

    pub fn native(opt_level: LLVMCodeGenOptLevel,
                  reloc_mode: LLVMRelocMode,
                  code_model: LLVMCodeModel) -> Result<TargetMachine, String> {
        let native_target = try!(Target::native());

        Ok(TargetMachine::new(&native_target,
                              &native_target_triple(),
                              &native_cpu_name(),
                              &native_cpu_features(),
                              opt_level,
                              reloc_mode,
                              code_model))
    }

    pub fn native_default() -> Result<TargetMachine, String> {
        TargetMachine::native(LLVMCodeGenOptLevel::LLVMCodeGenLevelDefault,
                              LLVMRelocMode::LLVMRelocDefault,
                              LLVMCodeModel::LLVMCodeModelJITDefault)
    }
}
