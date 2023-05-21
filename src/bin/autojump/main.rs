use clap::{crate_version, value_parser, Arg, ArgAction, Command};

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
        let app = Command::new("autojump-rs")
            .version(crate_version!())
            .about("Automatically jump to directory passed as an argument.")
            .arg(Arg::new("dir").action(ArgAction::Append))
            .arg(
                Arg::new("add")
                    .short('a')
                    .long("add")
                    .value_name("DIR")
                    .action(ArgAction::Set)
                    .help("add path"),
            )
            .arg(
                Arg::new("complete")
                    .long("complete")
                    .action(ArgAction::SetTrue)
                    .help("used for tab completion"),
            )
            .arg(
                Arg::new("purge")
                    .long("purge")
                    .action(ArgAction::SetTrue)
                    .help("remove non-existent paths from database"),
            )
            .arg(
                Arg::new("stat")
                    .short('s')
                    .long("stat")
                    .action(ArgAction::SetTrue)
                    .help("show database entries and their key weights"),
            )
            .arg(
                Arg::new("increase")
                    .short('i')
                    .long("increase")
                    .value_parser(value_parser!(isize))
                    .action(ArgAction::Set)
                    .value_name("WEIGHT")
                    .num_args(0..=1)
                    .help("increase current directory weight, default 10"),
            )
            .arg(
                Arg::new("decrease")
                    .short('d')
                    .long("decrease")
                    .value_parser(value_parser!(isize))
                    .action(ArgAction::Set)
                    .value_name("WEIGHT")
                    .num_args(0..=1)
                    .help("decrease current directory weight, default 15"),
            )
            .get_matches();

        let flag_increase = if app.contains_id("increase") {
            Some(app.get_one::<isize>("increase").copied())
        } else {
            None
        };

        let flag_decrease = if app.contains_id("decrease") {
            Some(app.get_one::<isize>("decrease").copied())
        } else {
            None
        };

        Args {
            arg_dir: app
                .get_many::<String>("dir")
                .map_or(vec![], |x| x.cloned().collect()),
            flag_complete: app.get_flag("complete"),
            flag_purge: app.get_flag("purge"),
            flag_add: app.get_one::<String>("add").cloned(),
            flag_increase,
            flag_decrease,
            flag_stat: app.get_flag("stat"),
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
