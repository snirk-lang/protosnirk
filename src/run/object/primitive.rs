//! Primitive types
//!
//! The protosnirk primitives live on the stack.
//!
//! Shhhh, protosnirk doesn't know what the heap is yet.

/// Primitive values.
///
/// Primitives are used purely for being _stored on the stack_.
///
/// They are to be _recopied_ when moved.
#[derive(Debug, Clone, Copy)]
pub enum Primitive {
    /// Yup numbers are f64 for now
    Number(f64),
}
