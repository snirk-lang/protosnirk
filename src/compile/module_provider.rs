use llvm::{Module, FunctionPassManager};

pub trait ModuleProvider<'ctx> {
    fn get_module(&self) -> &Module<'ctx>;
    fn get_pass_manager(&mut self) -> &FunctionPassManager;
}

pub struct SimpleModuleProvider<'ctx> {
    module: Module<'ctx>,
    fn_pass_manager: FunctionPassManager,
}
impl<'ctx> SimpleModuleProvider<'ctx> {
    pub fn new(module: Module<'ctx>, optimizations: bool) -> SimpleModuleProvider<'ctx> {
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
    fn get_module(&self) -> &Module<'ctx> {
        &self.module
    }
    fn get_pass_manager(&mut self) -> &FunctionPassManager {
        &mut self.fn_pass_manager
    }
}
