use std::collections::HashMap;

use parse::ScopedId;

/// Generic structure used by the `SymbolTableChecker` to build the symbol table.
///
/// # Motivation
/// Because of block indentation rules, symbol tables must generally be stored in a tree
/// structure to prevent variables from different blocks from colliding with each other:
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
/// There are root blocks, child blocks, and sibiling blocks to differentiate.
///
/// Because Rust's support for defining complex data structures recursively, and updating them
/// (especially with forwards and backwards search) is not very great, we chose a simpler apprach:
/// store the symbols with a special `ScopedId`, a list of indices into the scope tree.
/// For example, `z` on line 7 has index `[1, 0]` as it's the first declaration in the second
/// block child of the root.
///
/// In order to build this representation, however, we don't need a tree. Instead, we keep track of
/// this index as we construct the table. We associate a `ScopedId` with each encountered
/// variable reference. We only need a stack of `HashMap<String, Symbol>` when going through the
/// AST in order to identify where a variable came from.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct SymbolTableBuilder {
    scopes: Vec<HashMap<String, ScopedId>>
}

impl SymbolTableBuilder {
    /// Create a new empty lexical scope manager.
    ///
    /// It starts off with a global scope.
    pub fn new() -> SymbolTableBuilder {
        SymbolTableBuilder { scopes: vec![HashMap::new()] }
    }

    /// Create a new scope
    pub fn new_scope(&mut self) {
        self.scopes.push(HashMap::new())
    }

    // Pop the topmost scope from the stack
    pub fn pop(&mut self) -> Option<HashMap<String, ScopedId>> {
        self.scopes.pop()
    }

    /// Define a new variable in the local scope
    pub fn define_local(&mut self, name: String, value: ScopedId) {
        debug_assert!(!self.scopes.is_empty(),
            "Attempted to define variable {} with no scopes", name);
        let last_ix = self.scopes.len() - 1usize;
        trace!("Defining {} in scope {}", &name, last_ix);
        &mut self.scopes[last_ix].insert(name, value);
    }

    /// Define a variable in the global scope
    pub fn define_global(&mut self, name: String, value: ScopedId) {
        debug_assert!(!self.scopes.is_empty(),
            "Attempted to define a global {} with no scopes", name);
        &mut self.scopes[0].insert(name, value);
    }

    /// Get a variable from any scope
    pub fn get(&self, name: &str) -> Option<&ScopedId> {
        trace!("Searching for {} in {:#?}", name, self);
        debug_assert!(!self.scopes.is_empty(),
            "Attempted to search for a variable {} with no scopes", name);
        for scope in self.scopes.iter().rev() {
            trace!("Checking for {} in scope {:?}", name, scope);
            if let Some(var_ref) = scope.get(name) {
                return Some(var_ref)
            }
        }
        trace!("Didn't find {}", name);
        None
    }

    /// Get a variable defined in local scopeh
    pub fn get_local(&self, name: &str) -> Option<&ScopedId> {
        debug_assert!(!self.scopes.is_empty(),
            "Attempted to get local var {} with no scopes", name);
        let local_scope_ix = self.scopes.len() - 1usize;
        self.scopes[local_scope_ix].get(name)
    }

    /// Get a scopeIndex defined at the global level.
    pub fn get_global(&self, name: &str) -> Option<&ScopedId> {
        debug_assert!(!self.scopes.is_empty(),
            "Attempted to get global var {} with no scopes", name);
        self.scopes[0].get(name)
    }

    /// Get a variable, starting from the given scope
    pub fn get_in_scope(&self, name: &str, scope_level: usize) -> Option<&ScopedId> {
        debug_assert!(self.scopes.len() >= scope_level,
            "Do not have {} scopes to search, only have {}", scope_level, self.scopes.len());
        for scope in self.scopes[0..scope_level].iter().rev() {
            if let Some(var_ref) = scope.get(name) {
                return Some(var_ref)
            }
        }
        None
    }
}
