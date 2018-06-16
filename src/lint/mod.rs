//! Lint a verified AST for possible programmer forgetfulness.
//!
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

//mod usage_checker;

//pub use self::usage_checker::UsageChecker;
