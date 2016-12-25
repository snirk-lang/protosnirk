use iron_llvm::core::Module;

pub trait ModuleProvider {
    fn get_module(&self) -> &Module;
    fn get_module_mut(&mut self) -> &mut Module;
}

pub struct SimpleModuleProvider {
    module: Module
}

impl ModuleProvider for SimpleModuleProvider {
    fn get_module(&self) -> &Module {
        &self.module
    }
    fn get_module_mut(&mut self) -> &mut Module {
        &mut self.module
    }
}
