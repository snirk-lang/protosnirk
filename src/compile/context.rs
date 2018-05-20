use std::collections::HashMap;

use compile::ModuleProvider;

use llvm::{Context, Module, Value, Builder};

/// Contains structures pertaining to the LLVM's
/// Context object.
pub struct LLVMContext<'ctx, 'b> where 'ctx: 'b {
    context: &'ctx Context,
    builder: &'ctx Builder<'ctx>,
    named_values: &'b HashMap<String, Value<'ctx>>
}

impl<'ctx, 'b> LLVMContext<'ctx, 'b> {
    /// Creates a new LLVMContext with llvm's
    /// global Context
    pub fn new(context: &'ctx Context,
               builder: &'ctx Builder<'ctx>,
               named_values: &'b HashMap<String, Value<'ctx>>)
               -> LLVMContext<'ctx, 'b> {
        LLVMContext { context, builder, named_values }
    }

    pub fn context(&self) -> &'ctx Context {
        &self.context
    }

    /// Gets the IR builder of this context
    pub fn builder(&self) -> &'ctx Builder<'ctx> {
        &self.builder
    }
}
