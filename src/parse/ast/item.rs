//! Item values
//!
//! An `Item` is a declaration made in the root context of a program
//! -- namely the imort item `use`, and declarations such as `class`,
//! `enum`, `struct`.

use parse::ast::Block;

// This will expand greatly in the future, but for now it's a solid way
// to have an "enty point" in the compiler (and allow nested blocks)

/// A single "unit" of computation.
///
/// In the future this should be an entire progam definition,
/// complete with lists of defined types, functions, etc.
#[derive(Debug, Clone, PartialEq)]
pub struct Unit {
    pub block: Block
}

impl Unit {
    /// Create a new unit with the given block
    pub fn new(block: Block) -> Unit {
        Unit { block: block }
    }
}
