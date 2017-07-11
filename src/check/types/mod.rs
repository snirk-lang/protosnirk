//! Definition of data types in a compiled protosnirk program.

mod types;
mod environment;

mod expression_collector;
mod item_collector;
mod type_checker;

pub use self::type_checker::TypeChecker;
pub use self::environment::*;
pub use self::types::*; // We're defining parts of the post-AST IR glue here.
