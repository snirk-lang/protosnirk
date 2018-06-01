//! Builds the `TypeGraph` using code defined in items

use ast::*;
use visit;
use visit::visitor::{ItemVisitor, TypeVisitor, DefaultUnitVisitor};
use identify::{ConcreteType, TypeScopeBuilder};
use identify::types::{TypeGraph, InferenceSource};
use check::ErrorCollector;

/// Assigns `TypeId`s on items.
#[derive(Debug)]
pub struct ItemTypeChecker<'builder, 'err, 'graph> {
    builder: &'builder TypeScopeBuilder,
    errors: &'err mut ErrorCollector,
    graph: &'graph mut TypeGraph
}

impl<'builder, 'err, 'graph> ItemTypeChecker<'builder, 'err, 'graph> {
    pub fn new(builder: &'builder TypeScopeBuilder,
               errors: &'err mut ErrorCollector,
               graph: &'graph mut TypeGraph)
               -> ItemTypeChecker<'builder, 'err, 'graph> {
        ItemTypeChecker { builder, errors, graph }
    }
}
impl<'builder, 'err, 'graph> DefaultUnitVisitor
    for ItemTypeChecker<'builder, 'err, 'graph> { }

impl<'builder, 'err, 'graph> ItemVisitor
    for ItemTypeChecker<'builder, 'err, 'graph> {

    fn visit_block_fn_decl(&mut self, block_fn: &BlockFnDeclaration) {
        let fn_scope_id = block_fn.get_ident().get_id();
        if fn_scope_id.is_default() { return }

        // Block fn:
        // fn foo(x: Type, y: Type2) -> Rettype
        //     (block)

        // x.type_id = tx
        // tx = tType
        // y.type_id = ty
        // ty = tType

        let fn_ty_ix = self.graph.get_type(&fn_scope_id)
                           .expect("Unidentified block fn");

        // This check is done during this phase because the identify phase
        // does not have the type graph.

        for &(ref param_ident, ref param_ty_expr) in block_fn.get_params() {
            // t_param = t_param_expr

            let param_ty_id = param_ty_expr.get_id();
            // Stop if identify phase did not identify parameter type
            if param_ty_id.is_default() { return }

            let param_var_id = param_ident.get_id();
            if param_var_id.is_default() { return }

            let param_ty_ix = self.graph.get_type(&param_ty_id)
                .expect("Function had unknown argument type");
            let param_var_ix = self.graph.add_variable(param_var_id.clone());

            self.graph.add_inference(param_var_ix, param_ty_ix,
                InferenceSource::FnParameter(block_fn.get_ident().clone()));
        }

        // Don't need to explicitly add the return type to the graph.
    }
}
