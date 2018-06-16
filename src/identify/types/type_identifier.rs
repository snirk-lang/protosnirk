//! Visitor which walks through a TypeExpression to assign its
//! `ScopedId`.

use ast::{ScopedId, visit::*};
use ast::types::*;
use check::{CheckerError, ErrorCollector};
use identify::TypeScopeBuilder;

/// Visitor which identifies TypeExpressions,
/// by assigning their IDs to those found in
/// a `TypeScopeBuilder`.
#[derive(Debug)]
pub struct TypeIdentifier<'err, 'builder> {
    errors: &'err mut ErrorCollector,
    /// New types cannot be defined yet.
    builder: &'builder TypeScopeBuilder,
}

impl<'err, 'builder> TypeIdentifier<'err, 'builder> {
    pub fn new(errors: &'err mut ErrorCollector,
               builder: &'builder TypeScopeBuilder)
               -> TypeIdentifier<'err, 'builder> {
        TypeIdentifier { errors, builder }
    }
}

impl<'err, 'builder> TypeVisitor for TypeIdentifier<'err, 'builder> {
    fn visit_named_type_expr(&mut self, named_ty: &NamedTypeExpression) {
        if let Some(type_id) =
            self.builder.get_named_type_id(named_ty.get_name()) {
            // Found the already defined type.
            named_ty.set_id(type_id.clone());
        }
        else {
            self.errors.add_error(CheckerError::new(
                named_ty.get_ident().get_token().clone(),
                vec![],
                format!("Unknown type {}", named_ty.get_name())
            ));
        }
    }
}
