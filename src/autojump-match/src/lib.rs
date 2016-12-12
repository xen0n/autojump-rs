extern crate regex;
extern crate strsim;

mod fuzzy;
mod matcher;
mod re_based;

pub use matcher::*;


#[cfg(test)]
mod tests;
