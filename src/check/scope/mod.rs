//! Scope checking: giving `Identifier`s `ScopedId`s.
//!
//! The `scope` module of `check` primarily deals with giving identifiers
//! of various kinds unique `ID`s for the purpose of later analysis. This
//! includes things like detecting whether some variable or name is being
//! used without being declared (i.e. detecting a typo) and setting up
//! later passes to resolve types, mapping the structure of declared types
//! or pinning function signature types to invocations of the functions.
//!
//! Scoping is split into two parts: an initial `Item` pass which resolves
//! global names (declared types in one id-space, declared names in another)
//! and one which resolves identifiers within functions (and is thus able
//! to recognize i.e. a function being called after it has been declared).
//!
//! The purpose of this system is to mitigate the need for some transformation
//! passes. The `ScopedId` of identifiers is the only metadata which appears
//! in the AST itself, and by using it we are able to create mappings of
//! `ID` to other symbolic data, such as types, scopes, lifetimes, symbols, etc
//!
//! Although some forms of errors can be detected this early, we may want to
//! continue parsing until we can get a better picture
//!
//! In the future this module should be split up to deal with types and methods
//! separately.

mod expression_identifier;
mod type_identifier;
mod scope_checker;
mod scope_builder;

pub use self::scope_checker::ScopeChecker;
pub use self::scope_builder::ScopeBuilder;
