//! Definition of data types in a compiled protosnirk program.

mod inference_source;
pub use self::inference_source::InferenceSource;
mod type_graph;
pub use self::type_graph::TypeGraph;
mod item_checker;
use self::item_checker::ItemTypeChecker;
mod expr_checker;
use self::expr_checker::ExprTypeChecker;

use ast::Unit;
use visit::visitor::UnitVisitor;
use identify::TypeScopeBuilder;
use check::ErrorCollector;

/// Infers the types of data on the AST.
#[derive(Debug)]
pub struct ASTTypeChecker<'builder, 'err, 'graph> {
    builder: &'builder mut TypeScopeBuilder,
    graph: &'graph mut TypeGraph,
    errors: &'err mut ErrorCollector
}

impl<'builder, 'err, 'graph> ASTTypeChecker<'builder, 'err, 'graph> {
    pub fn new(builder: &'builder mut TypeScopeBuilder,
               errors: &'err mut ErrorCollector,
               graph: &'graph mut TypeGraph)
               -> ASTTypeChecker<'builder, 'err, 'graph> {
        ASTTypeChecker { builder, errors, graph }
    }
}

impl<'builder, 'err, 'graph> UnitVisitor for ASTTypeChecker<'builder, 'err, 'graph> {
    fn visit_unit(&mut self, unit: &Unit) {
        ItemTypeChecker::new(self.builder, self.errors, self.graph)
                        .visit_unit(unit);
        ExprTypeChecker::new(self.builder, self.errors, self.graph)
                        .visit_unit(unit);
    }
}
