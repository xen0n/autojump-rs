#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate rustc_serialize;
extern crate docopt;

extern crate autojump;
extern crate autojump_data;
extern crate autojump_utils;

mod manip;
mod purge;
mod stat;


const VERSION_TRACK: &'static str = "22.5.0";
const VERSION: &'static str = "0.1";


docopt!(Args derive Debug, "
Automatically jump to directory passed as an argument.

Usage:
  autojump [<dir>...]
  autojump --complete
  autojump --purge
  autojump (-a <dir> | --add <dir>)
  autojump (-i <weight> | --increase <weight>)
  autojump (-d <weight> | --decrease <weight>)
  autojump (-s | --stat)
  autojump (-h | --help)
  autojump (-v | --version)


positional arguments:
  DIR                          directory to jump to

optional arguments:
  -h, --help                   show this help message and exit
  -a DIR, --add DIR            add path
  -i WEIGHT, --increase WEIGHT
                               increase current directory weight
  -d WEIGHT, --decrease WEIGHT
                               decrease current directory weight
  --complete                   used for tab completion
  --purge                      remove non-existent paths from database
  -s, --stat                   show database entries and their key weights
  -v, --version                show version information

Please see autojump(1) man pages for full documentation.
",
    arg_dir: Vec<String>,
    flag_complete: bool,
    flag_purge: bool,
    flag_add: Option<String>,
    flag_increase: Option<isize>,
    flag_decrease: Option<isize>,
    flag_stat: bool,
    flag_version: bool,
);


#[cfg(not(windows))]
fn check_if_sourced() {
    if !autojump_utils::is_autojump_sourced() {
        println!("Please source the correct autojump file in your shell's");
        println!("startup file. For more information, please reinstall autojump");
        println!("and read the post installation instructions.");
        std::process::exit(1);
    }
}


#[cfg(windows)]
fn check_if_sourced() {
    // no-op on Windows
}


fn main() {
    check_if_sourced();

    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    let config = autojump::Config::defaults();

    // Process arguments.
    // All arguments are mutually exclusive, so we just check for presence
    // one-by-one.
    if args.flag_add.is_some() {
        manip::add(&config, args.flag_add.unwrap());
        return;
    }
    if args.flag_increase.is_some() {
        manip::increase(&config, args.flag_increase.unwrap() as f64);
        return;
    }
    if args.flag_decrease.is_some() {
        manip::decrease(&config, args.flag_decrease.unwrap() as f64);
        return;
    }
    if args.flag_purge {
        purge::purge(&config);
        return;
    }
    if args.flag_stat {
        stat::print_stat(&config);
        return;
    }
    if args.flag_version {
        println!("autojump v{} (autojump-rs v{})", VERSION_TRACK, VERSION);
        return;
    }

    // TODO
    println!("{:?}", args);
    let entries = autojump_data::load(&config);
    println!("entries = {:?}", entries);
}
