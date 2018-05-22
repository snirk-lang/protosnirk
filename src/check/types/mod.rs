//! Definition of data types in a compiled protosnirk program.

mod inference_source;
pub use self::inference_source::InferenceSource;
mod type_graph;
pub use self::type_graph::TypeGraph;
mod type_expr_checker;
use self::type_expr_checker::TypeExprChecker;
mod item_checker;
use self::item_checker::ItemTypeChecker;
mod expr_checker;
use self::expr_checker::ExprTypeChecker;

use ast::Unit;
use visit::visitor::UnitVisitor;
use identify::TypeScopeBuilder;
use check::ErrorCollector;

/// Infers the types of data on the AST.
#[derive(Debug, PartialEq)]
pub struct ASTTypeChecker<'builder, 'err> {
    builder: &'builder mut TypeScopeBuilder,
    errors: &'err mut ErrorCollector
}

impl<'builder, 'err> ASTTypeChecker<'builder, 'err> {
    pub fn new(builder: &'builder mut TypeScopeBuilder,
               errors: &'err mut ErrorCollector)
               -> ASTTypeChecker<'builder, 'err> {
        ASTTypeChecker { builder, errors }
    }
}

impl<'builder, 'err> UnitVisitor for ASTTypeChecker<'builder, 'err> {
    fn visit_unit(&mut self, unit: &Unit) {
        ItemTypeChecker::new(self.builder, self.errors).visit_unit(unit);
        ExprTypeChecker::new(self.builder, self.errors).visit_unit(unit);
    }
}
