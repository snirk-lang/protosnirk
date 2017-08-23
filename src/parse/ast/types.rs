//! Type expressions
//!
//! Ways of representing a type. Can end up being complicated, like expressions.
// let x: Vector<Clone + Ordered> // Generic bounds
// let tuple: (int, int) // Tuple - unnamed (possibly ambiguous if we use param shortening )
// let anonStruct: struct(x: int, y: int) // anonymous named structs
// let tuple4: (array: [int], sizedArray: [int: 6] sizedArraySlice: &[int: 5], slice: &[int])

use std::cell::Cell;

use lex::{Token, TokenType, TokenData};
use parse::ast::{TypeId, Literal, Identifier};

/// Represents type expressions in protosnirk.
#[derive(Debug, PartialEq, Clone)]
pub enum TypeExpression {
    // Inferred type, used when the type is not specified in source.
    //Inferred(Cell<TypeId>),
    /// Named type, such as `bool` or `String`
    Named(NamedTypeExpression),
    /// Function type, such as `(x: float) -> bool`
    Function(FnTypeExpression),
}
impl TypeExpression {
    /// Get the `TypeId` of this type.
    pub fn get_type_id(&self) -> TypeId {
        match self {
            any => any.get_id()
        }
    }
    /// Set the `TypeId` of this type.
    pub fn set_type_id(&self, new_id: TypeId) {
        match self {
            any => any.set_type_id(new_id)
        }
    }
}

/// A named type expression.
///
/// This is what most types in protosnirk will be made of.
/// This includes `float`, `bool`, etc.
/// Later, generic/const parameters will be added.
#[derive(Debug, PartialEq, Clone)]
pub struct NamedTypeExpression {
    ident: Identifier,
    type_id: Cell<TypeId>,
}
impl NamedTypeExpression {
    /// Create a new `NamedTypeExpression` with
    /// the given name and default `TypeId`.
    pub fn new(name: Identifier) -> NamedTypeExpression {
        NamedTypeExpression { name, type_id: TypeId::default() }
    }

    /// Create a new `NamedTypeExpression` with the given name and `TypeId`.
    pub fn with_id(name: Identifier, type_id: TypeId) -> NamedTypeExpression {
        NamedTypeExpression { name, type_id }
    }

    /// Gets the identifier of this type.
    pub fn get_ident(&self) -> Identifier {
        &self.ident
    }

    /// Get the `TypeId` of this type.
    pub fn get_type_id(&self) -> TypeId {
        *self.type_id.get()
    }

    /// Set the `TypeId` of this type expression.
    pub fn set_type_id(&self, new_id: TypeId) {
        self.type_id.set(new_id);
    }
}

/// A function type expression.
///
/// This is essentially everything that comes after the name of a function:
/// ```
/// fn foo(arg1: Type, arg2: Type2) -> ResultType
///       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
/// ```
/// So this item has the identifier `foo` and a `FnTypeExpression` for its type.
///
/// It could also be used for named HKT - there's more to figure out between
/// named/ordered params.
#[derive(Debug, PartialEq, Clone)]
pub struct FnTypeExpression {
    params: Vec<(Identifier, TypeExpression)>,
    return_type: Option<TypeExpression>,
    type_id: Cell<TypeId>
}
impl FnTypeExpression {
    /// Create a new `FnTypeExpression` with
    /// the given argument list, return type, and default `TypeId`.
    pub fn new(params: Vec<(Identifier, TypeExpression)>,
               return_type: Option<TypeExpression>) -> FnTypeExpression {
        FnTypeExpression {
            params,
            return_type,
            type_id: TypeId::default()
        }
    }

    /// Create a new `FnTypeExpression` with the given argument list,
    /// return type, and `TypeId`.
    pub fn with_id(params: Vec<(Identifier, TypeExpression)>,
                   return_type: TypeExpression,
                   type_id: TypeId) -> FnTypeExpression {
        FnTypeExpression { params, return_type, type_id }
    }

    /// Get the parameter list of this function type.
    pub fn get_params(&self) -> &[(Identifier, TypeExpression)] {
        &self.params
    }
    /// Get the return type of this function type.
    ///
    /// If none, the return type is `()` but undeclared.
    pub fn get_return_type(&self) -> Option<&TypeExpression> {
        &self.return_type
    }

    /// Get the `TypeId` of this type.
    pub fn get_type_id(&self) -> TypeId {
        *self.type_id.get()
    }

    /// Set the `TypeId` of this type expression.
    pub fn set_type_id(&self, new_id: TypeId) {
        self.type_id.set(new_id);
    }
}
