use std::env;
use std::path;


pub struct Config {
    data_path: path::PathBuf,
    backup_path: path::PathBuf,
}


#[cfg(unix)]
fn home_dir() -> path::PathBuf {
    match env::home_dir() {
        Some(p) => p,
        // be consistent with Python's `os.path.expand_user('~')`
        None => path::PathBuf::from("/"),
    }
}


#[cfg(target_os = "linux")]
fn data_home() -> path::PathBuf {
    // Use $XDG_DATA_HOME if defined, ~/.local/share/autojump otherwise.
    if let Some(home_s) = env::var_os("XDG_DATA_HOME") {
        path::PathBuf::from(home_s)
    } else {
        let mut tmp = home_dir();
        tmp.push(".local");
        tmp.push("share");
        tmp.push("autojump");
        tmp
    }
}


#[cfg(target_os = "macos")]
fn data_home() -> path::PathBuf {
    let mut tmp = home_dir();
    tmp.push("Library");
    tmp.push("autojump");
    tmp
}


#[cfg(windows)]
fn data_home() -> path::PathBuf {
    // `%APPDATA%` is always present on Windows, unless someone actually
    // decided to remove it in Control Panel. We wouldn't want to support
    // those people indeed...
    let mut tmp = path::PathBuf::from(env::var_os("APPDATA").unwrap());
    tmp.push("autojump");
    tmp
}


impl Config {
    pub fn defaults() -> Config {
        let data_home = data_home();
        let data_path_join = |s| {
            let mut tmp = data_home.clone();
            tmp.push(s);
            tmp
        };
        let data_path = data_path_join("autojump.txt");
        let backup_path = data_path_join("autojump.txt.bak");

        Config {
            data_path: data_path,
            backup_path: backup_path,
        }
    }
}
