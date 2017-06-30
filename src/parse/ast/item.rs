//! Item values
//!
//! An `Item` is a declaration made in the root context of a program
//! -- namely declarations such as `class`, `enum`, `struct`.
use std::cell::Cell;

use lex::{Token};
use parse::Id;
use parse::ast::{Identifier, Block, Expression};
use parse::ast::types::{TypeExpression,
                        FnTypeExpression,
                        InlineFnTypeExpression};

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
    FnDeclaration(FnDeclaration)
}

#[derive(Debug, Clone, PartialEq)]
pub enum FnDeclaration {
    /// Function is declared inline with `=>`
    InlineFnDeclaration(InlineFnDeclaration),
    /// Function is declared as a block
    BlockFnDeclaration(BlockFnDeclaration)
}

/// Inline fn declaration.
///
/// Functions can be declared inline using the inline arrow. Inline
/// fn declarations are only allowed one expression, but the return
/// type is inferred.
///
/// # Example
/// ```snirk
/// fn foo(arg: float) => arg + 1
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct InlineFnDeclaration {
    fn_token: Token,
    ident: Identifier,
    type_expr: InlineFnTypeExpression,
    expr: Expression,
}
impl InlineFnDeclaration {
    /// Create a new `InlineFnDeclaration`
    pub fn new(fn_token: Token,
               ident: Identifier,
               type_expr: InlineFnTypeExpression,
               expr: Expression)
               -> BlockFnDeclaration {
        BlockFnDeclaration {
            fn_token, ident, type_expr, expr,
            type_id: Cell::new(Id::default())
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
    /// Get the type expression of the function
    pub fn get_type_expr(&self) -> &InlineFnTypeExpression {
        &self.type_expr
    }
    /// Get the expression of the function
    pub fn get_expr(&self) -> &Expression {
        &self.block
    }
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
            fn_token, ident, type_expr, block,
            type_id: Cell::new(Id::default())
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
