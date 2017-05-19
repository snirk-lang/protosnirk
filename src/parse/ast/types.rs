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
use std::cell::Cell;


use lex::{Token, TokenType, TokenData};
use parse::Id;
use parse::ast::{Literal, Identifier};

/// Type expressions in the AST.
#[derive(Debug, PartialEq, Clone)]
pub struct TypeExpression {
    /// ID given to types in `check::types`
    id: Cell<Id>,
    /// Type expression i.e. as in the source
    kind: TypeKind
}
impl TypeExpression {
    /// Creates a new TypeExpression with the given type kind.
    pub fn new(kind: TypeKind) -> TypeExpression {
        TypeExpression { id: Cell::default(), kind: kind }
    }

    /// Creates a new TypeExpression with the given type id and type kind.
    pub fn with_type_id(id: Id, kind: TypeKind) -> TypeExpression {
        TypeExpression { id: Cell::new(id), kind: kind }
    }

    /// Gets the type id of this TypeExpression
    pub fn get_type_id(&self) -> Id {
        &self.id
    }

    /// Whether this TypeExpression has a TypeId
    pub fn has_type_id(&self) -> bool {
        !self.id.is_default()
    }

    /// Sets the type ID of this TypeExpression
    pub fn set_type_id(&self, id: Id) {
        self.id = id;
    }

    /// Gets the `TypeKind` of this TypeExpression
    pub fn get_type(&self) -> &TypeKind {
        &self.kind
    }
}


/// TypeExpression expressions
#[derive(Debug, PartialEq, Clone)]
pub enum TypeKind {
    /// A primitive type, i.e. defined via LLVM
    Primitive(PrimitiveType),
    /// Single type, such as a struct or primitive
    Named(NamedType),
    /// The `_` in type expressions.
    PleaseInfer,
    /// Generic type, such as `List<T>`
    Generic(GenericType),
    /// Sized array, such as `[int: 3]`
    SizedArray(SizedArrayType),
    // Borrowed
    // Shared
}

/// Arguments to generic types.
#[derive(Debug, PartialEq, Clone)]
pub enum GenericParameter {
    /// Simple named arg, the `T` in `List<T>`
    Named(NamedType),
    // bounded
    // with-default?
}

/// Primitive (intrinsic) types.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PrimitiveType {
    /// Unit type ()
    Unit,
    /// Boolean type: `true` | `false`
    Bool,
    /// Numeric type
    Float64,
}

/// A named basic type (non-intrinsic), like `String`
#[derive(Debug, PartialEq, Clone)]
pub struct NamedType {
    ident: Identifier
}
impl NamedType {
    pub fn new(ident: Identifier) -> NamedType {
        NamedType { ident: ident }
    }

    pub fn get_ident(&self) -> &Identifier { &self.ident }
}

/// A type with generic parameters, like `List<T>`
#[derive(Debug, PartialEq, Clone)]
pub struct GenericType {
    ident: Identifier,
    args: Vec<GenericParameter>
}
impl GenericType {
    /// Create a new Generic `TypeKind` with the given identifier and generic args.
    pub fn new(ident: Identifier, args: Vec<GenericParameter>) -> GenericType {
        GenericType { ident: ident, args: args }
    }

    pub fn get_ident(&self) -> &Identifier { &self.ident }

    pub fn get_params(&self) -> &[GenericParameter] { &self.args }
}

/// An array with a fixed size, like `[int: 3]`
#[derive(Debug, PartialEq, Clone)]
pub struct SizedArrayType {
    value: Box<TypeExpression>,
    size: u64
}
impl SizedArrayType {
    pub fn new(value: Box<TypeExpression>, size: Literal) -> SizedArrayType {
        SizedArrayType { value: value, size: size }
    }
    pub fn get_inner_type(&self) -> &TypeExpression { &self.value }

    pub fn get_size_expr(&self) -> &Literal { &self.size }
}
