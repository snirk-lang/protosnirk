//! Symbol table containing information about a given variable declaration

use std::collections::{HashMap, HashSet};
use std::ops::{Deref, DerefMut};

use lex::{Token, TextLocation};
use parse::ast::{Declaration, Identifier};
use parse::ScopedId;

/// A mapping from `ScopedId` given to AST `Identifier`s to `Symbol`s.
pub type SymbolTable = HashMap<ScopedId, Symbol>;

/// Data that can be attached to a symbol when it is declared, marking it as
/// a variable or item.
#[derive(Debug, PartialEq, Clone)]
pub enum Symbol {
    Item(ItemSymbol),
    Variable(VariableSymbol)
}

/// Data used for declaring an item (such as a function or static value)
#[derive(Debug, PartialEq, Clone)]
pub struct ItemSymbol {
    // visibility
    // external?
}

/// Data used for declaring a variable, such as whether it is declared mutable
#[derive(Debug, PartialEq, Clone)]
pub struct VariableSymbol {
    // ownership
    // declared type
    mutable: bool,
}

impl VariableSymbol {
    pub fn new(mutable: bool) -> VariableSymbol {
        VariableSymbol { mutable: mutable }
    }
    pub fn is_mutable(&self) -> bool {
        self.mutable
    }
}

/// Generate some helper methods to get
macro_rules! symbol_methods {
    ($($enum_type:ident $var_type:ident : $ctor_name:ident, $is_name:ident, $get_name:ident,)+) => {
        impl Symbol {
            $(
                pub fn $is_name(&self) -> bool {
                    match self.data {
                        &Symbol::$enum_type(_) => true,
                        _ => false
                    }
                }

                pub fn $get_name(&self) -> Option<&$var_type> {
                    match self.data {
                        &Symbol::$enum_type(ref data) => Some(data).as_ref(),
                        _ => None
                    }
                }

            )+
        }
    }
}

symbol_methods! {
    Item ItemSymbol : new_item, is_item, get_item_data,
    Variable VariableSymbol : new_var, is_variable, get_var_data,
}
