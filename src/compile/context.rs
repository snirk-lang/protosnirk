use std::collections::HashMap;

use compile::ModuleProvider;

use iron_llvm::LLVMRef;
use iron_llvm::core::{Context, Builder, Function};
use iron_llvm::core::types::{RealTypeRef, FunctionTypeRef};
use iron_llvm::core::types::{RealTypeCtor, FunctionTypeCtor};
use iron_llvm::core::value::{FunctionRef, FunctionCtor};
use llvm_sys::prelude::LLVMValueRef;

/// Contains structures pertaining to the LLVM's
/// Context object.
pub struct LLVMContext {
    context: Context,
    builder: Builder,
    named_values: HashMap<String, LLVMValueRef>
}

impl LLVMContext {
    /// Creates a new LLVMContext with llvm's
    /// global Context
    pub fn new() -> LLVMContext {
        let global_context = Context::get_global();
        let builder = Builder::new();
        let named_values = HashMap::new();

        LLVMContext {
            context: global_context,
            builder: builder,
            named_values: named_values,
        }
    }
    /// Gets the LLVM context object
    pub fn global_context(&self) -> &Context {
        &self.context
    }
    pub fn global_context_mut(&mut self) -> &mut Context {
        &mut self.context
    }
    /// Gets the IR builder of this context
    pub fn builder(&self) -> &Builder {
        &self.builder
    }

    pub fn builder_mut(&mut self) -> &mut Builder {
        &mut self.builder
    }

}
