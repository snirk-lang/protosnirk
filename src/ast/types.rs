//! Type expressions
//!
//! Ways of representing a type. Can end up being complicated, like expressions.
// let x: Vector<Clone + Ordered> // Generic bounds
// let tuple: (int, int) // Tuple - unnamed (possibly ambiguous if we use param shortening )
// let anonStruct: struct(x: int, y: int) // anonymous named structs
// let tuple4: (array: [int], sizedArray: [int: 6] sizedArraySlice: &[int: 5], slice: &[int])

use std::cell::{RefCell, Ref};

use lex::{Token, TokenType, TokenData};
use ast::{ScopedId, Literal, Identifier};

/// Represents type expressions in protosnirk.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TypeExpression {
    /// Named type, in this case `float` or `bool`
    Named(NamedTypeExpression),
}

impl TypeExpression {
    pub fn get_id(&self) -> Ref<ScopedId> {
        match self {
            any => any.get_id()
        }
    }
}

/// A named type expression.
///
/// This is what most types in protosnirk will be made of.
/// This includes `float`, `bool`, etc.
/// Later, generic/const parameters will be added.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NamedTypeExpression {
    ident: Identifier,
}
impl NamedTypeExpression {
    /// Create a new `NamedTypeExpression` with
    /// the given name and default `TypeId`.
    pub fn new(ident: Identifier) -> NamedTypeExpression {
        NamedTypeExpression { ident }
    }

    /// Gets the identifier of this type.
    pub fn get_ident(&self) -> &Identifier {
        &self.ident
    }

    pub fn get_id<'a>(&'a self) -> Ref<'a, ScopedId> {
        self.ident.get_id()
    }

    pub fn set_id<'a>(&'a self, id: ScopedId) {
        self.ident.set_id(id);
    }
}
