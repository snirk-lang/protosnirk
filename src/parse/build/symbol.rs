//! Symbol table containing information about a given variable declaration

use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use lex::Token;
use parse::ast::Declaration;

/// Symbol stored in the symbol table
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Symbol {
    declaration_token: Token,
    mutable: bool,
    used: bool,
    mutated: bool
}
impl Symbol {
    pub fn new(token: Token, mutable: bool) -> Symbol {
        Symbol {
            declaration_token: token,
            mutable: mutable,
            used: false,
            mutated: false
        }
    }
    pub fn from_declaration(decl: &Declaration) -> Symbol {
        Symbol {
            declaration_token: decl.token.clone(),
            mutable: decl.mutable,
            used: false,
            mutated: false
        }
    }
    pub fn get_token(&self) -> &Token {
        &self.declaration_token
    }
    pub fn is_mutable(&self) -> bool {
        self.mutable
    }
    pub fn is_used(&self) -> bool {
        self.used
    }
    pub fn is_mutated(&self) -> bool {
        self.mutated
    }
    pub fn set_mutated(&mut self) {
        self.mutated = true;
    }
    pub fn set_used(&mut self) {
        self.used = true;
    }
}
