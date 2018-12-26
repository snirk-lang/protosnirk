mod type_identifier;
use self::type_identifier::TypeIdentifier;

mod item_namer;
mod expr_namer;

pub use self::item_namer::ItemTypeIdentifier;
pub use self::expr_namer::ExprTypeIdentifier;

mod inference_source;
pub use self::inference_source::InferenceSource;
mod type_graph;
pub use self::type_graph::*; // This is just `TypeGraph` to other modules, but
// includes the full def of the graph for use in `identify/tests`.
mod item_typographer;
use self::item_typographer::ItemTypographer;
mod expr_typographer;
use self::expr_typographer::ExprTypographer;

use ast::{Unit, visit::UnitVisitor};
use identify::TypeScopeBuilder;
use check::ErrorCollector;

/// Infers the types of data on the AST.
#[derive(Debug)]
pub struct ASTTypeChecker<'builder, 'graph, 'err> {
    builder: &'builder mut TypeScopeBuilder,
    graph: &'graph mut TypeGraph,
    errors: &'err mut ErrorCollector
}

impl<'builder, 'graph, 'err> ASTTypeChecker<'builder, 'graph, 'err> {
    pub fn new(builder: &'builder mut TypeScopeBuilder,
               graph: &'graph mut TypeGraph,
               errors: &'err mut ErrorCollector)
               -> ASTTypeChecker<'builder, 'graph, 'err> {
        ASTTypeChecker { builder, graph, errors }
    }
}

impl<'builder, 'graph, 'err> UnitVisitor
    for ASTTypeChecker<'builder, 'graph, 'err> {

    fn visit_unit(&mut self, unit: &Unit) {
        trace!("Visting unit");
        debug!("Calling ItemTypeIdentifier");
        ItemTypeIdentifier::new(self.errors, self.builder)
                           .visit_unit(unit);
        debug!("Calling ItemTypographer");
        ItemTypographer::new(self.builder, self.errors, self.graph)
                        .visit_unit(unit);
        debug!("Calling ExprTypeIdentifier");
        ExprTypeIdentifier::new(self.errors, self.builder)
                        .visit_unit(unit);
        debug!("Calling ExprTypographer");
        ExprTypographer::new(self.builder, self.errors, self.graph)
                        .visit_unit(unit);
    }
}
