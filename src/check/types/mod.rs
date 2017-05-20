//! Definition of data types in a compiled protosnirk program.

mod inferred;
mod types;
mod type_checker;

pub use self::type_checker::TypeChecker;
pub use self::types::*; // We're defining parts of the post-AST IR glue here.
