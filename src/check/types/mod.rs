//! Definition of data types in a compiled protosnirk program.

mod inference_source;
pub use self::inference_source::InferenceSource;
mod type_graph;
pub use self::type_graph::TypeGraph;
mod item_identifier;
use self::item_identifier::ItemTypeIdentifier;
mod type_expr_identifier;
use self::type_expr_identifier::TypeExprIdentifier;
mod expr_identifier;
use self::expr_identifier::ExprTypeIdentifier;

use visit::visitor::UnitVisitor;
use check::ErrorCollector;

/// Infers the types of data on the AST.
#[derive(Debug, PartialEq)]
pub struct ASTTypeIdentifier<'builder, 'err> {
    builder: &'builder mut TypeScopeBuilder,
    errors: &'err mut ErrorCollector
}

impl<'builder, 'err> ASTTypeIdentifier<'builder, 'err> {
    pub fn new(builder: &'builder mut TypeScopeBuilder,
               errors: &'err mut ErrorCollector)
               -> ASTTypeIdentifier<'builder, 'err> {
        ASTTypeIdentifier { builder, errors }
    }
}

impl<'builder, 'err> UnitVisitor for ASTTypeIdentifier<'builder, 'err> {
    fn visit_unit(&mut self, unit: &Unit) {
        ItemTypeIdentifier::new(self.builder, self.errors).visit_unit(unit);
        ExprTypeIdentifier::new(self.builder, self.errors).visit_unit(unit);
    }
}
