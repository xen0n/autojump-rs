#![cfg_attr(feature = "nightly", feature(conservative_impl_trait))]
#![deny(warnings)]

extern crate atomicwrites;
extern crate docopt;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate strsim;

pub mod cli;
pub mod config;
pub mod data;
pub mod matcher;
mod utils;

pub use self::cli::*;
pub use self::config::*;


/// The version of original `autojump` this library is compatible with.
pub const VERSION_TRACK: &'static str = "22.5.0";

/// The library's version.
pub const VERSION: &'static str = "0.2.2";
