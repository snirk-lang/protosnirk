//! Item values
//!
//! An `Item` is a declaration made in the root context of a program
//! -- namely declarations such as `class`, `enum`, `struct`.
use std::cell::Cell;

use lex::{Token};
use parse::ast::{Identifier, Block, Expression};
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
    type_expr: FnTypeExpression,
    block: Block,
}
impl BlockFnDeclaration {
    /// Create a new FnDeclaration
    pub fn new(fn_token: Token,
               ident: Identifier,
               type_expr: FnTypeExpression,
               block: Block)
               -> BlockFnDeclaration {
        BlockFnDeclaration {
            fn_token, ident, type_expr, block
        }
    }
    /// Get the `fn` token
    pub fn get_token(&self) -> &Token {
        &self.fn_token
    }
    /// Get the identifier of the function
    pub fn get_ident(&self) -> &Identifier {
        &self.name
    }
    /// Get the textual name of the function
    pub fn get_name(&self) -> &str {
        &self.name.get_name()
    }
    /// Get the parameters of the function
    pub fn get_type_expr(&self) -> &FnTypeExpression {
        &self.type_expr
    }
    /// Get the block inside the function
    pub fn get_block(&self) -> &Block {
        &self.block
    }
}
