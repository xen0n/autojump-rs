use std::env;
use std::path;

use autojump::Config;
use autojump_data;
use autojump_match::Matcher;
use autojump_utils;


pub fn query(config: &Config, needles: Vec<String>) {
    let needles = if needles.is_empty() {
        vec!["".to_owned()]
    } else {
        needles
    };

    let result = do_query(config, needles, true);
    println!("{}", result.to_string_lossy());
}


fn do_query(config: &Config,
            needles: Vec<String>,
            check_existence: bool) -> path::PathBuf {
    let needles = autojump_utils::sanitize(&needles);

    // Try to parse the first needle (command-line argument) as tab entry
    // spec.
    let tab = autojump_utils::get_tab_entry_info(needles[0]);
    if tab.path.is_some() {
        // Just trust the auto-completion, like the original impl does.
        return path::Path::new(tab.path.unwrap()).to_path_buf();
    }

    // Override query needles if tab entry is found, also set the index
    // requested.
    let index;
    let needles = if tab.index.is_some() {
        index = tab.index.unwrap();
        vec![tab.needle.unwrap()]
    } else {
        index = 0;
        needles
    };

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

    if result.len() < index + 1 {
        // Index is out-of-bounds, return something for shell.
        path::Path::new(".").to_path_buf()
    } else {
        result[index].to_path_buf()
    }
}
