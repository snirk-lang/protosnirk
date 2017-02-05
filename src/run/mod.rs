//! Contains the runtime for protosnirk

mod jit;
mod llvm_state;

pub use self::llvm_state::LLVMState;
pub use self::jit::LLVMJIT;
