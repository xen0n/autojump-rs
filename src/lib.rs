#![deny(warnings)]

pub mod cli;
pub mod config;
pub mod data;
pub mod matcher;
mod utils;

pub use self::cli::*;
pub use self::config::*;


/// The version of original `autojump` this library is compatible with.
pub const VERSION_TRACK: &str = "22.5.0";

/// The library's version.
pub const VERSION: &str = "0.3.1";
