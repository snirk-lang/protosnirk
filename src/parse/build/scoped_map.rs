use std::collections::HashMap;
use std::default::Default;
use std::fmt::Debug;
use std::hash::Hash;

/// Generic strucutre for mapping scoped variable names to values
#[derive(Debug, PartialEq, Clone, Default)]
pub struct ScopedMap<T> {
    scopes: Vec<Vec<HashMap<String, T>>>
}

impl<T: Hash + Debug> ScopedMap<T> {
    /// Create a new empty lexical scope manager
    pub fn new() -> ScopedMap<T> {
        ScopedMap { scopes: vec![] }
    }

    /// Create a new scope
    pub fn new_scope(&mut self) {
        self.scopes.push(HashMap::new())
    }

    // Pop the topmost scope from the stack
    pub fn pop(&mut self) -> Option<HashMap<String, T>> {
        self.scopes.pop()
    }

    /// Define a new variable in the local scope
    pub fn define_local(&mut self, name: String, value: T) {
        debug_assert!(!self.scopes.is_empty(),
            "Attempted to define variable {} with no scopes", name);
        let last_ix = self.scopes.len() - 1usize;
        trace!("Defining {} in scope {}", &name, last_ix);
        &mut self.scopes[last_ix].insert(name, value);
    }

    /// Get a variable from any scope
    pub fn get(&self, name: &str) -> Option<(&T, usize)> {
        trace!("Searching for {} in {:#?}", name, self);
        debug_assert!(!self.scopes.is_empty(),
            "Attempted to search for a variable {} with no scopes", name);
        for (ix, scope) in self.scopes.iter().rev().enumerate() {
            trace!("Checking for {} in scope {}", name, ix);
            if let Some(var_ref) = scope.get(name) {
                return Some((var_ref, ix))
            }
        }
        trace!("Didn't find {}", name);
        None
    }

    /// Get a variable defined in local scope
    pub fn get_local(&self, name: &str) -> Option<&T> {
        debug_assert!(!self.scopes.is_empty(),
            "Attempted to get local var {} with no scopes", name);
        let local_scope_ix = self.scopes.len() - 1usize;
        self.scopes[local_scope_ix].get(name)
    }

    /// Get a variable, starting from the given scope
    pub fn get_in_scope(&self, scope_level: usize) -> Option<(&T, usize)> {
        debug_assert!(self.scopes.len() >= scope_level,
            "Do not have {} scopes to search, only have {}", scope_level, self.scopes.len());
        for (ix, scope) in self.scopes[0..scope_level].iter().rev().enumerate() {
            if let Some(var_ref) = scope.get(name) {
                return Some((var_ref, ix))
            }
        }
        None
    }
}
