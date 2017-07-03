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

mod expression;
mod item;
mod stmt;
mod operator;
pub mod types;

pub use self::expression::*;
pub use self::item::*;
pub use self::stmt::*;
pub use self::types::*;
pub use self::operator::Operator;

use std::cell::RefCell;

use lex::Token;
use parse::{Id, ScopedId};

/// A `TypeId` is an `Id` used for type inference.
pub type TypeId = Id;

/// Basic identifier type
#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub token: Token,
    pub id: RefCell<ScopedId>
}
impl Identifier {
    pub fn new(token: Token) -> Self {
        Identifier { token: token, index: RefCell::new(ScopedId::default()) }
    }
    pub fn get_name(&self) -> &str {
        &self.token.text
    }
    pub fn get_token(&self) -> &Token {
        &self.token
    }

    pub fn get_id(&self) -> &ScopedId {
        self.index.borrow()
    }

    pub fn set_id(&self, index: ScopedId) {
        *self.index.borrow_mut() = index;
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
    scope_id: RefCell<ScopedId>
}
impl Block {
    /// Create a new block from the given statements and scope id.
    pub fn new(statements: Vec<Statement>, scope_id: ScopedId) -> Block {
        Block { statements, scope_id }
    }
    pub fn has_value(&self) -> bool {
        if self.statements.len() == 0 {
            return false
        }
        let last_ix = self.statements.len() - 1;
        // TODO actual analysis
        for (ix, statement) in self.statements.iter().enumerate() {
            if ix == last_ix {
                return statement.has_value()
            }
            // else if stmt == return {
            //     return stmt.has_value()
            // }
        }
        return false
    }
    pub fn get_stmts(&self) -> &[Statement] {
        &self.statements
    }
    pub fn get_id(&self) -> &ScopedId {
        self.scope_id.borrow()
    }
    pub fn set_id(&self, id: ScopedId) {
        *self.scope_id.borrow_mut() = id;
    }
}
