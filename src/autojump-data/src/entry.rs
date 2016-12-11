use std::path;


#[derive(Debug)]
pub struct Entry {
    pub path: path::PathBuf,
    pub weight: f64,
}


impl Entry {
    pub fn new<P>(path: P, weight: f64) -> Entry where P: Into<path::PathBuf> {
        Entry {
            path: path.into(),
            weight: weight,
        }
    }
}
