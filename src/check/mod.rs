//! Visitors which validate a parsed protosnirk program.
//!
//! This module is concerned with ensuring that the
//! program accessed from the parser is legal: it passes type
//! checking and lifetime checks.
//!
//! Checkers may attach metadata to the AST which will use each AST's
//! `index`, such as a symbol table or type table.
//!
//! ## Verifier Results
//! The verifier will either give a successful `Program` (with metadata the
//! compiler needs, like a symbol and type table), or a collection of
//! `VerifierError`s.
//!
//! ### Errors
//! These are errors that are identified by various checks.
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
//! fn foo() => 1
//! let x = 1 + foo
//!             ^ `foo`: expected integer type (for addition expression)
//!               `foo` is of type `fn() -> int`
//! ```
//! ### Warnings
//! If a checked `Program` has only warnings,
//! it is considered compileable.
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

mod program;
mod visitor;
mod collector;
mod errors;

mod scope;
mod symbol;
mod lint;

mod unit_checker;

pub mod types;

pub use self::visitor::{ASTVisitor}; // Allow external use of the trait (i.e. in compiler)
pub use self::collector::ErrorCollector;
pub use self::errors::CheckerError;
pub use self::program::Program;

pub use self::symbol::*;

pub use self::unit_checker::UnitChecker;

use std::collections::HashMap;
use parse::{Id, ScopedId};
use self::types::Type;

/// Mapping of ScopeIndex to Symbol. Each correct variable reference
/// has a scopedId, and each corresponds to a Symbol with information
/// about that reference
pub type SymbolTable = HashMap<ScopedId, Symbol>;

/// Each symbol has a TypeId, and each TypeId corresponds to a given
/// type.
pub type TypeTable = HashMap<Id, Type>;
