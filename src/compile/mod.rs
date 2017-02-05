mod context;
mod module_compiler;
mod module_provider;

#[cfg(test)]
mod tests;

pub use self::module_provider::{ModuleProvider, SimpleModuleProvider};
pub use self::module_compiler::ModuleCompiler;
pub use self::context::LLVMContext;
