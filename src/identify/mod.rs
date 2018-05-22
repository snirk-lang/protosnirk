//! Scope checking: giving `Identifier`s `ScopedId`s.
//!
//! # Overview
//!
//! The `identify` module of primarily deals with giving identifiers
//! of various kinds unique `ID`s for the purpose of later analysis. This
//! includes things like detecting whether some variable or name is being
//! used without being declared (i.e. detecting a typo) and setting up
//! later passes to resolve types, mapping the structure of declared types
//! or pinning function signature types to invocations of the functions.
//!
//! Scoping is usually split into two parts: an initial `Item` pass which
//! resolves global names (declared types in one id-space, declared names in
//! another) and one which resolves identifiers within functions (and is thus
//! able to recognize i.e. a function being called earlier in a file before
//! being declared).
//!
//! The purpose of this system is to mitigate the need for some transformation
//! passes. The `ScopedId` of identifiers is the only metadata which appears
//! in the AST itself, and by using it we are able to create mappings of
//! `ID` to other symbolic data, such as types, scopes, lifetimes, symbols, etc.
//!
//! Although some forms of errors can be detected this early, we may want to
//! continue parsing until we can get a better picture.
//!
//! # Relevant structures
//!
//! This module contains the `ASTIdentifier`, which fills in the
//! `ScopedId` of nodes with `Identifier`s in the parsed AST.
//!
//! This pass alters the `Unit` in-place (using `Cell` and `RefCell`).
//!
//! # Invariants from this pass
//!
//! - Calling `.get_id()` on an `Identifier` in the `AST` should yield a valid
//! (non-default) `ScopedId` if the `Identifier` is being used in valid code.
//! This applies to identifiers used for variables in expressions _and_ for
//! identifiers used in type expressions.
//! - Getting a default `ScopedId` from a call to `get_id()` is an indication of
//! a variable or type not being defined or possibly being defined twice.
//!
//! In the future errors will be held in a `ScopeErrorMap` structure.

mod names;
mod concrete_type;
mod types;
mod scope_builder;
mod type_scope_builder;
pub use self::scope_builder::{ScopeBuilder, NameScopeBuilder};
pub use self::type_scope_builder::TypeScopeBuilder;
pub use self::concrete_type::*;

use ast::{Unit, ScopedId};
use check::ErrorCollector;
use visit::visitor::UnitVisitor;

use self::names::*;
use self::types::*;

/// Identifies `Ident`s in the AST.
///
/// Each `Identifier`'s `ScopedId` is set based on whether
/// it appears in an expression context or type context.
/// The IDs take scoping rules into account, identifying
/// types and variables with unique IDs.
#[derive(Debug, PartialEq)]
pub struct ASTIdentifier<'var_scope, 'ty_scope, 'err> {
    var_scope: &'var_scope mut NameScopeBuilder,
    type_scope: &'ty_scope mut TypeScopeBuilder,
    errors: &'err mut ErrorCollector
}
impl<'var_scope, 'ty_scope, 'err> ASTIdentifier<'var_scope, 'ty_scope, 'err> {
    pub fn new(var_scope: &'var_scope mut NameScopeBuilder,
               type_scope: &'ty_scope mut TypeScopeBuilder,
               errors: &'err mut ErrorCollector)
               -> ASTIdentifier<'var_scope, 'ty_scope, 'err> {
        ASTIdentifier { var_scope, type_scope, errors }
    }
}

impl<'var_scope, 'ty_scope, 'err> UnitVisitor
                                for ASTIdentifier<'var_scope, 'ty_scope, 'err> {
    fn visit_unit(&mut self, unit: &Unit) {
        // The ItemVarIdentifier uses its ScopedId to set up actual scoping.
        // This could be handled by ScopeBuilder.
        ItemVarIdentifier::new(self.errors, self.var_scope, ScopedId::default())
                          .visit_unit(unit);
        ItemTypeIdentifier::new(self.errors, self.type_scope)
                           .visit_unit(unit);
        ExpressionVarIdentifier::new(self.errors, self.var_scope)
                                .visit_unit(unit);
        ExprTypeIdentifier::new(self.errors, self.type_scope)
                           .visit_unit(unit);
    }
}
