#![allow(dead_code, unused_imports)]


#[macro_use]
extern crate maplit;
extern crate unicode_categories;

pub mod lex;
pub mod parse;
pub mod compile;
pub mod run;

#[cfg(test)]
mod tests;
