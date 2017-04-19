//! Type expressions
//!
//! Ways of representing a type. Can end up being complicated, like expressions.
//! let x: Vector<Clone + Ordered> // Generic bounds
//! // let x: Vec<?Sized> // Not-bounds, etc. from Rust or Pony
//! let point: (x: int, y: int) // Tuple - named
//! let namedTuple: (int,) // Tuple - unnamed
//! let tuple4: (array: [int], sizedArray: [int: 6] sizedArraySlice: &[int: 5], slice: &[int])
//! let array: [int] = myList.getArray()
//! let x =
//! Arrays can all be allocated on the stack
//! Maybe just make array syntax a macro...

use lex::{Token, TokenType, TokenData};
use parse::ast::{Literal, Identifier};

/// Represents the type of a value.
#[derive(Debug, PartialEq, Clone)]
pub struct Type {
    reference: Ownership,
    value: TypeExpression
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Ownership {
    Owned,
    Borrowed,
    Shared
}

/// Types.
#[derive(Debug, PartialEq, Clone)]
pub enum TypeExpression {
    /// Single type, such as a struct or primitive
    Named(Named),
    /// Generic type, such as `List<T>`
    Generic(Generic),
    /// Named tuple, such as `(x: int, y: int)`
    NamedTuple(NamedTuple),
    /// Unnamed tuple, such as `(int, int)`
    UnnamedTuple(UnnamedTuple),
    /// Sized array, such as `[int: 3]`
    SizedArray(SizedArray),
    /// Unsized array, such as `[int]`
    UnsizedArray(UnsizedArray)
}

/// A basic type, just a name like `int` or `String`
#[derive(Debug, PartialEq, Clone)]
pub struct Named {
    ident: Identifier
}

/// A type with generic parameters, like `List<T>`
#[derive(Debug, PartialEq, Clone)]
pub struct Generic {
    ident: Identifier,
    args: Vec<Type>
}

/// A named tuple type, like `(x: int, y: int)`
#[derive(Debug, PartialEq, Clone)]
pub struct NamedTuple {
    types: HashMap<Identifier, Type>
}

/// An unnamed tuple type, like `(int, String)`
#[derive(Debug, PartialEq, Clone)]
pub struct UnnamedTuple {
    types: Vec<Type>
}

/// An array with a fixed size, like `[int: 3]`
#[derive(Debug, PartialEq, Clone)]
pub struct SizedArray {
    value: Type,
    size: ast::Literal
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnsizedArray {
    inner: Box<Type>
}
