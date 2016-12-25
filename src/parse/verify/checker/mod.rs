//! Checkers used to verify the expression trees created by the parser.
//!
//! Checkers have two purposes: check the parse tree for semantic errors,
//! (attemtped mutation of immutable variable) and build up representations
//! (such as the symbol table) of the parsed program to be used by the compiler.
//!
//! The verifiers in this module will build structures from the `build` module.
mod symbol_checker;
mod usage_checker;
mod constant_checker;

pub use self::symbol_checker::SymbolTableChecker;
pub use self::usage_checker::UsageChecker;
pub use self::constant_checker::ConstantAssembler;
