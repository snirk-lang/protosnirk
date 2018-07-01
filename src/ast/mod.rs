//! Abstract syntax tree definitions.
//!
//! This module contains definitions of all of the
//! AST node types used to parse a protosnirk program,
//! with a `Unit` being the root of the syntax tree.
//!
//! Currently, the parser and checkers do not run
//! transformative passes to the AST. Instead, many
//! nodes contain `parse::Id`s which point to data
//! tables collected in various passes, such as
//! symbol or type information.

mod index;
mod expression;
mod item;
mod stmt;
mod operator;
pub mod types;
pub mod visit;

pub use self::index::*;
pub use self::expression::*;
pub use self::item::*;
pub use self::stmt::*;
pub use self::operator::*;
pub use self::types::*;

use std::cell::{RefCell, Ref};

use lex::Token;

/// Basic identifier type
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Identifier {
    token: Token,
    id: RefCell<ScopedId>,
}
impl Identifier {
    pub fn new(token: Token) -> Self {
        Identifier { token, id: RefCell::default() }
    }
    pub fn name(&self) -> &str {
        &self.token.text()
    }
    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn id<'a>(&'a self) -> Ref<'a, ScopedId> {
        self.id.borrow()
    }

    pub fn set_id(&self, index: ScopedId) {
        debug_assert!(!index.is_default(),
            "Attempted to reset the ID of {:?}", self);
        *self.id.borrow_mut() = index;
    }
}

impl Into<Token> for Identifier {
    fn into(self) -> Token {
        self.token
    }
}

/// Collection of statements which may have an expression value
#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    /// Statements in the block
    pub statements: Vec<Statement>,
    /// Identifier used for typechecking.
    scope_id: RefCell<ScopedId>,
    /// What uses the value of this block as an expression?
    source: RefCell<Option<ScopedId>>,
}
impl Block {
    /// Create a new block from the given statements and scope id.
    pub fn new(statements: Vec<Statement>) -> Block {
        Block {
            statements,
            scope_id: RefCell::default(),
            source: RefCell::new(None)
        }
    }

    pub fn stmts(&self) -> &[Statement] {
        &self.statements
    }
    pub fn id<'a>(&'a self) -> Ref<'a, ScopedId> {
        self.scope_id.borrow()
    }
    pub fn set_id(&self, id: ScopedId) {
        *self.scope_id.borrow_mut() = id;
    }
    pub fn source<'a>(&'a self) -> Ref<'a, Option<ScopedId>> {
        self.source.borrow()
    }
    pub fn set_source(&self, source: ScopedId) {
        *self.source.borrow_mut() = Some(source);
    }
    pub fn has_source(&self) -> bool {
        self.source.borrow().is_some()
    }
}
