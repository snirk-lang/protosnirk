use parse::ScopedId;
use parse::ast::*;

use check::{CheckerError, ErrorCollector};
use check::visitor::*;
use check::scope::NameScopeBuilder;

/// Does the first pass of scope checking to ensure
/// items can be used before being declared.
#[derive(Debug, PartialEq, Clone)]
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
    fn visit_inline_fn_decl(&mut self, fn_decl: &mut InlineFnDeclaration) {
        // Don't need to look at param name idents.
        visit::walk_inline_fn_type(self, fn_decl.get_type_expr());
    }

    fn visit_block_fn_decl(&mut self, fn_decl: &mut BlockFnDeclaration) {
        // Don't need to look at param name idents.
        visit::walk_fn_type(self, fn_decl.get_type_expr());
    }
}

impl<'err, 'builder> TypeVisitor for ItemTypeIdentifier<'err, 'builder> {
    fn visit_named_type_expr(&mut self, named_ty: &NamedTypeExpression) {
        // There shouldn't be unrecognized named types right now.
        if let Some(id) = self.builder.get(named_ty.get_name()) {
            named_ty.get_ident().set_id(id.clone());
        }
        else {
            trace!("Encountered unexpected type name {}", named_ty.get_name());
            let err_text = format!("Unknown type {}", named_ty.get_name());
            self.errors.add_error(CheckerError::new(
                named_ty.get_ident().get_token().clone(), vec![], err_text
            ))
        }
    }
    fn visit_fn_type_expr(&mut self, fn_ty: &FnTypeExpression) {
        // Skipping over this type by `visit`ing while handling the item itself
        unreachable!()
    }
    fn visit_inline_fn_ty_expr(&mut self,
                               inline_fn_ty: &InlineFnTypeExpression) {
        // Skipping over this type by `visit`ing while handling the item itself
        unreachable!()
    }
}
