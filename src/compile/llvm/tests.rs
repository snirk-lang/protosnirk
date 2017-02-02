use parse::tests::parser;
use parse::{ErrorCollector, SymbolTable, ASTVisitor};
use compile::llvm::{ModuleProvider, ModuleCompiler, SimpleModuleProvider};

pub fn create_module_compiler(input: &'static str, name: &str, optimize: bool)
        -> ModuleCompiler<SimpleModuleProvider> {
    let mut parser = parser(input);
    let program = parser.parse_unit()
        .expect("Could not parse program");
    let (block, table, _consts, _errors) = program.decompose();
    let module_provider = SimpleModuleProvider::new(name);
    let mut compiler = ModuleCompiler::new(table, module_provider, optimize);
    compiler.check_unit(&block);
    compiler
}

#[test]
fn compile_basic_programs() {
    ::env_logger::LogBuilder::new()
        .parse("TRACE")
        .init()
        .unwrap();
    let inputs = &[
        "1",
        "let a = 0 a",
        "let a = 0 return a",
        "let a = 0 a + 1",
        "let mut a = 0 a + 1",

        "let mut a = 0 \n\
        let b = a + 1 \n\
        a = a + b \n\
        a = a + 1 \n\
        a = a % 2 \n\
        a = a * 2 \n\
        return a",

        "let mut b = 0\n\
        b += 1\n\
        do \n\
            let mut c = 0
            c = c + 1
        ",
    ];
    for (ix, input) in inputs.into_iter().enumerate() {
        println!("Checking program {}", ix);
        let name = format!("dump_basic_definitions_{}", ix);
        let compiler = create_module_compiler(input, &name, false);
        let (_provider, _context, _symbols) = compiler.decompose();
    }
}
