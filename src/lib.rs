#[macro_use]
extern crate nom;

mod lex;
mod parse;
mod compile;
mod run;

mod example;

#[cfg(test)]
mod tests;
