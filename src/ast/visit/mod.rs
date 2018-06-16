//! Code for AST walking.

pub mod visitor;
mod default; // Default visitors are reexported in `visitor`.
mod walk;

pub use self::walk::*;
