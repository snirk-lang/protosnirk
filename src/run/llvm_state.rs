use iron_llvm::execution_engine::{ExecutionEngine};
use iron_llvm::execution_engine::execution_engine::FrozenModule;

/// LLVM JIT is done by compiling a module per line of input.
/// In order to provide a JIT we need to keep track of all the
/// modules and also provide some symbol resolution.
#[derive(Default)]
pub struct LLVMState {
    pub execution_engines: Vec<ExecutionEngine>,
    pub modules: Vec<FrozenModule>
}

impl LLVMState {
    #[inline]
    pub fn new() -> LLVMState {
        LLVMState::default()
    }
    pub fn get_fn_address(&self, name: &str) -> u64 {
        for engine in &self.execution_engines {
            let addr = engine.get_function_address(name);
            if addr != 0  {
                return addr
            }
        }

        0
    }
}
