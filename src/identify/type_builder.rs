use std::collections::HashMap;
use std::hash::Hash;

use ast::TypeId;
use identify::ConcreteType;

/// Builds a mapping of `TypeId` -> `ConcreteType`.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct TypeBuilder {
    defined: HashMap<TypeId, ConcreteType>,
    ids: HashMap<ConcreteType, TypeId>,
    current_id: TypeId
}

impl TypeBuilder {
    pub fn new() -> TypeBuilder {
        TypeBuilder::default()
    }

    pub fn decompose(self) -> (HashMap<TypeId, ConcreteType>) {
        self.defined
    }

    pub fn define_type(&mut self, ty: ConcreteType) -> TypeId {
        if let Some(found) = self.ids.get(&ty) {
            return *found
        }
        self.current_id.increment();
        let new_id = self.current_id;
        self.defined.insert(new_id, ty.clone());
        self.ids.insert(ty, new_id);
        new_id
    }

    pub fn get(&self, id: TypeId) -> Option<&ConcreteType> {
        self.defined.get(&id)
    }

    pub fn get_id(&self, ty: &ConcreteType) -> Option<TypeId> {
        self.ids.get(ty).cloned()
    }
}
