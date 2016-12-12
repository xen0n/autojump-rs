use std::iter;
use std::path;

use regex;

use super::fuzzy;
use super::re_based;


pub struct Matcher<'a> {
    fuzzy_matcher: fuzzy::FuzzyMatcher<'a>,
    re_anywhere: regex::Regex,
    re_consecutive: regex::Regex,
}


/// Returns whether matches should ignore case based on uppercase letter's
/// presence in the needles.
fn detect_smartcase(needles: &[&str]) -> bool {
    for s in needles {
        for ch in s.chars() {
            if ch.is_uppercase() {
                return false;
            }
        }
    }

    true
}


impl<'a> Matcher<'a> {
    pub fn new_smartcase(needles: Vec<&'a str>) -> Matcher<'a> {
        let ignore_case = detect_smartcase(&needles);
        Matcher::new(needles, ignore_case)
    }

    pub fn new(needles: Vec<&'a str>, ignore_case: bool) -> Matcher<'a> {
        let fuzzy_matcher = fuzzy::FuzzyMatcher::defaults(needles[needles.len() - 1]);
        let re_anywhere = re_based::prepare_regex(
            &needles,
            re_based::re_match_anywhere,
            ignore_case,
            );
        let re_consecutive = re_based::prepare_regex(
            &needles,
            re_based::re_match_consecutive,
            ignore_case,
            );

        Matcher {
            fuzzy_matcher: fuzzy_matcher,
            re_anywhere: re_anywhere,
            re_consecutive: re_consecutive,
        }
    }

    pub fn execute<'p, P>(&'a self, haystack: &'p [P]) -> impl iter::Iterator<Item=&'p P> + 'a
            where P: AsRef<path::Path>, 'p: 'a {
        // Iterator sadness...
        macro_rules! filter_path_with_re {
            ($l: expr, $re: expr) => {
                $l
                    .iter()
                    .filter(move |&p| $re.is_match(p.as_ref().to_string_lossy().to_mut()))
            };
        }


        filter_path_with_re!(haystack, self.re_consecutive)
            .chain(self.fuzzy_matcher.filter_path(haystack))
            .chain(filter_path_with_re!(haystack, self.re_anywhere))
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_smartcase() {
        macro_rules! a {
            ($needles: tt, $y: expr) => {
                assert_eq!(detect_smartcase(&vec! $needles), $y);
            };
        }

        a!([], true);
        a!([""], true);
        a!(["foo"], true);
        a!(["foo", "bar"], true);
        a!(["测试", "bar"], true);
        a!(["foo", "bar", "测试", "baZ"], false);
    }
}
