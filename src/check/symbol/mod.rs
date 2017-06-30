//! Gives unique IDs to type `Identifier`s in the AST.
//!
//! The code in this module is built around the `SymbolChecker`
//! which IDs `Identifier`s in type expressions and type declarations.
//!
//! Consider the following code:
//! ```text
//! struct Foo
//!     field: Type
//!
//!     fn create() -> Foo
//!         ...
//!
//! fn bar() -> Foo
//!     let mut foo: Foo = Foo.create()
//!     foo.update()
//!     foo
//! ```
//! The job of the `SymbolChecker` is to ID `Foo` and other type-related
//! identifiers. This gives the type inference system IDs to work with and
//! can be done separately from identifiers.
//! In the future, this could be used to check whether methods on types exist
//! in the same way as the `ScopeChecker` makes sure variables are initialized
//! and mutable.

mod symbol;
mod symbol_checker;
mod table_builder;
mod item_checker;

pub use self::symbol::*;
pub use self::symbol_checker::SymbolChecker;
