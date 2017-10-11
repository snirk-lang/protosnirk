//! Concrete type definitions.

use parse::ScopedId;
use parse::ast::{Identifier, Primitive};

/// A fully qualified type.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ConcreteType {
    Primitive(Primitive),
    Function(FnType),
}

/// The type signature of a function.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FnType {
    ident: ScopedId,
    args: Vec<(Identifier, ConcreteType)>,
    ret: Box<ConcreteType>
}
