use std::env;
use std::path;

use super::super::data;
use super::super::matcher::Matcher;
use super::super::utils;
use super::super::utils::TabEntryInfo;
use super::super::Config;

struct QueryConfig<'a> {
    needles: Vec<&'a str>,
    check_existence: bool,
    index: usize,
    count: usize,
    use_fallback: bool,
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

    match prepare_query(&needles, false, 9, false) {
        Query::Execute(query) => {
            let real_needle = query.needles[0].clone();
            let result = do_query(config, query);
            // Convert to `&str` for tab entry info creation.
            let result: Vec<_> = result
                .into_iter()
                .map(|p| p.to_string_lossy().into_owned())
                .collect();
            let strs: Vec<_> = result.iter().map(|p| p.as_str()).collect();
            // Directly print out the directory if it's the only entry.
            if strs.len() == 1 {
                println!("{}", strs[0]);
                return;
            }
            // Output the tab completion menu
            let tab_entries = TabEntryInfo::from_matches(real_needle, &strs);
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
    let result = match prepare_query(&needles, true, 1, true) {
        Query::Execute(query) => do_query(config, query).iter().next().unwrap().clone(),
        Query::EarlyResult(path) => path,
    };
    println!("{}", result.to_string_lossy());
}

fn prepare_query<'a>(
    needles: &'a [&'a str],
    check_existence: bool,
    count: usize,
    use_fallback: bool,
) -> Query<'a> {
    let mut count = count;
    let needles = if needles.is_empty() {
        vec![""]
    } else {
        utils::sanitize(needles)
    };

    // Try to parse the first needle (command-line argument) as tab entry
    // spec.
    let tab = utils::get_tab_entry_info(needles[0]);
    if tab.path.is_some() {
        // Just trust the auto-completion, like the original impl does.
        let result = path::Path::new(tab.path.unwrap()).to_path_buf();
        return Query::EarlyResult(result);
    }

    // Override query needles if tab entry is found, also set the index
    // requested.
    let index;
    let needles = if tab.index.is_some() {
        // process "foo__" and "foo__1" differently
        if tab.index_explicit {
            // explicit match requested, override count
            count = 1;
        }

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
        use_fallback: use_fallback,
    })
}

fn do_query<'a>(config: &Config, query: QueryConfig<'a>) -> Vec<path::PathBuf> {
    let needles = query.needles;
    let check_existence = query.check_existence;
    let index = query.index;
    let count = query.count;

    let mut entries = data::load(config);
    // Default order is ascending, but apparently we want to match the
    // other way around.
    entries.sort_by(|a, b| b.cmp(a));

    let matcher = Matcher::new_smartcase(needles);
    let result = matcher.execute(&entries);

    // Filter out cwd and (when requested) non-existent directories.
    let cwd: Option<_> = match env::current_dir() {
        Ok(cwd) => Some(cwd),
        Err(_) => None,
    };
    let mut result: Vec<_> = result
        .filter(|p| {
            if let Some(cwd) = &cwd {
                &p.path != cwd
            } else {
                true
            }
        })
        .filter(|p| {
            if check_existence {
                p.path.exists()
            } else {
                true
            }
        })
        .skip(index)
        .take(count)
        .map(|p| p.path.clone())
        .collect();
    if query.use_fallback {
        result.push(".".into())
    }

    result
}
