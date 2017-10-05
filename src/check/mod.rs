//! This module is responsible for checking invariants on the AST to
//! ensure an AST represents a legal program.
//!
//! Checking passes may involve referencing data in the AST into another
//! table, such as the type check pass which updates the mapping of
//! identifiers to types.
//!
//! ## Verifier Results
//! The verifier will either give a successful `Program` (with metadata the
//! compiler needs, like a symbol and type table), or a collection of
//! `VerifierError`s.
//!
//! ### Errors
//! These are errors that are identified by various checks.
//!
//! #### Unknown Identifier
//! Probably a typo
//! ```text
//! let x = y
//!         ^ `y` is not defined
//! ```
//! #### Variable already defined
//! ```text
//! let mut y = 0
//! let y = 0
//!     ^ `y` has already been defined on line x
//! ```
//! #### Variable of wrong type
//! ```text
//! fn foo() -> bool
//!    true
//! let x = 1 + foo()
//!             ^ `foo()`: expected integer type (for addition expression)
//!               `foo()` is of type `bool`
//! ```
//! ### Warnings
//! If a checked `Program` has only warnings,
//! it is considered compileable.
//!
//! #### Unused mutable
//! ```text
//! let mut var = 0
//!         ^ `var` is declared mutable but not mutated
//! return var
//! ```
//! #### Unused variable
//! ```text
//! let x = 0
//!     ^ `x` is declared but not used
//! return y
//! ```
//! #### Unused function
//! ```text
//! fn foo() -> bool
//!    ^ `foo` is declared but not used
//!     true
//! ```
//!

mod program;
pub mod visitor;
mod collector;
mod errors;

pub mod scope;
pub mod lint;
pub mod types;

pub use self::collector::ErrorCollector;
pub use self::errors::CheckerError;
pub use self::program::Program;
