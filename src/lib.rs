//! The protosnirk compiler library.
//!
//! # Organization
//!
//! This library is roughly organized by data structures and passes.
//! For the most part, each module in the order depends only on those that
//! came before it.
//!
//! # Modules
//!
//! ## `Lex`
//!
//! Responsible for tokenizinig an input stream of text.
//!
//! See `lex::Token`, `lex::Tokenizer`.
//!
//! ## `Parse`
//!
//! Responsible for parsing the protosnirk AST. Contains definitions of the
//! basic AST. Also includes identifier types that are used by later passes.
//!
//! See `parse::ast`, `parse::ast::Unit`, `parse::ScopedId`, `parse::errors`,
//! `parse::symbol`.
//!
//! ## `Visit`
//!
//! Visitor traits for the AST. These are used to organize later passes.
//!
//! See `visit::visitor`, `visit::walk`.
//!
//! ## `Check`
//!
//! This is temporarily included for access to `CheckerError`, which is the
//! basic form of error handling currently used. More pass-specific error
//! handling will be added in the future.
//!
//! ## `Identify`
//!
//! This pass sets `ScopedId`s on the AST. This sets up lexical scoped naming
//! of data in the AST, as well as setting up the names of types.
//!
//! See `identify::scope_builder`, `identify::ASTIdentifier`.
//!
//! ## `TypeInfer`
//!
//! This pass runs unification-based type inference on the AST, and sets the
//! `TypeIds` of `Identifier`s on the AST to map to `ConcreteType`s which can
//! be used by later passes.
//!
//! See `typeinfer::ConcreteType`, `typeinfer::TypeInferrer`.
//!
//! ## `Lint`
//!
//! Run lints on the verified AST. In addition to complaints like bad names,
//! lints should in the future catch logic errors where possible.
//!
//! See `lint::UsageChecker`.
//!
//! ## `Compile`
//!
//! Compile code to the LLVM IR. This is currently unoptimized.
//!
//! See `compile::ModuleCompiler`.
//!
//! ## `Pipeline`
//!
//! Orchastrate the compilation process.

#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate maplit; // Convenience for initializing `HashMap`s
extern crate unicode_categories; // Used by tokenizer for valid idents
extern crate smallvec; // Optimize storage of ScopedIds
extern crate petgraph; // Type inference unification algorithm
extern crate llvm_sys; // LLVM bindings
extern crate libc; // LLVM Bindings

pub mod lex;
pub mod ast;
pub mod parse;
pub mod llvm;
pub mod identify;
pub mod check;
pub mod lint;
pub mod compile;
pub mod pipeline;

#[cfg(test)]
mod tests;
