//! Symbol table containing information about a given variable declaration

use std::collections::{HashMap, HashSet};
use std::ops::{Deref, DerefMut};

use lex::Token;
use parse::ast::{Declaration, Identifier};
use parse::types::Type;
use check::scope::ScopeIndex;


/// Symbol stored in the symbol table
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Symbol {
    /// Unique index for symbol
    index: ScopeIndex,
    /// Cloned token of symbol (i.e. for viewing source)
    decl_token: Token,
    /// Whether the symbol has been used
    used: bool,
    /// Whether the symbol is mutable (tracked per-reference)
    mutable: bool,
    /// Whether the symbol has been mutated
    mutated: bool,
    /// The type of the symbol
    type_: Type,
    /// The source of the symbol
    source: SymbolSource
}
impl Symbol {
    pub fn new(index: ScopeIndex, token: Token, mutable: bool, type_: Type, source: SymbolSource) -> Symbol {
        Symbol {
            decl_token: token,
            index: index,
            mutable: mutable,
            used: false,
            mutated: false,
            type_: type_,
            source: source,
        }
    }
    pub fn from_declaration(decl: &Declaration, index: ScopeIndex) -> Symbol {
        Symbol {
            decl_token: decl.get_ident().get_token().clone(),
            index: index,
            mutable: decl.mutable,
            used: false,
            mutated: false,
            type_: Type::Float,
            source: SymbolSource::Variable,
        }
    }
    pub fn from_parameter(ident: &Identifier, index: ScopeIndex) -> Symbol {
        Symbol {
            decl_token: ident.get_token().clone(),
            index: index,
            mutable: false, // Just gonna strait up refuse mutable parameters
            mutated: false,
            used: false,
            type_: Type::Float,
            source: SymbolSource::Parameter,
        }
    }
    pub fn from_fn_decl(ident: &Identifier, index: ScopeIndex, type_: Type) -> Symbol {
        Symbol {
            decl_token: ident.get_token().clone(),
            index: index,
            mutable: false,
            mutated: false,
            used: false,
            type_: type_,
            source: SymbolSource::DeclaredFn,
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
    pub fn get_type(&self) -> &Type {
        &self.type_
    }
    pub fn get_source(&self) -> SymbolSource {
        self.source
    }
}

/// Source of a particular symbol
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SymbolSource {
    /// The symbol was declared as a variable
    Variable,
    /// The symbol was declared as a fn parameter
    Parameter,
    /// The symbol was declared as a function
    DeclaredFn,
}
impl SymbolSource {
    pub fn get_name(self) -> &'static str {
        match self {
            SymbolSource::Variable => "variable",
            SymbolSource::Parameter => "function parameter",
            SymbolSource::DeclaredFn => "declared function"
        }
    }
}
