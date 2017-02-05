//! Visitors which validate a parsed protosnirk program.
//!
//! This module is concerned with ensuring that the
//! program accessed from the parser is valid: it adhers
//! to protosnirk's semantic rules in addition to syntax rules.
//!
//! In addition, metadata is added to the expression tree, such as
//! the symbol table. The `verify` module will also handle type inference.
//!
//! # Parser Steps
//! The parser handles some of the basics.
//!
//! ### lvalue vs rvalue
//! ```text
//! x + 2 = 5
//!   ^ lvalue needed here
//!
//! let 4 = y
//!     ^ lvalue needed here
//! ```
//!
//! ### Expression vs Statement (Parser)
//! ```text
//! let x = y += 2
//!            ^ expression needed here
//!
//! return let x = 0
//!            ^ expression needed here
//! ```
//!
//! # Verifier Steps
//! These are the passes the verifier makes
//! ### Unknown identifier
//! ```text
//! let x = y
//!         ^ Unknown identifier `y`
//! z += 2
//! ^ Unknown identifier `z`
//! ```
//! ### Variable already defined
//! ```text
//! let mut y = 0
//! let y = 0
//!     ^ Variable `y` already defined on line x
//! ```
//! ### Attempt to reassign immutable value
//! ```text
//! y = 12
//!   ^ Cannot reassign immutable variable `y` defined on line x
//!
//! z += 12
//!   ^ Cannot reassign immutable variable `z` defined on line x
//! ```
//! ## Warnings
//!
//! ### Unused mutable
//! ```text
//! let mut var = 0
//!         ^ WARN unusedMutable
//!         ^ `var` is declared mutable but not mutated
//! return var
//! ```
//! ### Unused variable
//! ```text
//! let x = 0
//!     ^ WARN unusedVariable
//!     ^ `var` is declared but not used
//! return y
//! ```
pub mod checker;
pub mod scope;
mod symbol;
mod verification_result;
mod collector;
mod verifier;

pub use self::symbol::*;
pub use self::collector::ErrorCollector;
pub use self::verifier::Verifier;
pub use self::verification_result::VerifyError;
