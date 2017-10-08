use parse::ScopedId;
use parse::ast::*;

use check::{CheckerError, ErrorCollector};
use identify::NameScopeBuilder;
use visit;
use visit::visitor::*;

/// Does the first pass of scope checking to ensure
/// items can be used before being declared.
#[derive(Debug, PartialEq)]
pub struct ItemTypeIdentifier<'err, 'builder> {
    builder: &'builder mut NameScopeBuilder,
    errors: &'err mut ErrorCollector,
    // There aren't any new type declarations yet :|
    //current_id: ScopedId
}

impl<'err, 'builder> ItemTypeIdentifier<'err, 'builder> {
    pub fn new(errors: &'err mut ErrorCollector,
               builder: &'builder mut NameScopeBuilder)
               -> ItemTypeIdentifier<'err, 'builder> {
        ItemTypeIdentifier {
            errors: errors,
            builder: builder,
            //current_id: ScopedId::default()
        }
    }
}

impl<'err, 'builder> DefaultUnitVisitor
    for ItemTypeIdentifier<'err, 'builder> { }

impl<'err, 'builder> ItemVisitor for ItemTypeIdentifier<'err, 'builder> {
    fn visit_block_fn_decl(&mut self, fn_decl: &BlockFnDeclaration) {
        // Don't need to look at param name idents.
        for &(_,ref param_ty) in fn_decl.get_params() {
            self.visit_type_expr(param_ty);
        }
        if let Some(ret_type) = fn_decl.get_return_type() {
            self.visit_type_expr(ret_type);
        }
    }
}

impl<'err, 'builder> TypeVisitor for ItemTypeIdentifier<'err, 'builder> {
    fn visit_named_type_expr(&mut self, named_ty: &NamedTypeExpression) {
        // There shouldn't be unrecognized named types right now.
        if let Some(id) = self.builder.get(named_ty.get_ident().get_name()) {
            named_ty.get_ident().set_id(id.clone());
        }
        else {
            trace!("Encountered unexpected type name {:?}", named_ty.get_ident());
            let err_text = format!("Unknown type {}",
                named_ty.get_ident().get_name());
            self.errors.add_error(CheckerError::new(
                named_ty.get_ident().get_token().clone(), vec![], err_text
            ))
        }
    }
    fn visit_fn_type_expr(&mut self, fn_ty: &FnTypeExpression) {
        // Skipping over this type by `visit`ing while handling the item itself
        unreachable!()
    }

    fn visit_primitive_type_expr(&mut self, prim: &Primitive) {

    }
}
