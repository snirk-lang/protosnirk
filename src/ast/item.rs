//! Item values
//!
//! An `Item` is a declaration made in the root context of a program
//! -- namely declarations such as `class`, `enum`, `struct`.
use std::cell::{Cell, Ref};

use lex::{Token};
use ast::{Identifier, Block, Expression, TypeExpression, ScopedId};

/// A single "unit" of parsed code.
#[derive(Debug, PartialEq, Clone)]
pub struct Unit {
    items: Vec<Item>
}

impl Unit {
    /// Create a new unit with the given block
    pub fn new(items: Vec<Item>) -> Unit {
        Unit { items: items }
    }
    /// Gets the collection of exported items
    pub fn items(&self) -> &[Item] {
        &self.items
    }
}

/// Items exported from a protosnirk program
#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    /// Declaraion of a function
    BlockFnDeclaration(BlockFnDeclaration)
}

/// Declaration of a function
#[derive(Debug, Clone, PartialEq)]
pub struct BlockFnDeclaration {
    fn_token: Token,
    ident: Identifier,
    params: Vec<(Identifier, TypeExpression)>,
    ret_ty: TypeExpression,
    explicit_ret_ty: bool,
    block: Block,
}
impl BlockFnDeclaration {
    /// Create a new FnDeclaration
    pub fn new(fn_token: Token,
               ident: Identifier,
               params: Vec<(Identifier, TypeExpression)>,
               ret_ty: TypeExpression,
               explicit_ret_ty: bool,
               block: Block)
               -> BlockFnDeclaration {
        BlockFnDeclaration {
            fn_token, ident, params, ret_ty, explicit_ret_ty, block
        }
    }
    /// Get the `fn` token
    pub fn token(&self) -> &Token {
        &self.fn_token
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
}
