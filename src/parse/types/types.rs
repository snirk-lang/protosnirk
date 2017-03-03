//! Definition of types in protosnirk

use std::collections::HashMap;

/// Representation of types in protosnirk
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    /// `()`
    Empty,
    /// Standard type for now
    Float,
    /// Function - only used in declarations
    Fn(FnType)
}

/// Type representation of functions in protosnirk
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FnType {
    return_type: Box<Type>,
    args: HashMap<String, Type>
}
impl FnType {
    pub fn new(return_type: Box<Type>, args: HashMap<String, Type>) -> FnType {
        FnType { return_type: return_type, args: args }
    }
    pub fn get_return(&self) -> &Type {
        &self.return_type
    }
    pub fn get_args(&self) -> &HashMap<String, Type> {
        &self.args
    }
}
