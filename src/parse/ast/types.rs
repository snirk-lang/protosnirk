//! TypeExpression expressions
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

/// TypeExpression expressions
#[derive(Debug, PartialEq, Clone)]
pub enum TypeExpression {
    /// A primitive type, i.e. defined via LLVM
    Primitive(PrimitiveType),
    /// Single type, such as a struct or primitive
    Named(Named),
    /// Generic type, such as `List<T>`
    Generic(Generic),
    /// Sized array, such as `[int: 3]`
    SizedArray(SizedArray),
    // Borrowed
    // Shared
}

/// Primitive (intrinsic) types.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PrimitiveType {
    /// Unit type ()
    Unit,
    /// Boolean type: true | false
    Bool,
    /// Numeric type
    Float64,
}

/// A named basic type (non-intrinsic), like `String`
#[derive(Debug, PartialEq, Clone)]
pub struct Named {
    ident: Identifier
}
impl Named {
    pub fn new(ident: Identifier) -> Named {
        Named { ident: ident }
    }

    pub fn get_ident(&self) -> &Identifier { &self.ident }
}

/// A type with generic parameters, like `List<T>`
#[derive(Debug, PartialEq, Clone)]
pub struct Generic {
    ident: Identifier,
    args: Vec<TypeExpression>
}
impl Generic {
    pub fn new(ident: Identifier, args: Vec<TypeExpression>) -> Generic {
        Generic { ident: ident, args: args }
    }

    pub fn get_ident(&self) -> &Identifier { &self.ident }

    pub fn get_args(&self) -> &[TypeExpression] { &self.args }
}

/// An array with a fixed size, like `[int: 3]`
#[derive(Debug, PartialEq, Clone)]
pub struct SizedArray {
    value: Box<TypeExpression>,
    size: Literal
}
impl SizedArray {
    pub fn new(value: Box<TypeExpression>, size: Literal) -> SizedArray {
        SizedArray { value: value, size: size }
    }
    pub fn get_inner_type(&self) -> &TypeExpression { &self.value }

    pub fn get_size_expr(&self) -> &Literal { &self.size }
}
