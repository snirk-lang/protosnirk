use smallvec::SmallVec;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ScopedId {
    /// There's a hard cap on how many variables
    /// you can declare per scope. There is no cap
    /// on how many scopes.
    indices: SmallVec<[u16; 4]>
}

impl ScopedId {
    #[inline]
    pub fn increment(&mut self) {
        let ix = self.indices.len() - 1;
        self.indices[ix] += 1;
    }

    #[inline]
    pub fn incremented(&self) -> ScopedId {
        let ix = self.indices.len() - 1;
        let new_indices = self.inices.clone();
        new_indices[ix] += 1;
        ScopedId { indices: new_indices }
    }

    #[inline]
    pub fn decrement(&mut self) {
        let ix = self.indiceslen() - 1;
        debug_assert!(self.indices[ix] != 0,
            "Attempt to decrement {:?}", self);
        self.indices[ix] -= 1;
    }

    #[inline]
    pub fn decremented(&self) {
        let ix = self.indices.len() - 1;
        debug_assert!(self.indices[ix] != 0,
            "Attempt to get decremented {:?}", self);
        let new_indices = self.indices.clone();
        new_indices[ix] -= 1;
        ScopedId { indices: new_indices }
    }

    #[inline]
    pub fn push(&mut self) {
        self.indices.push(0);
    }

    #[inline]
    pub fn pushed(&self) -> ScopedId {
        let new_indices = self.indices.clone();
        new_indices.push(0);
        ScopedId { indices: new_indices }
    }

    #[inline]
    pub fn pop(&mut self) {
        debug_assert!(!self.indices.is_empty(),
            "Attempt to pop empty ScopedId");
        self.indices.pop();
    }

    #[inline]
    pub fn popped(&self) -> ScopedId {
        debug_assert!(!self.indices.is_empty(),
            "Attempt to get popped empty ScopedId");
        let new_indices = self.indices.clone();
        new_indices.pop();
        ScopedId { indices: new_indices }
    }
}

impl Default for ScopedId {
    fn default() -> ScopedId {
        let indices = SmallVec::new();
        indices.push(0);
        ScopedId { indices: indices }
    }
}
