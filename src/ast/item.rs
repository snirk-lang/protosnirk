//! Item values
//!
//! An `Item` is a declaration made in the root context of a program
//! -- namely declarations such as `class`, `enum`, `struct`.
use std::cell::Ref;

use lex::{Location, Span};
use ast::{Identifier, Block, TypeExpression, ScopedId};

/// A single "unit" of parsed code.
#[derive(Debug, PartialEq, Clone)]
pub struct Unit {
    items: Vec<Item>,
    span: Span
}

impl Unit {
    /// Create a new unit with the given block
    pub fn new(span: Span, items: Vec<Item>) -> Unit {
        Unit { span, items }
    }
    /// Gets the collection of exported items
    pub fn items(&self) -> &[Item] {
        &self.items
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

/// Items exported from a protosnirk program
#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    /// Declaraion of a function
    BlockFnDeclaration(BlockFnDeclaration),
    /// Declaration of a type alias
    Typedef(Typedef)
}

/// Declaration of a function
#[derive(Debug, Clone, PartialEq)]
pub struct BlockFnDeclaration {
    ident: Identifier,
    params: Vec<(Identifier, TypeExpression)>,
    ret_ty: TypeExpression,
    explicit_ret_ty: bool,
    block: Block,
    span: Span
}

impl BlockFnDeclaration {
    /// Create a new FnDeclaration
    pub fn new(start: Location,
               ident: Identifier,
               params: Vec<(Identifier, TypeExpression)>,
               ret_ty: TypeExpression,
               explicit_ret_ty: bool,
               block: Block)
               -> BlockFnDeclaration {
        BlockFnDeclaration {
            span: Span::from(start ..= block.span().end()),
            ident,
            params,
            ret_ty,
            explicit_ret_ty,
            block
        }
    }

    /// Get the identifier of the function
    pub fn ident(&self) -> &Identifier {
        &self.ident
    }
    pub fn params(&self) -> &[(Identifier, TypeExpression)] {
        &self.params
    }
    pub fn return_type(&self) -> &TypeExpression {
        &self.ret_ty
    }
    pub fn has_explicit_return_type(&self) -> bool {
        self.explicit_ret_ty
    }
    pub fn id<'a>(&'a self) -> Ref<'a, ScopedId> {
        self.ident.id()
    }
    pub fn set_id(&self, id: ScopedId) {
        self.ident.set_id(id);
    }
    /// Get the textual name of the function
    pub fn name(&self) -> &str {
        &self.ident.name()
    }
    /// Get the block inside the function
    pub fn block(&self) -> &Block {
        &self.block
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

/// Declaration of a type alias
#[derive(Debug, Clone, PartialEq)]
pub struct Typedef {
    alias_ident: Identifier,
    type_expr: TypeExpression,
    span: Span
}

impl Typedef {
    pub fn new(start: Location,
               alias_ident: Identifier,
               type_expr: TypeExpression)
               -> Typedef {
        Typedef {
            span: Span::from(start ..= type_expr.span().end()),
            alias_ident,
            type_expr
        }
    }

    pub fn ident(&self) -> &Identifier {
        &self.alias_ident
    }

    pub fn id<'a>(&'a self) -> Ref<'a, ScopedId> {
        self.alias_ident.id()
    }

    pub fn set_id(&self, id: ScopedId) {
        self.alias_ident.set_id(id)
    }

    pub fn name(&self) -> &str {
        self.alias_ident.name()
    }

    pub fn type_expr(&self) -> &TypeExpression {
        &self.type_expr
    }

    pub fn span(&self) -> Span {
        self.span
    }
}
