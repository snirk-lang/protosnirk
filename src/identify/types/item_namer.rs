//! ItemVisitor for mapping `TypeId`s to concrete types.

use ast::{*, visit::*};
use check::{CheckerError, ErrorCollector};
use identify::{ConcreteType, FnType, TypeScopeBuilder};
use identify::types::TypeIdentifier;

/// Identifies type expressions in items, such as function parameters and
/// function names as concrete function types.
#[derive(Debug, PartialEq)]
pub struct ItemTypeIdentifier<'err, 'builder> {
    errors: &'err mut ErrorCollector,
    builder: &'builder mut TypeScopeBuilder,
}

impl<'err, 'builder> ItemTypeIdentifier<'err, 'builder> {
    pub fn new(errors: &'err mut ErrorCollector,
               builder: &'builder mut TypeScopeBuilder)
               -> ItemTypeIdentifier<'err, 'builder> {
        ItemTypeIdentifier { errors, builder }
    }
}

impl<'err, 'builder> UnitVisitor for ItemTypeIdentifier<'err, 'builder> {
    fn visit_unit(&mut self, unit: &Unit) {
        trace!("Visting a unit");
        visit::walk_unit(self, unit);
    }
}

impl<'err, 'builder> ItemVisitor for ItemTypeIdentifier<'err, 'builder> {
    fn visit_block_fn_decl(&mut self, fn_decl: &BlockFnDeclaration) {
        trace!("Visiting block fn declaration");
        if fn_decl.id().is_default() {
            debug!("Skipping fn {} with default ID", fn_decl.name());
            return
        }

        // Declared functions' types are handled here because we do not want
        // to run full type inference at the item level.
        let mut arg_types = Vec::with_capacity(fn_decl.params().len());

        for &(ref param_ident, ref param_ty_expr) in fn_decl.params() {
            TypeIdentifier::new(self.errors, self.builder)
                           .visit_type_expr(param_ty_expr);
            // Stop if we can't idenify a parameter type.
            if param_ty_expr.id().is_default() {
                return
            }
            let param_ty = self.builder.get_type(&param_ty_expr.id())
                .expect("TypeIdentifier did not update param's type ID");
            arg_types.push((param_ident.name().to_string(),
                            param_ty.clone()));
        }
        let return_ty = fn_decl.return_type();
        TypeIdentifier::new(self.errors, self.builder)
                       .visit_type_expr(return_ty);

        if return_ty.id().is_default() { return }
        let ret_ty = self.builder.get_type(&return_ty.id())
            .expect("TypeIdentifier did not update param's type ID")
            .clone();

        let fn_concrete = ConcreteType::Function(
            FnType::new(arg_types, ret_ty));
        self.builder.add_type(fn_decl.id().clone(), fn_concrete);
    }
}
