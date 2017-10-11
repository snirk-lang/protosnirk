//! Assign the `TypeId`s of `Identifier`s in the AST.
//!
//! This AST pass is responsible for assigning the `TypeId`s of the AST, which
//! will later be used for type inference equations and then mapped to
//! `ConcreteType`s.

mod type_equation_builder;
mod type_expr_identifier;
mod item_identifier;
mod expr_identifier;

use self::type_expr_identifier::TypeExprIdentifier;
use self::item_identifier::ItemTypeIdentifier;
use self::expr_identifier::ExprTypeIdentifier;
use self::type_equation_builder::TypeEquationBuilder;

use parse::ast::Unit;
use check::ErrorCollector;
use visit::visitor::UnitVisitor;

/// Identifies `TypeId`s on the AST.
#[derive(Debug, PartialEq)]
pub struct ASTTypeIdentifier<'builder, 'err> {
    builder: &'builder mut TypeEquationBuilder,
    errors: &'err mut ErrorCollector
}

impl<'builder, 'err> ASTTypeIdentifier<'builder, 'err> {
    pub fn new(builder: &'builder mut TypeEquationBuilder,
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
