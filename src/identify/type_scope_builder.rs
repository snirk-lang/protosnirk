//!

use ast::ScopedId;
use identify::{ConcreteType, NamedType};

use std::collections::HashMap;

/// These are NamedTypes which are "injected" into the default
/// type scope. This means that we parse things like `float` or `bool`
/// as `NamedTypeExpression`s. We treat them as being special during the compile
/// phase.
const PRIMITIVE_TYPE_NAMES: &[&'static str] = &[
    "()",
    "float",
    "bool",
];

#[derive(Debug, PartialEq, Clone)]
pub struct TypeScopeBuilder {
    /// ScopedIds for named types (primitives)
    names: HashMap<String, ScopedId>,
    /// ScopedIds for other types (function types)
    types: HashMap<ScopedId, ConcreteType>,
    current_id: ScopedId
}

impl TypeScopeBuilder {
    pub fn with_primitives() -> TypeScopeBuilder {
        let mut curr_id = ScopedId::default().pushed().incremented();

        let mut names = HashMap::new();
        let mut types = HashMap::new();

        for PRIM_TYPE in PRIMITIVE_TYPE_NAMES.iter() {
            names.insert(PRIM_TYPE.to_string(), curr_id);
            types.insert(curr_id, ConcreteType::Named(NamedType::new(PRIM_TYPE.to_string())));
            curr_id.increment();
        }

        TypeScopeBuilder { names, types, current_id: curr_id }
    }

    pub fn get_type(&self, id: &ScopedId) -> Option<&ConcreteType> {
        self.types.get(id)
    }

    pub fn get_named_type_id(&self, name: &str) -> Option<&ScopedId> {
        self.names.get(name)
    }

    pub fn get_named_type(&self, name: &str) -> Option<&ConcreteType> {
        self.names.get(name).and_then(|id| self.types.get(id))
    }

    /// Add a new concrete type with the given ID to the type scope.
    ///
    /// This is used for adding function types, which are not tracked by name
    /// but instead by ID, as they are first identified for expressions.
    pub fn add_type(&mut self, id: ScopedId, ty: ConcreteType) {
        self.types.insert(id, ty);
    }
}
