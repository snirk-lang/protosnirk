//! ItemVisitor for mapping `TypeId`s to concrete types.

use ast::*;
use check::{CheckerError, ErrorCollector};
use identify::{ConcreteType, FnType, TypeBuilder};
use visit;
use visit::visitor::*;

/// Does the first pass of scope checking to ensure
/// items can be used before being declared.
#[derive(Debug, PartialEq)]
pub struct ItemTypeIdentifier<'err, 'builder> {
    builder: &'builder mut TypeBuilder,
    errors: &'err mut ErrorCollector,
    current_id: TypeId
}

impl<'err, 'builder> ItemTypeIdentifier<'err, 'builder> {
    pub fn new(errors: &'err mut ErrorCollector,
               builder: &'builder mut TypeBuilder)
               -> ItemTypeIdentifier<'err, 'builder> {
        ItemTypeIdentifier {
            errors: errors,
            builder: builder,
            current_id: TypeId::default()
        }
    }
}

impl<'err, 'builder> DefaultUnitVisitor
    for ItemTypeIdentifier<'err, 'builder> { }

impl<'err, 'builder> ItemVisitor for ItemTypeIdentifier<'err, 'builder> {
    fn visit_block_fn_decl(&mut self, fn_decl: &BlockFnDeclaration) {
        // Block functions don't explicitly have a FnTypeExpression
        // (unless they use first class functions in their arguments)
        // but are handled here.
        // This _could_ be done in the type graph
        let mut arg_names = Vec::with_capacity(fn_decl.get_params().len());

        for &(ref param_ident, ref param_ty_expr) in fn_decl.get_params() {
            self.visit_type_expr(param_ty_expr);
            let param_id = self.current_id;
            arg_names.push((param_ident.get_name(), param_id))
            // We have the IDS of the params. It's up to the type graph to map
            // them to the param idents later on.
        }
        // Grab the return type if it exists, or get the unary type id.
        let ret_ty = if let Some(ret_ty) = fn_decl.get_return_type() {
            self.visit_type_expr(ret_ty);
            self.builder.get(self.current_id)
                        .expect("Could not get just-defined type")
                        .clone()
        }
        else {
            self.builder.define_type(ConcreteType::Primitive(Primitive::Unary));
            ConcreteType::Primitive(Primitive::Unary)
        };
        let fn_args = arg_names.into_iter()
            .map(|(name, ty_id)| {
                    (name.into(),
                     self.builder.get(ty_id)
                                 .expect("Could not get just-defined type")
                                 .clone())
            })
            .collect::<Vec<_>>();

        let fn_concrete = ConcreteType::Function(FnType::new(fn_args, ret_ty));
        let foo_id = self.builder.define_type(fn_concrete);
        fn_decl.get_ident().set_type_id(foo_id);
    }
}

impl<'err, 'builder> TypeVisitor for ItemTypeIdentifier<'err, 'builder> {
    fn visit_named_type_expr(&mut self, named_ty: &NamedTypeExpression) {
        unreachable!("All named types are parsed as primitives");
    }
    fn visit_fn_type_expr(&mut self, fn_ty: &FnTypeExpression) {
        // These are not part of general function declarations
        unreachable!("Function types are not parsed");
    }

    fn visit_primitive_type_expr(&mut self, prim: &Primitive) {
        let concrete = ConcreteType::Primitive(*prim);
        self.builder.define_type(concrete);
    }
}
