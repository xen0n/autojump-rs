extern crate regex;

mod matcher;
mod re_based;

pub use matcher::*;


#[cfg(test)]
mod tests;
