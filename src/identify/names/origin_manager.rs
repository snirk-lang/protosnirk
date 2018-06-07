//! Allows `ExpressionVarIdentifier` to keep track of which

use ast::ScopedId;

#[derive(Debug, Clone, Default)]
pub struct OriginManager {
    lvalues: Vec<Option<ScopedId>>
}

impl OriginManager {
    pub fn new() -> OriginManager {
        OriginManager::default()
    }

    /// Add a source to the `OriginManager` of a node
    // with a child that must return it a value.
    pub fn add_source(&mut self, source: ScopedId) {
        self.lvalues.push(Some(source));
    }

    /// Check if the OriginManager has a required source
    pub fn has_source(&self) -> bool {
        self.lvalues.last()
                    .and_then(|top| top.as_ref())
                    .is_some()
    }

    /// Supress `has_source` until `end_block` is called.
    ///
    /// # Usage
    /// This is designed for visiting an expression block:
    /// we want to preserve the original `source` of the
    /// block itself, but we don't want any statements
    /// within the block to think they're returning a value,
    /// expect for the last one.
    pub fn begin_block(&mut self) {
        self.lvalues.push(None);
    }

    /// Pair with `end_block()` to supress `has_source`.
    pub fn end_block(&mut self) -> bool{
        let last = self.lvalues.pop();
        // Assert that the top of the stack is `None`.
        last.is_some() && last.expect("Checked expect").is_none()
    }

    /// Pop the last source from the stack in order to consume it.
    pub fn pop_source(&mut self) -> Option<ScopedId> {
        self.lvalues.pop().and_then(|top| top)
    }
}
