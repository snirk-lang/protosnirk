//! Run type inference to produce a mapping of the actual conrete types of
//! things.

use lex::Token;
use ast::*;
use check::{CheckerError, ErrorCollector};
use identify::{ConcreteType, FnType, TypeGraph, TypeScopeBuilder};
use visit;
use visit::visitor::*;

use std::collections::HashMap;

/// Mapping of a `ScopedId` to its concrete type.
pub type TypeMapping = HashMap<ScopedId, ConcreteType>;

/// Solves type equations in a `TypeGraph` in order to produce a `TypeMapping`.
#[derive(Debug)]
pub struct TypeConcretifier<'err, 'builder, 'graph> {
    errors: &'err mut ErrorCollector,
    graph: &'graph mut TypeGraph,
    builder: &'builder TypeScopeBuilder,
    results: HashMap<ScopedId, ConcreteType>
}

impl<'err, 'builder, 'graph> TypeConcretifier<'err, 'builder, 'graph> {
    pub fn new(builder: &'builder TypeScopeBuilder,
               errors: &'err mut ErrorCollector,
               graph: &'graph mut TypeGraph)
               -> TypeConcretifier<'err, 'builder, 'graph> {
        TypeConcretifier {
            builder, errors, graph,
            results: HashMap::new()
        }
    }

    pub fn into_results(self) -> HashMap<ScopedId, ConcreteType> {
        self.results
    }

    fn infer_var(&mut self,
                 id: &ScopedId,
                 source: &Token,
                 context: String) -> bool {
        if let Some(_known) = self.results.get(id) {
            trace!("Known type of {:?}", id);
            return true
        }
        trace!("Inferring the type of {}", source.get_text());
        let inferred = self.graph.infer_type_of_var(id);
        match inferred {
            Ok((_ix, ty)) => {
                if let Some(concrete) = self.builder.get_type(&ty) {
                    debug!("Type of {} {:?} => {:?}",
                        source.get_text(), id, ty);
                    self.results.insert(id.clone(), concrete.clone());
                    true
                }
                else {
                    debug!("Error: unknown type");
                    // Shouldn't happen?
                    false
                }
            },
            Err(possibles) => {
                if possibles.is_empty() {
                    self.errors.add_error(CheckerError::new(
                        source.clone(),
                        vec![],
                        format!("Could not determine type of {} (conflicts)",
                                context)
                    ));
                }
                else {
                    self.errors.add_error(CheckerError::new(
                        source.clone(),
                        vec![],
                        format!("Could not type of {}", context)
                    ));
                }
                false
            }
        }
    }

    fn infer_type(&mut self, _id: &ScopedId) -> bool {
        unimplemented!("Type expression and function types are known during
            identification and need no further resolution");
    }
}

impl<'err, 'builder, 'graph> DefaultUnitVisitor
    for TypeConcretifier<'err, 'builder, 'graph> { }

impl<'err, 'builder, 'graph> ItemVisitor
    for TypeConcretifier<'err, 'builder, 'graph> {

    fn visit_block_fn_decl(&mut self, block_fn: &BlockFnDeclaration) {
        self.infer_var(&block_fn.get_id(), block_fn.get_token(),
            format!("fn declaration {}", block_fn.get_name()));

        for &(ref param, ref _param_ty) in block_fn.get_params() {
            self.infer_var(&param.get_id(), param.get_token(),
                format!("fn {} param {}",
                    block_fn.get_name(), param.get_name()));
        }

        visit::walk_block(self, block_fn.get_block());
    }
}

impl<'err, 'builder, 'graph> DefaultBlockVisitor
    for TypeConcretifier<'err, 'builder, 'graph> { }

impl<'err, 'builder, 'graph> DefaultStmtVisitor
    for TypeConcretifier<'err, 'builder, 'graph> { }

impl<'err, 'builder, 'graph> ExpressionVisitor
    for TypeConcretifier<'err, 'builder, 'graph> {

    fn visit_literal_expr(&mut self, _literal: &Literal) {
        // Literal types are all known.
    }

    fn visit_var_ref(&mut self, ident: &Identifier) {
        self.infer_var(&ident.get_id(), ident.get_token(),
            format!("Variable {}", ident.get_name()));
    }

    fn visit_if_expr(&mut self, if_expr: &IfExpression) {
        visit::walk_if_expr(self, if_expr);
    }

    fn visit_unary_op(&mut self, unary_op: &UnaryOperation) {
        visit::walk_unary_op(self, unary_op);
    }

    fn visit_binary_op(&mut self, binary_op: &BinaryOperation) {
        visit::walk_bin_op(self, binary_op);
    }

    fn visit_fn_call(&mut self, fn_call: &FnCall) {
        self.infer_var(&fn_call.get_id(), fn_call.get_token(),
            format!("Call to {}", fn_call.get_text()));
        for arg in fn_call.get_args() {
            self.visit_expression(arg.get_expression());
        }
    }

    fn visit_assignment(&mut self, assign: &Assignment) {
        self.visit_expression(assign.get_rvalue());
        self.infer_var(&assign.get_lvalue().get_id(),
            assign.get_lvalue().get_token(),
            format!("assignment to {}",
                    assign.get_lvalue().get_name()));
    }

    fn visit_declaration(&mut self, decl: &Declaration) {
        self.visit_expression(decl.get_value());
        self.infer_var(&decl.get_id(), decl.get_token(),
            format!("definition of variable {}", decl.get_token()));
    }
}
