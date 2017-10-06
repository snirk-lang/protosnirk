//! Type expressions
//!
//! Ways of representing a type. Can end up being complicated, like expressions.
// let x: Vector<Clone + Ordered> // Generic bounds
// let tuple: (int, int) // Tuple - unnamed (possibly ambiguous if we use param shortening )
// let anonStruct: struct(x: int, y: int) // anonymous named structs
// let tuple4: (array: [int], sizedArray: [int: 6] sizedArraySlice: &[int: 5], slice: &[int])

use std::cell::{Cell, Ref};

use lex::{Token, TokenType, TokenData};
use parse::ScopedId;
use parse::ast::{TypeId, Literal, Identifier};

/// Represents type expressions in protosnirk.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TypeExpression {
    /// Expression for a primitive type.
    Primitive(Primitive),
    /// Named type, such as `String`
    Named(NamedTypeExpression),
    /// Function type, such as `(x: float) -> bool`
    Function(FnTypeExpression),
}
impl TypeExpression {
}

/// A named type expression.
///
/// This is what most types in protosnirk will be made of.
/// This includes `float`, `bool`, etc.
/// Later, generic/const parameters will be added.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NamedTypeExpression {
    ident: Identifier,
    type_id: Cell<TypeId>,
}
impl NamedTypeExpression {
    /// Create a new `NamedTypeExpression` with
    /// the given name and default `TypeId`.
    pub fn new(ident: Identifier) -> NamedTypeExpression {
        NamedTypeExpression { ident, type_id: Cell::default() }
    }

    /// Gets the identifier of this type.
    pub fn get_ident(&self) -> &Identifier {
        &self.ident
    }

    pub fn get_id<'a>(&'a self) -> Ref<'a, ScopedId> {
        self.ident.get_id()
    }

    /// Get the `TypeId` of this type.
    pub fn get_type_id(&self) -> TypeId {
        self.type_id.get()
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
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FnTypeExpression {
    params: Vec<(Identifier, TypeExpression)>,
    return_type: Option<Box<TypeExpression>>,
    type_id: Cell<TypeId>
}
impl FnTypeExpression {
    /// Create a new `FnTypeExpression` with
    /// the given argument list, return type, and default `TypeId`.
    pub fn new(params: Vec<(Identifier, TypeExpression)>,
               return_type: Option<TypeExpression>) -> FnTypeExpression {
        FnTypeExpression {
            params,
            return_type: return_type.map(|ret| Box::new(ret)),
            type_id: Cell::default()
        }
    }

    /// Get the parameter list of this function type.
    pub fn get_params(&self) -> &[(Identifier, TypeExpression)] {
        &self.params
    }
    /// Get the return type of this function type.
    ///
    /// If none, the return type is `()` but undeclared.
    pub fn get_return_type(&self) -> Option<&TypeExpression> {
        // &Option<Box<T>> -> Option<&Box<T>> -> Option<&T>
        self.return_type.as_ref().map(|b| b.as_ref())
    }

    /// Get the `TypeId` of this type.
    pub fn get_type_id(&self) -> TypeId {
        self.type_id.get()
    }

    /// Set the `TypeId` of this type expression.
    pub fn set_type_id(&self, new_id: TypeId) {
        self.type_id.set(new_id);
    }
}

/// Type expression for a primitive type.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Primitive {
    /// () or void
    Unary,
    /// `int` becomes `Int64`
    Int,
    /// `bool` becomes `Bool`
    Bool,
}
