use clap::{crate_version, App, Arg};

use autojump::Config;

mod manip;
mod purge;
mod query;
mod stat;
mod utils;

struct Args {
    arg_dir: Vec<String>,
    flag_complete: bool,
    flag_purge: bool,
    flag_add: Option<String>,
    flag_increase: Option<Option<isize>>,
    flag_decrease: Option<Option<isize>>,
    flag_stat: bool,
}

#[cfg(not(windows))]
fn check_if_sourced() {
    if !utils::is_autojump_sourced() {
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

pub fn main() {
    check_if_sourced();

    let args: Args = {
        let app = App::new("autojump-rs")
            .version(crate_version!())
            .about("Automatically jump to directory passed as an argument.")
            .arg(Arg::new("dir").multiple_occurrences(true))
            .arg(
                Arg::new("add")
                    .short('a')
                    .long("add")
                    .takes_value(true)
                    .value_name("DIR")
                    .help("add path"),
            )
            .arg(
                Arg::new("complete")
                    .long("complete")
                    .help("used for tab completion"),
            )
            .arg(
                Arg::new("purge")
                    .long("purge")
                    .help("remove non-existent paths from database"),
            )
            .arg(
                Arg::new("stat")
                    .short('s')
                    .long("stat")
                    .help("show database entries and their key weights"),
            )
            .arg(
                Arg::new("increase")
                    .short('i')
                    .long("increase")
                    .takes_value(true)
                    .value_name("WEIGHT")
                    .min_values(0)
                    .help("increase current directory weight, default 10"),
            )
            .arg(
                Arg::new("decrease")
                    .short('d')
                    .long("decrease")
                    .takes_value(true)
                    .value_name("WEIGHT")
                    .min_values(0)
                    .help("decrease current directory weight, default 15"),
            )
            .get_matches();

        let flag_increase = if app.is_present("increase") {
            Some(app.value_of("increase").map(|x| x.parse().unwrap()))
        } else {
            None
        };

        let flag_decrease = if app.is_present("decrease") {
            Some(app.value_of("decrease").map(|x| x.parse().unwrap()))
        } else {
            None
        };

        Args {
            arg_dir: app
                .values_of("dir")
                .map(|x| x.map(|i| i.to_owned()).collect())
                .unwrap_or(vec![]),
            flag_complete: app.is_present("complete"),
            flag_purge: app.is_present("purge"),
            flag_add: app.value_of("add").map(|x| x.to_owned()),
            flag_increase,
            flag_decrease,
            flag_stat: app.is_present("stat"),
        }
    };
    let config = Config::defaults();

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
    if let Some(weight) = args.flag_increase {
        manip::increase(&config, weight);
        return;
    }
    if let Some(weight) = args.flag_decrease {
        manip::decrease(&config, weight);
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

    query::query(&config, args.arg_dir);
}
