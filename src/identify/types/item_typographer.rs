//! Builds the `TypeGraph` using code defined in items

use ast::*;
use visit;
use visit::visitor::{ItemVisitor, TypeVisitor, DefaultUnitVisitor};
use identify::{ConcreteType, TypeScopeBuilder};
use identify::types::{TypeGraph, InferenceSource};
use check::ErrorCollector;

/// Assigns `TypeId`s on items.
#[derive(Debug)]
pub struct ItemTypographer<'builder, 'err, 'graph> {
    builder: &'builder TypeScopeBuilder,
    errors: &'err mut ErrorCollector,
    graph: &'graph mut TypeGraph
}

impl<'builder, 'err, 'graph> ItemTypographer<'builder, 'err, 'graph> {
    pub fn new(builder: &'builder TypeScopeBuilder,
               errors: &'err mut ErrorCollector,
               graph: &'graph mut TypeGraph)
               -> ItemTypographer<'builder, 'err, 'graph> {
        ItemTypographer { builder, errors, graph }
    }
}
impl<'builder, 'err, 'graph> DefaultUnitVisitor
    for ItemTypographer<'builder, 'err, 'graph> { }

impl<'builder, 'err, 'graph> ItemVisitor
    for ItemTypographer<'builder, 'err, 'graph> {

    fn visit_block_fn_decl(&mut self, block_fn: &BlockFnDeclaration) {
        trace!("Visiting block fn {}", block_fn.get_name());
        let fn_scope_id = block_fn.get_ident().get_id();
        if fn_scope_id.is_default() {
            debug!("Ignoring unnamed fn {}", block_fn.get_name());
            return
        }

        match self.builder.get_type(&fn_scope_id) {
            Some(_ty) => {
                trace!("Adding type of fn {} to graph",
                    block_fn.get_name());
                self.graph.add_type(fn_scope_id.clone())
            }
            None => {
                debug!("Ignoring unknown type fn {}", block_fn.get_name());
                return
            }
        };

        // Block fn:
        // fn foo(x: Type, y: Type2) -> Rettype
        //     (block)

        // x.type_id = tx
        // tx = tType
        // y.type_id = ty
        // ty = tType

        // This check is done during this phase because the identify phase
        // does not have the type graph.

        for &(ref param_ident, ref param_ty_expr) in block_fn.get_params() {
            trace!("Checking fn {} param {}",
                block_fn.get_name(), param_ident.get_name());
            // t_param = t_param_expr

            let param_ty_id = param_ty_expr.get_id();
            // Stop if identify phase did not identify parameter type
            if param_ty_id.is_default() {
                debug!("Ignoring fn {}, unknown type of param {}",
                    block_fn.get_name(), param_ident.get_name());
                return
            }

            let param_var_id = param_ident.get_id();
            if param_var_id.is_default() {
                debug!("Ignoring fn {}, unknown param {}",
                    block_fn.get_name(), param_ident.get_name());
                return
            }

            let param_ty_ix = match self.builder.get_type(&param_ty_id) {
                Some(_ty) => {
                    trace!("Ensuring type of fn {} param {} in graph",
                        block_fn.get_name(), param_ident.get_name());
                    self.graph.add_type(param_ty_id.clone())
                },
                None => {
                    debug!("Ignoring fn {}, unknown type of param {}",
                        block_fn.get_name(), param_ident.get_name());
                    return
                }
            };

            let param_var_ix = self.graph.add_variable(param_var_id.clone());

            self.graph.add_inference(param_var_ix, param_ty_ix,
                InferenceSource::FnParameter(block_fn.get_ident().clone()));
        }

        // Don't need to explicitly add the return type to the graph.
    }
}
