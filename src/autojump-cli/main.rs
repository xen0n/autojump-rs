extern crate rustc_serialize;
extern crate docopt;

extern crate autojump;
extern crate autojump_data;
extern crate autojump_match;
extern crate autojump_utils;

mod manip;
mod purge;
mod query;
mod stat;


const VERSION_TRACK: &'static str = "22.5.0";
const VERSION: &'static str = "0.1.0";


const USAGE: &'static str = "
Automatically jump to directory passed as an argument.

Usage:
  autojump [<dir>...]
  autojump --complete [<dir>...]
  autojump --purge
  autojump (-a <dir> | --add <dir>)
  autojump (-i | --increase) [<weight>]
  autojump (-d | --decrease) [<weight>]
  autojump (-s | --stat)
  autojump (-h | --help)
  autojump (-v | --version)


Positional arguments:
  DIR                          directory to jump to
  WEIGHT                       weight to increase/decrease for current dir

Optional arguments:
  -h, --help                   show this help message and exit
  -a DIR, --add DIR            add path
  -i, --increase               increase current directory weight, default 10
  -d, --decrease               decrease current directory weight, default 15
  --complete                   used for tab completion
  --purge                      remove non-existent paths from database
  -s, --stat                   show database entries and their key weights
  -v, --version                show version information

Please see autojump(1) man pages for full documentation.
";


#[derive(RustcDecodable)]
struct Args {
    arg_dir: Vec<String>,
    arg_weight: Option<isize>,
    flag_complete: bool,
    flag_purge: bool,
    flag_add: Option<String>,
    flag_increase: bool,
    flag_decrease: bool,
    flag_stat: bool,
    flag_version: bool,
}


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

    let args: Args = docopt::Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());
    let config = autojump::Config::defaults();

    // Process arguments.
    // All arguments are mutually exclusive, so we just check for presence
    // one-by-one.
    if args.flag_complete {
        query::complete(&config, args.arg_dir);
        return;
    }
    if args.flag_add.is_some() {
        manip::add(&config, args.flag_add.unwrap());
        return;
    }
    if args.flag_increase {
        manip::increase(&config, args.arg_weight);
        return;
    }
    if args.flag_decrease {
        manip::decrease(&config, args.arg_weight);
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

    query::query(&config, args.arg_dir);
}
