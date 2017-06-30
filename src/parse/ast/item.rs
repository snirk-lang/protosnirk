//! Item values
//!
//! An `Item` is a declaration made in the root context of a program
//! -- namely declarations such as `class`, `enum`, `struct`.
use std::cell::{Cell, Ref};

use lex::{Token};
use parse::ast::{Identifier, Block, Expression, ScopedId};
use parse::ast::types::{TypeExpression,
                        FnTypeExpression};

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
    pub fn get_items(&self) -> &[Item] {
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
    ret_ty: Option<TypeExpression>,
    block: Block,
}
impl BlockFnDeclaration {
    /// Create a new FnDeclaration
    pub fn new(fn_token: Token,
               ident: Identifier,
               params: Vec<(Identifier, TypeExpression)>,
               ret_ty: Option<TypeExpression>,
               block: Block)
               -> BlockFnDeclaration {
        BlockFnDeclaration {
            fn_token, ident, params, ret_ty, block
        }
    }
    /// Get the `fn` token
    pub fn get_token(&self) -> &Token {
        &self.fn_token
    }
    /// Get the identifier of the function
    pub fn get_ident(&self) -> &Identifier {
        &self.ident
    }
    pub fn get_params(&self) -> &[(Identifier, TypeExpression)] {
        &self.params
    }
    pub fn get_return_type(&self) -> Option<&TypeExpression> {
        self.ret_ty.as_ref()
    }
    pub fn get_id<'a>(&'a self) -> Ref<'a, ScopedId> {
        self.ident.get_id()
    }
    pub fn set_id(&self, id: ScopedId) {
        self.ident.set_id(id);
    }
    /// Get the textual name of the function
    pub fn get_name(&self) -> &str {
        &self.ident.get_name()
    }
    /// Get the block inside the function
    pub fn get_block(&self) -> &Block {
        &self.block
    }
}
