//! Information about types inferred by the `TypeIdentifier`s.

use std::collections::HashMap;

use parse::TypeId;
use typeinfer::ConcreteType;

/// Type information about a value inferred during `typeinfer`.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InferredType {
    /// The type is the same as another `TypeId`.
    Variable(TypeId),
    /// The type is known to be a real type.
    /// Usually a primitive.
    Known(ConcreteType),
    /// The type is known to be a function type.
    ///
    /// This is added both as a function is declared and every time it's
    /// called.
    Fn {
        /// Parameters of the function
        params: HashMap<String, InferredType>,
        /// Return type of the function
        return_type: Box<InferredType>
    },
}

/// An equation relating a `TypeId` to an inferred type.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TypeEquation {
    pub lhs: TypeId,
    pub rhs: InferredType
}
impl TypeEquation {
    pub fn new(lhs: TypeId, rhs: InferredType) -> TypeEquation {
        TypeEquation { lhs, rhs }
    }
}
