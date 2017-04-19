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
    data: SymbolData,
    ident: Identifier // Has copy of symbol id
}

impl Symbol {
    /// Create a new symbol (for a variable or item) with the given data.
    pub fn new(id: SymbolId, data: SymbolData, ident: Identifier) -> Symbol {
        Symbol { id: id, data: data, ident: ident }
    }

    /// Gets the data associated with the symbol
    pub fn get_data(&self) -> &SymbolData {
        &self.data
    }
}

/// Data that can be attached to a symbol when it is declared, marking it as
/// a variable or item.
#[derive(Debug, PartialEq, Clone)]
pub enum SymbolData {
    Item(ItemData),
    Variable(VariableData)
}

/// Data used for declaring an item (such as a function or static value)
#[derive(Debug, PartialEq, Clone)]
pub struct ItemData {
    // visibility
    // external?
}

/// Data used for declaring a variable, such as whether it is declared mutable
#[derive(Debug, PartialEq, Clone)]
pub struct VariableData {
    // ownership
    // declared type
    mutable: bool,
}

impl VariableData {
    pub fn new(mutable: bool) -> VariableData {
        VariableData { mutable: mutable }
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
                pub fn $ctor_name(id: SymbolId, data: $var_type, ident: Identifier) -> Symbol {
                    Symbol { id: id, data: SymbolData::$enum_type(data), ident: ident }
                }

                pub fn $is_name(&self) -> bool {
                    match self.data {
                        &SymbolData::$enum_type(_) => true,
                        _ => false
                    }
                }

                pub fn $get_name(&self) -> Option<&$var_type> {
                    match self.data {
                        &SymbolData::$enum_type(ref data) => Some(data).as_ref(),
                        _ => None
                    }
                }

            )+
        }
    }
}

symbol_methods! {
    Item ItemData : new_item, is_item, get_item_data,
    Variable VariableData : new_var, is_variable, get_var_data,
}
