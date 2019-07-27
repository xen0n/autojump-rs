use std::path;


pub struct Config {
    pub prefix: path::PathBuf,
    pub data_path: path::PathBuf,
    pub backup_path: path::PathBuf,
}


#[cfg(unix)]
fn home_dir() -> path::PathBuf {
    match dirs::home_dir() {
        Some(p) => p,
        // be consistent with Python's `os.path.expand_user('~')`
        None => path::PathBuf::from("/"),
    }
}


#[cfg(unix)]
pub fn xdg_home_hardcoded() -> path::PathBuf {
    // ~/.local/share/autojump
    let mut tmp = home_dir();
    tmp.push(".local");
    tmp.push("share");
    tmp.push("autojump");
    tmp
}


// TODO: is this cfg appropriate for *all* Unix platforms, especially BSD?
#[cfg(all(unix, not(target_os = "macos")))]
fn data_home() -> path::PathBuf {
    use std::env;
    // Use $XDG_DATA_HOME if defined, ~/.local/share/autojump otherwise.
    if let Some(home_s) = env::var_os("XDG_DATA_HOME") {
        path::PathBuf::from(home_s)
    } else {
        xdg_home_hardcoded()
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
    use std::env;
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
        Config::from_prefix(&data_home)
    }

    pub fn from_prefix(data_home: &path::Path) -> Config {
        let data_home = data_home.to_path_buf();
        let data_path = data_home.join("autojump.txt");
        let backup_path = data_home.join("autojump.txt.bak");

        Config {
            prefix: data_home,
            data_path: data_path,
            backup_path: backup_path,
        }
    }
}
