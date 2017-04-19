//! Symbol table containing information about a given variable declaration

use std::collections::{HashMap, HashSet};
use std::ops::{Deref, DerefMut};

use lex::{Token, TextLocation};
use parse::ast::{Declaration, Identifier};
use parse::ScopedId;

/// Identification of a unique symbol.
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash, Default)]
pub struct SymbolId(u32);

/// Each symbol in the program is a value which is declared in a scope.
/// For now, this is just used
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Symbol {
    id: SymbolId,
    declaration: SymbolDeclaration,
    source: SymbolSource
}

impl Symbol {
    pub fn new(index: ScopedId, token: Token, , source: SymbolSource) -> Symbol {
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
    pub fn from_declaration(decl: &Declaration, index: ScopedId) -> Symbol {
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
    pub fn from_parameter(ident: &Identifier, index: ScopedId) -> Symbol {
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
    pub fn from_fn_decl(ident: &Identifier, index: ScopedId, type_: Type) -> Symbol {
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

    pub fn get_index(&self) -> &ScopedId {
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

/// Represents data about a declared symbol
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SymbolDeclaration {
    mutable: bool,
    location: TextLocation,

}
