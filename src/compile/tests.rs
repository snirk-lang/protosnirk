
use std::collections::HashMap;

use parse::ScopeIndex;
use parse::tests::parser;
use parse::{ErrorCollector, SymbolTable, ASTVisitor};
use compile::{ModuleProvider, ModuleCompiler, SimpleModuleProvider};

use llvm::{Context, Module, Builder, Value};

macro_rules! llvm_example {
    ($name:ident => $input:expr) => {
        #[test]
        fn $name() {/*
            ::env_logger::Builder::new()
                .filter(None, ::log::LevelFilter::Trace)
                .init();*/

            let mut parser = parser($input);
            let program = parser.parse_unit()
                .expect("Could not parse program");
            let (block, table, _errors) = program.decompose();
            let context = Context::new();
            let module = context.new_module(stringify!($name));
            {
                let builder = Builder::new(&context);
                let mut named = HashMap::new();
                let mut ir_code = Vec::new();
                let mut scopes = HashMap::new();

                {
                    let module_provider = SimpleModuleProvider::new(module, false);
                    let mut compiler = ModuleCompiler::new(table,
                        module_provider,
                        &context,
                        &builder,
                        &mut named,
                        &mut ir_code,
                        &mut scopes,
                        false);
                    compiler.check_unit(&block);

                    let (_provider, _context, _symbols) = compiler.decompose();
                    //provider.get_module().dump();
                }
            }
        }
    }
}

llvm_example! {
    fact =>
r#"
fn factHelper(n, acc)
    if n <= 2
        acc
    else
        factHelper(n: n - 1, acc: acc * n)
fn fact(n)
    factHelper(n, acc: 1)
"#
}
