#![feature(conservative_impl_trait)]

extern crate atomicwrites;
extern crate docopt;
extern crate regex;
extern crate rustc_serialize;
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
pub const VERSION: &'static str = "0.1.0";


/// Get a version string suitable for the CLI display.
pub fn get_version_str(
        vcs_commit: Option<&str>,
        vcs_clean: Option<bool>
        ) -> String {
    let mut tmp = String::new();
    tmp.push_str("autojump v");
    tmp.push_str(VERSION_TRACK);
    tmp.push_str("\nautojump-rs v");
    tmp.push_str(VERSION);

    // add version control status if available
    if let Some(commit) = vcs_commit {
        tmp.push_str(" (");
        tmp.push_str(commit);

        if let Some(clean) = vcs_clean {
            if !clean {
                tmp.push_str("-dirty");
            }
        }

        tmp.push(')');
    }

    tmp
}

