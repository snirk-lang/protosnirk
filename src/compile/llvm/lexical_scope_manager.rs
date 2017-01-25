use std::collections::HashMap;
use std::default::Default;
use std::hash::Hash;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct LexicalScopeManager<T> {
    scopes: Vec<HashMap<String, T>>
}

impl<T: Hash> LexicalScopeManager<T> {
    /// Create a new empty lexical scope manager
    pub fn new() -> LexicalScopeManager<T> {
        LexicalScopeManager { scopes: vec![] }
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
        &mut self.scopes[last_ix].insert(name, value);
    }

    /// Get a variable from any scope
    pub fn get(&self, name: &str) -> Option<(&T, usize)> {
        debug_assert!(!self.scopes.is_empty(),
            "Attempted to search for a variable {} with no scopes", name);
        for (ix, scope) in self.scopes.iter().enumerate() {
            if let Some(var_ref) = scope.get(name) {
                return Some((var_ref, ix))
            }
        }
        None
    }

    /// Get a variable defined in local scope
    pub fn get_local(&self, name: &str) -> Option<&T> {
        debug_assert!(!self.scopes.is_empty(),
            "Attempted to get local var {} with no scopes", name);
        let local_scope_ix = self.scopes.len() - 1usize;
        self.scopes[local_scope_ix].get(name)
    }
}
