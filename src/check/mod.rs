//! This module is currently responsible for a generic error type.
//!
//! This will be moved in the future.

mod collector;
mod errors;

pub use self::collector::ErrorCollector;
pub use self::errors::CheckerError;
