//! # Build
//!
//! This module contains structures and builders that add information to the parse tree
//! in order to create a complete program and pass information to the compiler.
mod symbol;
mod program;

pub use self::symbol::Symbol;
pub use self::program::Program;
