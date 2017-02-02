//! Symbol table containing information about a given variable declaration

use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use lex::Token;
use parse::verify::scope::ScopeIndex;
use parse::ast::Declaration;

/// Symbol stored in the symbol table
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Symbol {
    index: ScopeIndex,
    decl_token: Token,
    mutable: bool,
    used: bool,
    mutated: bool
}
impl Symbol {
    pub fn new(index: ScopeIndex, token: Token, mutable: bool) -> Symbol {
        Symbol {
            decl_token: token,
            index: index,
            mutable: mutable,
            used: false,
            mutated: false
        }
    }
    pub fn from_declaration(decl: &Declaration, index: ScopeIndex) -> Symbol {
        Symbol {
            decl_token: decl.token.clone(),
            index: index,
            mutable: decl.mutable,
            used: false,
            mutated: false
        }
    }
    pub fn get_index(&self) -> &ScopeIndex {
        &self.index
    }
    pub fn get_declaration(&self) -> &Token {
        &self.decl_token
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
