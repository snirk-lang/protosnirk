//! Set `TypeId`s on items.

use std::collections::HashMap;

use parse::{ScopedId, TypeId};
use parse::ast::*;
use parse::ast::types::*;
use visit::visitor::{ItemVisitor, TypeVisitor, DefaultUnitVisitor};
use check::{CheckerError, ErrorCollector};
use typeinfer::{ConcreteType, InferenceSource, TypeEquation, InferredType};
use typeinfer::identify::{TypeEquationBuilder, TypeExprIdentifier};

/// Assigns `TypeId`s on items.
#[derive(Debug, PartialEq)]
pub struct ItemTypeIdentifier<'builder, 'err> {
    builder: &'builder mut TypeEquationBuilder,
    errors: &'err mut ErrorCollector
}

impl<'builder, 'err> ItemTypeIdentifier<'builder, 'err> {
    pub fn new(builder: &'builder mut TypeEquationBuilder,
               errors: &'err mut ErrorCollector)
               -> ItemTypeIdentifier<'builder, 'err> {
        ItemTypeIdentifier { builder, errors }
    }
}
impl<'builder, 'err> DefaultUnitVisitor
    for ItemTypeIdentifier<'builder, 'err> { }

impl<'builder, 'err> ItemVisitor for ItemTypeIdentifier<'builder, 'err> {
    fn visit_block_fn_decl(&mut self, block_fn: &BlockFnDeclaration) {
        let fn_scope_id = block_fn.get_ident().get_id();
        if fn_scope_id.is_default() { return }
        // Block fn:
        // fn foo(x: Type, y: Type2) -> Rettype
        //     (block)

        // foo.type_id = tfoo
        // tfoo = (tx, ty -> tRetType)
        // x.type_id = tx
        // tx = tType
        // y.type_id = ty
        // ty = tType

        // For now, we just have a `TypeExpression` for tType.
        // We know the idents are legit from `identify`, and for now there
        // isn't more complexity in the type system.

        // Set the `TypeId` for the block fn.
        let fn_ty_id = self.builder.get_id(fn_scope_id.clone());
        block_fn.get_ident().set_type_id(fn_ty_id);

        // Collect the types of the parameters.
        let mut param_types = HashMap::with_capacity(block_fn.get_params().len());
        for &(ref param, ref param_ty) in block_fn.get_params() {
            if param.get_id().is_default() { return }
            // Set the param's TypeId.
            let param_ty_id = self.builder.get_id(param.get_id().clone());
            param.set_type_id(param_ty_id);

            // Get an `InferredType` from the param's TypeExpression.
            let param_type: InferredType = {
                let mut param_identifier =
                    TypeExprIdentifier::new(self.builder, self.errors);
                param_identifier.visit_type_expr(param_ty);
                param_identifier.get_type()
            };
            // tpararm = <param type>
            self.builder.add_equation(TypeEquation {
                lhs: param_ty_id,
                rhs: param_type.clone()
            });
            // tparam: from fn parameter
            self.builder.add_source(param_ty_id,
                InferenceSource::FnParameter(param.clone()));
            param_types.insert(param.get_name().to_string(), param_type);
        }
        // Get the return type of the function or `Unary` for none specified.
        let ret_type = if let Some(ret_type) = block_fn.get_return_type() {
            let mut ret_type_identifier =
                TypeExprIdentifier::new(self.builder, self.errors);
            ret_type_identifier.visit_type_expr(ret_type);
            ret_type_identifier.get_type()
        }
        else { // Function returns void
            InferredType::Known(ConcreteType::Primitive(Primitive::Unary))
        };
        // tfn = (targ... -> ret_type)
        self.builder.add_equation(TypeEquation {
            lhs: fn_ty_id,
            rhs: InferredType::Fn {
                params: param_types,
                return_type: Box::new(ret_type)
            }
        });
        // tfn: from fn signature
        self.builder.add_source(fn_ty_id,
            InferenceSource::FnSignature(block_fn.get_ident().clone()));
    }
}
