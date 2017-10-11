//! Identify type expressions.

use parse::{ScopedId, TypeId};
use parse::ast::types::*;
use check::{ErrorCollector};
use visit::visitor::TypeVisitor;
use visit::*;
use typeinfer::{ConcreteType, InferredType};
use typeinfer::identify::TypeEquationBuilder;

#[derive(Debug, PartialEq)]
pub struct TypeExprIdentifier<'err, 'builder> {
    errors: &'err mut ErrorCollector,
    builder: &'builder mut TypeEquationBuilder,
    matched_type: Option<InferredType>
}
impl<'err, 'builder> TypeExprIdentifier<'err, 'builder> {
    pub fn new(builder: &'builder mut TypeEquationBuilder,
               errors: &'err mut ErrorCollector)
               -> TypeExprIdentifier<'err, 'builder> {
        TypeExprIdentifier { builder, errors, matched_type: None }
    }

    /// Get the matched `TypeId` fomr the TypeExpression
    pub fn get_type(self) -> InferredType {
        self.matched_type
        .expect("Did not create an InferredType from visiting a type Expression")
    }
}

impl<'err, 'builder> TypeVisitor for TypeExprIdentifier<'err, 'builder> {
    fn visit_named_type_expr(&mut self, named_ty: &NamedTypeExpression) {
        panic!("Currently only using primitive types");
    }

    fn visit_fn_type_expr(&mut self, fn_ty: &FnTypeExpression) {
        panic!("TypeExprIdentifier cannot visit FnTypeExpression as first-\
                class functions are not a thing yet.");
    }

    fn visit_primitive_type_expr(&mut self, prim: &Primitive) {
        self.matched_type = Some(
            InferredType::Known(ConcreteType::Primitive(*prim)));
    }
}
