//! Code for AST walking.

mod visitor;
mod default; // Default visitors are reexported in `visitor`.
mod walk;

pub use self::walk::*;
pub use self::visitor::*;
