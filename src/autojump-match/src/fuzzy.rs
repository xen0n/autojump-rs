use std::path;

use strsim;


/// A fuzzy matcher based on Jaro-Winkler distances.
///
/// The similarity is calculated between the last component of the path and
/// the last part of the needle.
pub struct FuzzyMatcher<'a> {
    needle: &'a str,
    threshold: f64,
}


const DEFAULT_FUZZY_THRESHOLD: f64 = 0.6;


impl<'a> FuzzyMatcher<'a> {
    pub fn defaults(needle: &'a str) -> FuzzyMatcher<'a> {
        FuzzyMatcher::new(needle, DEFAULT_FUZZY_THRESHOLD)
    }


    pub fn new(needle: &'a str, threshold: f64) -> FuzzyMatcher<'a> {
        FuzzyMatcher {
            needle: needle,
            threshold: threshold,
        }
    }


    pub fn filter_path<'p, P>(&self, paths: &'p [P]) -> Vec<&'p path::Path>
            where P: AsRef<path::Path> {
        paths
            .iter()
            .map(|p| p.as_ref())
            .map(|p| (p.file_name(), p))
            .filter(|&(s, _)| s.is_some())
            .map(|(s, p)| (s.unwrap().to_string_lossy().into_owned(), p))
            .map(|(s, p)| (strsim::jaro(self.needle, &s), p))
            .filter(|&(sim, _)| sim >= self.threshold)
            .map(|(_, p)| p)
            .collect()
    }
}
