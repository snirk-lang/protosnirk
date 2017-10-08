use smallvec::SmallVec;

/// A unique identifier for the type of an identifier on the AST.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord, Default)]
pub struct TypeId(u32);

impl TypeId {
    /// Gets the next Id.
    #[inline]
    pub fn next(&self) -> TypeId {
        TypeId(self.0 + 1u32)
    }

    /// Increments this Id.
    #[inline]
    pub fn increment(&mut self) {
        self.0 += 1;
    }

    /// Whether this ID is the default ID
    #[inline]
    pub fn is_default(&self) -> bool {
        self.0 == TypeId::default().0
    }
}

/// An identifier which recognizes the concept of scopes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ScopedId {
    /// Cap on variables defined in one scope is `u16::MAX`,
    /// Cap on number of scopes is `usize::MAX`
    ///
    /// On 64-bit machines, the size/align of `SmallVec<[u16; 11]>` is the
    /// same as `SmallVec<[u16; 1]>` so we don't save space by cacing fewer
    /// indices before allocating.
    /// TODO get numbers for this on 32-bit.
    indices: SmallVec<[u16; 11]>
}

impl ScopedId {
    /// Increments this ID to the next `ScopedId` within this scope.
    #[inline]
    pub fn increment(&mut self) {
        let ix = self.indices.len() - 1;
        self.indices[ix] += 1;
    }

    /// Gets the next `ScopedId` within this scope.
    #[inline]
    pub fn incremented(&self) -> ScopedId {
        let ix = self.indices.len() - 1;
        let mut new_indices = self.indices.clone();
        new_indices[ix] += 1;
        ScopedId { indices: new_indices }
    }

    /// Decrements this ID to the previous `ScopedId` within this scope.
    #[inline]
    pub fn decrement(&mut self) {
        let ix = self.indices.len() - 1;
        debug_assert!(self.indices[ix] != 0,
            "Attempt to decrement {:?}", self);
        self.indices[ix] -= 1;
    }

    /// Gets the previous `ScopedId` within this scope.
    #[inline]
    pub fn decremented(&self) -> ScopedId {
        let ix = self.indices.len() - 1;
        debug_assert!(self.indices[ix] != 0,
            "Attempt to get decremented {:?}", self);
        let mut new_indices = self.indices.clone();
        new_indices[ix] -= 1;
        ScopedId { indices: new_indices }
    }

    /// Pushes this `ScopedId` to the next scope.
    #[inline]
    pub fn push(&mut self) {
        self.indices.push(0);
    }

    /// Gets a `ScopedId` pushed to the next scope.
    #[inline]
    pub fn pushed(&self) -> ScopedId {
        let mut new_indices = self.indices.clone();
        new_indices.push(0);
        ScopedId { indices: new_indices }
    }

    /// Pops this `ScopedId` to the previous scope.
    #[inline]
    pub fn pop(&mut self) {
        debug_assert!(!self.indices.is_empty(),
            "Attempt to pop empty ScopedId");
        self.indices.pop();
    }

    /// Gets a `ScopedId` popped to the previous scope.
    #[inline]
    pub fn popped(&self) -> ScopedId {
        debug_assert!(!self.indices.is_empty(),
            "Attempt to get popped empty ScopedId");
        let mut new_indices = self.indices.clone();
        new_indices.pop();
        ScopedId { indices: new_indices }
    }

    /// Whether another scopedId has a common prefix with this one.
    pub fn is_subindex_of(&self, other: &ScopedId) -> bool {
        other.indices.len() >= self.indices.len() &&
            &other.indices[0..self.indices.len() - 1] == &*self.indices
    }

    pub fn is_default(&self) -> bool {
        *self.indices == [0]
    }
}

impl Default for ScopedId {
    fn default() -> ScopedId {
        let mut indices = SmallVec::new();
        indices.push(0);
        ScopedId { indices: indices }
    }
}
