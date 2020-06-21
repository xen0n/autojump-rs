use std::cmp;
use std::fmt;
use std::path;

#[derive(Clone, Debug)]
pub struct Entry {
    pub path: path::PathBuf,
    pub weight: f64,
}

impl Entry {
    pub fn new<P>(path: P, weight: f64) -> Entry
    where
        P: Into<path::PathBuf>,
    {
        Entry {
            path: path.into(),
            weight: weight,
        }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.1}:\t{}", self.weight, self.path.to_string_lossy())
    }
}

impl AsRef<path::Path> for Entry {
    fn as_ref(&self) -> &path::Path {
        self.path.as_path()
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Entry) -> Option<cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Entry) -> bool {
        self.weight == other.weight
    }
}

impl Eq for Entry {}

impl Ord for Entry {
    fn cmp(&self, other: &Entry) -> cmp::Ordering {
        // We know that NaN's don't exist in our use case, so just unwrap it.
        self.partial_cmp(other).unwrap()
    }
}
