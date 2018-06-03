use parse::tests as parse_tests;

use ast::Unit;
use identify::{NameScopeBuilder, TypeScopeBuilder, ASTIdentifier};
use visit::visitor::UnitVisitor;
use check::ErrorCollector;

/// Identify an AST and return the compiler state.
pub fn identify(input: &'static str)
       -> (Unit, NameScopeBuilder, TypeScopeBuilder) {
    let mut parser = parse_tests::parser(input);
    let mut name_builder = NameScopeBuilder::new();
    let mut type_builder = TypeScopeBuilder::with_primitives();
    let mut errors = ErrorCollector::new();

    info!("Running parser");
    let unit = parse_tests::parser(input)
        .parse_unit()
        .expect("identify::tests::identify: Failed to parse input");
    info!("Running ASTIdentifer");
    ASTIdentifier::new(&mut name_builder, &mut type_builder, &mut errors)
        .visit_unit(&unit);

    assert!(errors.get_errors().is_empty(),
        "Errors during identification: {:?}", errors.get_errors());

    return (unit, name_builder, type_builder);
}


#[ignore]
#[test]
fn identify_example() {
    use std::fs::File;
    use std::io::Write;
    ::env_logger::Builder::new().parse("TRACE").init();

    let (unit, _, _) = identify(parse_tests::FACT_AND_HELPER);

    let mut file = File::create("/tmp/unit.rs").expect("Could not open file");
    write!(file, "{:#?}", unit).expect("Could not write file");
}
