use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

use parse::ScopedId;

/// A `ScopeBuilder` which handles named things defined in different
/// scopes, suitable for most `ScopeBuilder` uses.
pub type NameScopeBuilder = ScopeBuilder<String>;

/// Generic structure to build a scope mapping.
///
/// # Motivation
/// Many parts of the AST use `ScopedId` to map symbols to complex data
/// structures, like mapping between variable names and types, or function
/// definitions and signatures.
///
/// These structures usually have some sort of tree structure based on scope,
/// where the `ScopedId` must be carefully `.increment()`ed or `.push()`ed.
/// For example, scoping variables:
/// ```text
/// let x = 0
/// do
///     let y = 0
///     let x = 0
/// let y = 0
/// do
///     let z = 1
///     let y = 0
/// ```
/// There are differnt `ScopedId`s being mached to the `x` at different lines.
/// This analysis is also done for types and there it should mostly be based
/// on module scoping.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct ScopeBuilder<T: Debug + Hash + Eq> {
    scopes: Vec<HashMap<T, ScopedId>>,
    defined: HashSet<ScopedId>
}

impl<T> ScopeBuilder<T> {
    /// Create a new empty lexical scope manager
    pub fn new() -> ScopeBuilder<T> {
        ScopeBuilder { scopes: vec![] }
    }

    /// Create a new scope
    pub fn new_scope(&mut self) {
        self.scopes.push(HashMap::new())
    }

    // Pop the topmost scope from the stack
    pub fn pop(&mut self) -> Option<HashMap<T, ScopedId>> {
        self.scopes.pop()
    }

    /// Define a new variable in the local scope
    pub fn define_local(&mut self, key: T, value: ScopedId) {
        debug_assert!(!self.scopes.is_empty(),
            "Attempted to define {:?} with no scopes", key);
        let last_ix = self.scopes.len() - 1usize;
        trace!("Defining {:?} in scope {}", &key, last_ix);
        self.defined.insert(value.clone());
        &mut self.scopes[last_ix].insert(key, value);
    }

    /// Define a variable in the global scope
    pub fn define_global(&mut self, key: T, value: ScopedId) {
        debug_assert!(!self.scopes.is_empty(),
            "Attempted to define a global {:?} with no scopes", key);
        self.defined.insert(value.clone());
        &mut self.scopes[0].insert(key, value);
    }

    /// Get a variable from any scope
    pub fn get<K: Borrow<T> + Debug>(&self, key: &K) -> Option<&ScopedId> {
        trace!("Searching for {} in {:#?}", key, self);
        debug_assert!(!self.scopes.is_empty(),
            "Attempted to search for a variable {:?} with no scopes", key);
        for scope in self.scopes.iter().rev() {
            trace!("Checking for {:?} in scope {:?}", key, scope);
            if let Some(var_ref) = scope.get(key) {
                return Some(var_ref)
            }
        }
        trace!("Didn't find {:?}", key);
        None
    }

    /// Check if the `ScopedId` has been defined.
    pub fn contains_id(&self, id: &ScopedId) -> bool {
        trace!("Checking if {:?} is defined", id);
        self.defined.contains(id)
    }

    /// Get a variable defined in local scopeh
    pub fn get_local<K: Borrow<T> + Debug>(&self, key: &K)
                                           -> Option<&ScopedId> {
        debug_assert!(!self.scopes.is_empty(),
            "Attempted to get local var {:?} with no scopes", key);
        let local_scope_ix = self.scopes.len() - 1usize;
        self.scopes[local_scope_ix].get(key)
    }

    /// Get a variable, starting from the given scope
    pub fn get_in_scope<K>(&self, key: &K, scope_level: usize)
                           -> Option<&ScopedId> where K: Borrow<T> + Debug {
        debug_assert!(self.scopes.len() >= scope_level,
            "Do not have {} scopes to search, only have {}",
            scope_level, self.scopes.len());
        for scope in self.scopes[0..scope_level].iter().rev() {
            if let Some(var_ref) = scope.get(key) {
                return Some(var_ref)
            }
        }
        None
    }
}
