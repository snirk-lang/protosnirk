use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use check::Symbol;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ScopeIndex {
    indices: Vec<usize>
}
impl ScopeIndex {
    #[inline]
    pub fn increment(&mut self) {
        trace!("Calling increment on {:?}", self);
        let len = self.indices.len() - 1;
        self.indices[len] += 1;
    }
    #[inline]
    pub fn decrement(&mut self) {
        trace!("Calling decrement on {:?}", self);
        let len = self.indices.len() - 1;
        self.indices[len] -= 1;
    }
    #[inline]
    pub fn push(&mut self) {
        trace!("Calling push on {:?}", self);
        self.indices.push(0);
    }
    #[inline]
    pub fn pop(&mut self) {
        trace!("Calling pop on {:?}", self);
        self.indices.pop();
    }
    pub fn new(vec: Vec<usize>) -> ScopeIndex {
        trace!("Created new scope {:?}", vec);
        ScopeIndex { indices: vec }
    }
}

impl Default for ScopeIndex {
    fn default() -> ScopeIndex {
        trace!("Creating default scope");
        ScopeIndex { indices: vec![0usize] }
    }
}
