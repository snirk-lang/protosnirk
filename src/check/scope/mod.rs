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

mod names;
mod types;
mod scope_builder;

pub use self::scope_builder::{ScopeBuilder, NameScopeBuilder};

use parse::ast::Unit;
use check::ErrorCollector;

use self::names::*;
use self::types::*;

/// Identifies `Ident`s in the AST.
///
/// Each `Identifier`'s `ScopedId` is set based on whether
/// it appears in an expression context or type context.
/// The IDs take scoping rules into account, identifying
/// types and variables with unique IDs.
#[derive(Debug, PartialEq, Clone)]
pub struct ASTIdentifier { }
impl ASTIdentifier {
    pub fn check_unit(&self, errors: &mut ErrorCollector, unit: &Unit) {
        let mut type_scope = types::default_type_scope();
        ItemVarIdentifier::new(errors, types).visit_unit(unit);
        ItemTypeIdentifier::new(errors).visit_unit(unit);
        ExpressionVarIdentifier::new(errors).visit_unit(unit);
        ExpressionTypeIdentifier::new(errors, types).visit_unit(unit);
    }
}
