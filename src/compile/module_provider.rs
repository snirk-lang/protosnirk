use llvm::{self, Module, FunctionPassManager, TargetData};
use llvm_sys::target_machine::{LLVMCodeGenOptLevel, LLVMRelocMode, LLVMCodeModel};

use std::fmt;

pub trait ModuleProvider<'ctx> {
    fn module(&self) -> &Module<'ctx>;
    fn pass_manager(&mut self) -> &FunctionPassManager;
}

pub struct SimpleModuleProvider<'ctx> {
    module: Module<'ctx>,
    fn_pass_manager: FunctionPassManager,
}
impl<'ctx> SimpleModuleProvider<'ctx> {
    pub fn new(module: Module<'ctx>, optimizations: bool) -> SimpleModuleProvider<'ctx> {

        llvm::initialize_native_target();
        llvm::initialize_
        let layout = TargetData::native(LLVMCodeGenOptLevel::LLVMCodeGenLevelDefault,
                                        LLVMRelocMode::LLVMRelocDefault,
                                        LLVMCodeModel::LLVMCodeModelDefault);
        if let Err(message) = layout {
            panic!("Unable to initialize native target data: {}", message);
        }
        else if let Ok(layout) = layout {
            module.set_data_layout(&layout);
            module.set_target_triple(&llvm::target::native_target_triple());
        }

        let pass_manager = FunctionPassManager::new(&module);
        if optimizations {
            pass_manager.add_basic_alias_analysis_pass();
            pass_manager.add_instruction_combining_pass();
            pass_manager.add_reassociate_pass();
            pass_manager.add_gvn_pass();
            pass_manager.add_cfg_simplification_pass();
            assert!(pass_manager.initialize());
        }
        SimpleModuleProvider {
            module: module,
            fn_pass_manager: pass_manager
        }
    }
}

impl<'ctx> ModuleProvider<'ctx> for SimpleModuleProvider<'ctx> {
    fn module(&self) -> &Module<'ctx> {
        &self.module
    }
    fn pass_manager(&mut self) -> &FunctionPassManager {
        &mut self.fn_pass_manager
    }
}

impl<'ctx> fmt::Debug for SimpleModuleProvider<'ctx> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SimpleModuleProvider()")
    }
}
