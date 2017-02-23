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

        /*"let mut b = 0
        b += 1
        do
            let mut c = 0
            c = c + 1
            b *= c
        return b",
        "let a = 0 \n\
        let b = if a => 1 else 2\n\
        let c = b + 1 \n\
        return c",*/

r#"
let mut a = 0
if a < 1
    let b = 0
else if a > 1
    let b = 1
else
    let b = 2
return a"#,

r#"if 0 > 1
    let a = 0
return 1"#,

r#"if 1 > 0 => 1 else 0"#,

r#"
if 1 > 0
    let x = 0
    2
else if 2 < 3
    let y = 0
    4
//else if 1 + 2 > 4
//    let x = 1
//    5
else
    let x = 1
    4"#
    ];

    for (ix, input) in inputs.into_iter().enumerate() {
        trace!("Checking program {} - {:?}", ix, input);
        let name = format!("dump_basic_definitions_{}", ix);
        let compiler = create_module_compiler(input, &name, false);
        let (provider, _context, _symbols) = compiler.decompose();
        provider.get_module().dump();
    }
}
