//! Types in protosnirk.
//!
//! These represent concrete pieces of information about the types of
//! protosnirk values. Type checking involves comparing type expressions
//! in the AST with inferred type data to produce `check::types::Type`s.

use std::collections::HashMap;

use parse::ast::Identifier;

/// Representation of types in protosnirk.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    /// `()`
    Empty,
    /// Standard number type for now
    Float,
    /// Values of `true` or `false`.
    Boolean,
    /// Function - only used in declarations
    Fn(FnType)
}
impl Type {
    pub fn expect_fn(self) -> FnType {
        match self {
            Type::Fn(inner) => inner,
            other => panic!("`expect_fn` called on {:?}", other)
        }
    }
}

/// Type representation of functions in protosnirk
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FnType {
    /// Return type of the function
    return_type: Box<Type>,
    /// Argument types. Both order and names are important.
    args: Vec<(Identifier, Type)>
}
impl FnType {
    pub fn new(return_type: Box<Type>, args: Vec<(String, Type)>) -> FnType {
        FnType { return_type: return_type, args: args }
    }
    pub fn get_return(&self) -> &Type {
        &self.return_type
    }
    pub fn get_args(&self) -> &[(String, Type)] {
        &self.args
    }
    pub fn get_arg(&self, name: &str) -> Option<(usize, Type)> {
        for (ix, arg) in self.args.iter().enumerate() {
            if arg.0 == name {
                return Some((ix, arg.1.clone()))
            }
        }
        return None
    }
}
