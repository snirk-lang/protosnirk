//! A scope for defining types.

use lex::Token;
use parse::{ScopedId, TypeId};
use parse::ast::{Identifier, Literal};
use parse::ast::types::TypeExpression;
use typeinfer::{ConcreteType, InferredType, TypeEquation, InferenceSource};

use std::collections::HashMap;

/// Maintains a mapping of `ScopedId -> TypeId`.
#[derive(Debug, PartialEq)]
pub struct TypeEquationBuilder {
    /// Current unique TypeId.
    current_id: TypeId,
    /// Given `TypeId`s for named identifiers.
    ident_types: HashMap<ScopedId, TypeId>,
    /// Sources for each `TypeId` inferred.
    inference_sources: HashMap<TypeId, Vec<InferenceSource>>,
    /// List of type equations.
    equations: Vec<TypeEquation>
}

impl TypeEquationBuilder {
    pub fn new(current_id: TypeId) -> TypeEquationBuilder {
        TypeEquationBuilder {
            ident_types: HashMap::new(),
            inference_sources: HashMap::new(),
            equations: Vec::new(),
            current_id
        }
    }

    /// Get the TypeId of the corresponding identifier.
    pub fn get_id(&mut self, scope_id: ScopedId) -> TypeId {
        if !self.ident_types.contains_key(&scope_id) {
            self.current_id.increment();
            self.ident_types.insert(scope_id, self.current_id);
            self.current_id
        }
        else {
            self.ident_types[&scope_id]
        }
    }

    /// Get a new `TypeId` to use.
    pub fn fresh_id(&mut self) -> TypeId {
        self.current_id.increment();
        self.current_id
    }

    /// Add an inference source for a given `TypeId`.
    pub fn add_source(&mut self, type_id: TypeId, source: InferenceSource) {
        self.inference_sources
            .entry(type_id)
            .or_insert_with(|| Vec::with_capacity(2)) // save a double
            .push(source);
    }

    /// Add an equation to the known list of equations.
    pub fn add_equation(&mut self, equation: TypeEquation) {
        self.equations.push(equation);
    }
}
