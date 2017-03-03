use parse::tests::parser;
use parse::{ErrorCollector, SymbolTable, ASTVisitor};
use compile::{ModuleProvider, ModuleCompiler, SimpleModuleProvider};

pub fn create_module_compiler(input: &'static str, name: &str, optimize: bool)
        -> ModuleCompiler<SimpleModuleProvider> {
    let mut parser = parser(input);
    let program = parser.parse_unit()
        .expect("Could not parse program");
    let (block, table, _errors) = program.decompose();
    let module_provider = SimpleModuleProvider::new(name, optimize);
    let mut compiler = ModuleCompiler::new(table, module_provider, optimize);
    compiler.check_unit(&block);
    compiler
}

#[test]
fn compile_basic_programs() {
    let inputs = &[
r#"
fn fact(n)
    if n == 0
        0
    else
        1
"#
    ];

    for (ix, input) in inputs.into_iter().enumerate() {
        trace!("Checking program {} - {:?}", ix, input);
        let name = format!("dump_basic_definitions_{}", ix);
        let compiler = create_module_compiler(input, &name, false);
        let (provider, _context, _symbols) = compiler.decompose();
        provider.get_module().dump();
    }
}
