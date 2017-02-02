use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct ScopeIndex {
    indices: Vec<usize>
}
impl ScopeIndex {
    #[inline]
    pub fn increment(&mut self) {
        self.indices[self.indices.len() - 1] += 1;
    }
    #[inline]
    pub fn decrement(&mut self) {
        self.indices[self.indices.len() - 1] -= 1;
    }
    #[inline]
    pub fn push(&mut self) {
        self.indices.push(0);
    }
    #[inline]
    pub fn pop(&mut self) {
        self.indices.pop();
    }
}

#[derive(Debug, Clone)]
pub struct ScopedTable<T: Debug> {
    values: HashMap<ScopeIndex, T>
}

impl<T: Debug> ScopedTable<T> {
    pub fn new() -> ScopedTable<T> {
        ScopedTable { values: hashmap![] }
    }
}

impl<T: Debug> Deref for ScopedTable<T> {
    type Target = HashMap<ScopeIndex, T>;
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl<T: Debug> DerefMut for ScopedTable<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}
