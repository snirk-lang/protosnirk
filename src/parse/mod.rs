//! Parse an AST from a stream of `Token`s.
//!
//! The parser is responsible for turning the incoming token stream into
//! an AST. It emits errors if the code is written incorrectly (i.e. missing
//! paren, as well as some linting errors such as spacing that occur at the token
//! level.

mod errors;
mod parser;

pub mod symbol;

pub use self::errors::{ParseError, ParseResult, ExpectedNextType};
pub use self::parser::{Parser, IndentationRule};
