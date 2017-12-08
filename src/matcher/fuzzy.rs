use std::iter;
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


    #[cfg(feature = "nightly")]
    pub fn filter_path<'p, P>(&'a self, paths: &'p [P]) -> impl iter::Iterator<Item = &'p P> + 'a
        where P: AsRef<path::Path>,
              'p: 'a
    {
        paths.iter()
            .map(|p| (p.as_ref().file_name(), p))
            .filter(|&(s, _)| s.is_some())
            .map(|(s, p)| (s.unwrap().to_string_lossy().into_owned(), p))
            .map(move |(s, p)| (strsim::jaro_winkler(self.needle, &s), p))
            .filter(move |&(sim, _)| sim >= self.threshold)
            .map(|(_, p)| p)
    }


    #[cfg(not(feature = "nightly"))]
    pub fn filter_path<'p, P>(&'a self, paths: &'p [P]) -> Box<iter::Iterator<Item = &'p P> + 'a>
        where P: AsRef<path::Path>,
              'p: 'a
    {
        Box::new(paths.iter()
            .map(|p| (p.as_ref().file_name(), p))
            .filter(|&(s, _)| s.is_some())
            .map(|(s, p)| (s.unwrap().to_string_lossy().into_owned(), p))
            .map(move |(s, p)| (strsim::jaro_winkler(self.needle, &s), p))
            .filter(move |&(sim, _)| sim >= self.threshold)
            .map(|(_, p)| p))
    }
}
