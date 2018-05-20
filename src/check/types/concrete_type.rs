//! Concrete type definitions.

use parse::ScopedId;
use ast::{Identifier, Primitive};

/// A fully qualified type.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum ConcreteType {
    Primitive(Primitive),
    Function(FnType),
}

/// The type signature of a function.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct FnType {
    ident: ScopedId,
    args: Vec<(String, ConcreteType)>,
    ret: Box<ConcreteType>
}
