//! Item values
//!
//! An `Item` is a declaration made in the root context of a program
//! -- namely the imort item `use`, and declarations such as `class`,
//! `enum`, `struct`.

use lex::{Token};
use parse::ast::{Identifier, Block};

// This will expand greatly in the future, but for now it's a solid way
// to have an "enty point" in the compiler (and allow nested blocks)

/// A single "unit" of computation.
///
/// In the future this should be an entire progam definition,
/// complete with lists of defined types, functions, etc.
#[derive(Debug, Clone, PartialEq)]
pub struct Unit {
    items: Vec<Item>
}

/// Items exported from a protosnirk program
#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    FnDeclaration(FnDeclaration)
}

impl Unit {
    /// Create a new unit with the given block
    pub fn new(items: Vec<Item>) -> Unit {
        Unit { items: items }
    }
    pub fn get_items(&self) -> &[Item] {
        &self.items
    }
}

/// Declaration of a function
#[derive(Debug, Clone, PartialEq)]
pub struct FnDeclaration {
    fn_token: Token,
    name: Identifier,
    arg_list: Vec<Identifier>, // No types here yet :/
    block: Block
}
impl FnDeclaration {
    /// Create a new FnDeclaration
    pub fn new(fn_token: Token, name: Identifier, arg_list: Vec<Identifier>, block: Block)
               -> FnDeclaration {
        FnDeclaration {
            fn_token: fn_token,
            name: name,
            arg_list: arg_list,
            block: block
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
    /// Gets the textual name of the function
    pub fn get_name(&self) -> &str {
        &self.name.get_name()
    }
    /// Get the prototype of the function
    pub fn get_args(&self) -> &Vec<Identifier> {
        &self.arg_list
    }
    /// Get the block inside the function
    pub fn get_block(&self) -> &Block {
        &self.block
    }
}
