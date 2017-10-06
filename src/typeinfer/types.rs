//! Types in protosnirk.
//!
//! These represent concrete pieces of information about the types of
//! protosnirk values. Type checking involves comparing type expressions
//! in the AST with inferred type data to produce `check::types::Type`s.

use std::collections::HashMap;

use parse::ScopedId;
use parse::ast::Identifier;

use check::types::{TypeId, TypeEnvironment};

/// Representation of types which have been inferred.
/// This is intended to cover generics in the future; so the types are not
/// always concrete.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LinkedType {
    Identified(TypeId),
    BlockFn(LinkedFnType),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LinkedFnType {
    args: Vec<(ScopedId, LinkedType)>,
    return_: Box<LinkedType>
}

pub type TypeTable = HashMap<ScopedId, ConstructedType>;

/// Concrete information about a type created after inference is complete.
///
///
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ConstructedType {

}

/// Represents a description of a type.
#[derive(Debug, PartialEq, Clone, Hash)]
pub enum ProjectedType<'env> {
    Empty,
    Float(&'env TypeId),
    Bool(&'env TypeId),
    Function {
        params: Vec<(&'env ScopedId, &'env TypeId)>,
        return_type: ProjectedType<'env>
    }
}
impl<'env> ProjectedType<'env> {
    pub fn from_id(env: &'env TypeEnvironment,
                   id: TypeId)
                   -> Option<ProjectedType<'env>> {

    }
    pub fn from_known_id(env: &'env TypeEnvironment,
                         id: &'env TypeId)
                         -> ProjectedType<'env> {

    }
}
