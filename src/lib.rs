#![allow(dead_code, unused_imports)]

extern crate unicode_categories;
extern crate unicode_segmentation;
#[macro_use]
extern crate maplit;

mod lex;
mod parse;
mod compile;
mod run;

#[cfg(test)]
mod tests;
