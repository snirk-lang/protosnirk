use iron_llvm::core::{Module, FunctionPassManager};

pub trait ModuleProvider {
    fn get_module(&self) -> &Module;
    fn get_module_mut(&mut self) -> &mut Module;
    fn get_pass_manager(&mut self) -> &mut FunctionPassManager;
}

pub struct SimpleModuleProvider {
    module: Module,
    fn_pass_manager: FunctionPassManager,
}
impl SimpleModuleProvider {
    pub fn new(name: &str, optimizations: bool) -> SimpleModuleProvider {
        let module = Module::new(name);
        let mut pass_manager = FunctionPassManager::new(&module);
        if optimizations {
            pass_manager.add_basic_alias_analysis_pass();
            pass_manager.add_instruction_combining_pass();
            pass_manager.add_reassociate_pass();
            pass_manager.add_GVN_pass();
            pass_manager.add_CFG_simplification_pass();
            pass_manager.initialize();
        }
        SimpleModuleProvider {
            module: module,
            fn_pass_manager: pass_manager
        }
    }
}

impl ModuleProvider for SimpleModuleProvider {
    fn get_module(&self) -> &Module {
        &self.module
    }
    fn get_module_mut(&mut self) -> &mut Module {
        &mut self.module
    }
    fn get_pass_manager(&mut self) -> &mut FunctionPassManager {
        &mut self.fn_pass_manager
    }
}
