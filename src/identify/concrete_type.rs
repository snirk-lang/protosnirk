//! Concrete type definitions.

use ast::{Identifier, Primitive};
use parse::ScopedId;

use std::cell::{RefCell, Cell, Ref};

/// A fully qualified type.
///
/// These are first identified in `identify/types`,
/// and are the roots of the type inference graph.
/// The identify and infer stage both make sure not to duplicate
/// concrete type definitiions (i.e. for functions), although
/// this is not the case for most functions as parameter names are
/// part of a function's type. This deduplication is more of an
/// efficiency concern than correctness proof.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum ConcreteType {
    /// Primitive types are just those defined as AST primitive type expressions.
    Primitive(Primitive),
    /// Function types contain ordered, named arguments and a return type.
    Function(FnType),
}

/// A function type.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct FnType {
    // Arguments to the function - names matter
    args: Vec<(String, ConcreteType)>,
    ret: Box<ConcreteType>
}
impl FnType {
    pub fn new(args: Vec<(String, ConcreteType)>, ret: ConcreteType) -> FnType {
        FnType { args, ret: Box::new(ret) }
    }
    pub fn get_args(&self) -> &[(String, ConcreteType)] {
        &self.args
    }
    pub fn get_return_ty(&self) -> &ConcreteType {
        &*self.ret
    }
}
