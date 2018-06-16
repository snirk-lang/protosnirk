use ast::ScopedId;
use visit::visitor::UnitVisitor;
use parse::tests::parser;
use compile::{ModuleProvider, ModuleCompiler, SimpleModuleProvider};
use llvm::{Context, Module, Builder, Value};

use parse::tests as parse_tests;
use check::tests as check_tests;

use std::collections::HashMap;

/// Compile a program and return the LLVM module
pub fn compile<'ctx>(input: &'static str,
                     name: &'static str,
                     context: &'ctx Context)
                     -> SimpleModuleProvider<'ctx> {
    info!("Running checker");
    let (unit, _errs, _names, _types, _tygraph, tymap) = check_tests::check(input);
    info!("Got types: {:#?}", tymap);
    info!("Creating LLVM contexts");
    let module = context.new_module(name);
    {
        let builder = Builder::new(&context);
        let mut ir_code = Vec::new();
        let mut scopes = HashMap::new();
        {
            let module_provider = SimpleModuleProvider::new(module, false);
            let mut compiler = ModuleCompiler::new(tymap,
                module_provider,
                &context,
                &builder,
                &mut ir_code,
                &mut scopes,
                false);
            info!("Running compiler");
            compiler.visit_unit(&unit);

            let (provider, _types) = compiler.decompose();
            provider
        }
    }
}

#[ignore]
#[test]
fn compile_example() {
    ::env_logger::Builder::new().parse("TRACE").init();

    {
        let context = Context::new();
        let module_provider = compile(
                parse_tests::BLOCKS_IN_BLOCKS,
                "FACT_HELPER",
                &context);

        module_provider.get_module().dump();
    }
}
