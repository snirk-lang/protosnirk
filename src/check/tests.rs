use parse::tests as parse_tests;
use identify::tests as identify_tests;

use ast::Unit;
use identify::{NameScopeBuilder, TypeScopeBuilder, ASTIdentifier, TypeGraph};
use visit::visitor::UnitVisitor;
use check::{ErrorCollector, TypeConcretifier, TypeMapping};

/// Check an AST and return the compiler state.
pub fn check(input: &'static str)
            -> (Unit, ErrorCollector,
                NameScopeBuilder, TypeScopeBuilder,
                TypeGraph, TypeMapping) {

    let (unit, mut errors, name_builder, type_builder, mut graph)
        = identify_tests::identify(input);

    let results = {
        debug!("Calling TypeConcretifier");
        let mut tc = TypeConcretifier::new(&type_builder, &mut errors, &mut graph);
        tc.visit_unit(&unit);
        tc.into_results()
    };

    (unit, errors, name_builder, type_builder, graph, results)
}

pub const CHECK_EXAMPLE: &'static str = r#"
/// Computes the nth fibonacci number.
fn fib(n: float) -> float
    if n < 0 => n else (if n <= 2 => 1 else n + fib(n - 1))
"#;

#[ignore]
#[test]
fn check_example() {
    use identify::*;
    use check::ErrorCollector;

    ::env_logger::Builder::new().parse("TRACE").init();

    let (.., graph, results) = check(parse_tests::FACT_AND_HELPER);
    graph.write_svg("/tmp/checked-graph.svg");

    info!("Got result types: {:#?}", results);
}
