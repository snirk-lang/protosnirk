#![allow(dead_code, unused_imports)]

#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate maplit;
extern crate unicode_categories;
extern crate llvm_sys;

extern crate libc;

pub mod lex;
pub mod parse;
#[macro_use]
pub mod llvm;
pub mod check;
pub mod compile;

#[cfg(test)]
mod tests;
