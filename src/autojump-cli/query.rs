use std::env;
use std::path;

use autojump::Config;
use autojump_data;
use autojump_match::Matcher;
use autojump_utils;
use autojump_utils::TabEntryInfo;


struct QueryConfig<'a> {
    needles: Vec<&'a str>,
    check_existence: bool,
    index: usize,
    count: usize,
}


enum Query<'a> {
    Execute(QueryConfig<'a>),
    EarlyResult(path::PathBuf),
}


pub fn complete(config: &Config, needles: Vec<String>) {
    // Override needles to only consider the first entry (if present).
    let needle = if needles.is_empty() {
        ""
    } else {
        needles[0].as_str()
    };
    let needles = vec![needle];

    match prepare_query(&needles, false, 9) {
        Query::Execute(query) => {
            let result = do_query(config, query);
            // Convert to `&str` for tab entry info creation.
            let result: Vec<_> = result
                .into_iter()
                .map(|p| p.to_string_lossy().into_owned())
                .collect();
            let strs: Vec<_> = result.iter().map(|p| p.as_str()).collect();
            let tab_entries = TabEntryInfo::from_matches(needle, &strs);
            // Output the tab completion menu.
            for tab_entry in tab_entries.into_iter() {
                println!("{}", tab_entry);
            }
        }
        Query::EarlyResult(path) => {
            println!("{}", path.to_string_lossy());
        }
    }
}


pub fn query(config: &Config, needles: Vec<String>) {
    let needles: Vec<_> = needles.iter().map(|s| s.as_str()).collect();
    let result = match prepare_query(&needles, true, 1) {
        Query::Execute(query) => do_query(config, query).pop().unwrap(),
        Query::EarlyResult(path) => path,
    };
    println!("{}", result.to_string_lossy());
}


fn prepare_query<'a>(needles: &'a [&'a str],
                     check_existence: bool,
                     count: usize) -> Query<'a> {
    let needles = if needles.is_empty() {
        vec![""]
    } else {
        autojump_utils::sanitize(needles)
    };

    // Try to parse the first needle (command-line argument) as tab entry
    // spec.
    let tab = autojump_utils::get_tab_entry_info(needles[0]);
    if tab.path.is_some() {
        // Just trust the auto-completion, like the original impl does.
        let result = path::Path::new(tab.path.unwrap()).to_path_buf();
        return Query::EarlyResult(result);
    }

    // Override query needles if tab entry is found, also set the index
    // requested.
    let index;
    let needles = if tab.index.is_some() {
        // index is 1-based on the command line!
        index = tab.index.unwrap() - 1;
        vec![tab.needle.unwrap()]
    } else {
        index = 0;
        needles
    };

    Query::Execute(QueryConfig {
        needles: needles,
        check_existence: check_existence,
        index: index,
        count: count,
    })
}


fn do_query<'a>(config: &Config, query: QueryConfig<'a>) -> Vec<path::PathBuf> {
    let needles = query.needles;
    let check_existence = query.check_existence;
    let index = query.index;
    let count = query.count;

    let entries = {
        let mut tmp = autojump_data::load(config);
        // Default order is ascending, but apparently we want to match the
        // other way around.
        tmp.sort_by(|a, b| b.cmp(a));
        tmp
    };
    let matcher = Matcher::new_smartcase(needles);
    let result = matcher.execute(&entries);

    // Filter out cwd and (when requested) non-existent directories.
    let cwd: Option<_> = match env::current_dir() {
        Ok(cwd) => Some(cwd),
        Err(_) => None,
    };
    let result: Vec<_> = result
        .into_iter()
        .filter(|p| {
            if cwd.is_some() {
                p != cwd.as_ref().unwrap()
            } else {
                true
            }
        })
        .filter(|p| {
            if check_existence {
                p.exists()
            } else {
                true
            }
        })
        .collect();

    if count == 1 {
        if result.len() < index + 1 {
            // Index is out-of-bounds, return something for shell.
            return vec![path::Path::new(".").to_path_buf()];
        }
    }

    result.iter().skip(index).take(count).map(|p| p.to_path_buf()).collect()
}
