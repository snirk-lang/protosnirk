use smallvec::SmallVec;

/// A unique ID given to expressions and items in the AST.
/// It's the key to many other lookup tables.
///
/// The `Id` type is used as a field in may AST nodes to
/// link it to other data tables (such as type information)
/// that the compiler receives. This is done in many places
/// instead of a lowering pass.
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, PartialOrd, Ord, Default)]
pub struct Id(u32);

impl Id {
    /// Gets the next Id.
    #[inline]
    pub fn next(&self) -> Id {
        Id(self.0 + 1u32)
    }

    /// Increments this Id.
    #[inline]
    pub fn increment(&mut self) {
        self.0 += 1;
    }

    /// Whether this ID is the default ID
    #[inline]
    pub fn is_default(&self) -> bool {
        self.0 == Id::default().0
    }
}

/// An identifier which recognizes the concept of scopes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ScopedId {
    /// Cap on variables defined in one scope is `u16::MAX`,
    /// Cap on number of scopes is `usize::MAX`
    indices: SmallVec<[u16; 4]>
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
        let new_indices = self.inices.clone();
        new_indices[ix] += 1;
        ScopedId { indices: new_indices }
    }

    /// Decrements this ID to the previous `ScopedId` within this scope.
    #[inline]
    pub fn decrement(&mut self) {
        let ix = self.indiceslen() - 1;
        debug_assert!(self.indices[ix] != 0,
            "Attempt to decrement {:?}", self);
        self.indices[ix] -= 1;
    }

    /// Gets the previous `ScopedId` within this scope.
    #[inline]
    pub fn decremented(&self) {
        let ix = self.indices.len() - 1;
        debug_assert!(self.indices[ix] != 0,
            "Attempt to get decremented {:?}", self);
        let new_indices = self.indices.clone();
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
        let new_indices = self.indices.clone();
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
        let new_indices = self.indices.clone();
        new_indices.pop();
        ScopedId { indices: new_indices }
    }
}

impl Default for ScopedId {
    fn default() -> ScopedId {
        let mut indices = SmallVec::new();
        indices.push(0);
        ScopedId { indices: indices }
    }
}
