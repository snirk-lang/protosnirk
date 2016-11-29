#![allow(dead_code, unused_imports)]


#[macro_use]
extern crate maplit;
#[macro_use]
extern crate nom;
extern crate unicode_categories;
extern crate unicode_segmentation;


mod lex;
mod parse;
mod compile;
mod run;

#[cfg(test)]
mod tests;
