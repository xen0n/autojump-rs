use std::fs;
use std::io;
use std::io::BufRead;
use std::path;

use autojump::Config;
use super::entry::Entry;


const BACKUP_THRESHOLD: usize = 24 * 60 * 60;  // 1 d


#[cfg(target_os = "macos")]
fn migrate_osx_xdg_data(config: &Config) -> io::Result<()> {
    let xdg_aj_home = autojump::xdg_home_hardcoded();
    if !xdg_aj_home.exists() {
        Ok(())
    }

    let old_config = Config::from_prefix(&xdg_aj_home);

    fs::copy(old_config.data_path, config.data_path)?;
    fs::copy(old_config.backup_path, config.backup_path)?;

    fs::remove_file(old_config.data_path)?;
    fs::remove_file(old_config.backup_path)?;
}


#[cfg(not(target_os = "macos"))]
fn migrate_osx_xdg_data(_: &Config) -> io::Result<()> {
    Ok(())
}


fn load_line(line: &str) -> Option<Entry> {
    let parts: Vec<_> = line.splitn(2, '\t').collect();
    if parts.len() != 2 {
        return None;
    }

    let path = path::PathBuf::from(parts[1]);
    let weight = parts[0].parse::<f64>();
    if let Ok(weight) = weight {
        Some(Entry::new(path, weight))
    } else {
        None
    }
}


fn load_from_file(f: fs::File) -> io::Result<Vec<Entry>> {
    let reader = io::BufReader::new(f);
    let mut result = vec![];

    for line in reader.lines() {
        if let Ok(line) = line {
            let entry = load_line(&line);
            if entry.is_some() {
                result.push(entry.unwrap());
            }
        } else {
            return Err(line.unwrap_err());
        }
    }

    Ok(result)
}


pub fn load(config: &Config) -> Vec<Entry> {
    // Only necessary when running on macOS, no-op on others
    migrate_osx_xdg_data(config).unwrap();

    if !config.data_path.exists() {
        return vec![];
    }

    let result = load_from_file(fs::File::open(&config.data_path).unwrap());
    if let Ok(result) = result {
        result
    } else {
        load_backup(config)
    }
}


fn load_backup(config: &Config) -> Vec<Entry> {
    if config.backup_path.exists() {
        fs::rename(&config.backup_path, &config.data_path).unwrap();
        load_from_file(fs::File::open(&config.data_path).unwrap()).unwrap()
    } else {
        vec![]
    }
}
