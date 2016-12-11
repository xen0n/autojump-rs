use std::path;

use regex;

use super::re_based;


pub struct Matcher {
    // needles: Vec<&'a str>,
    re_anywhere: regex::Regex,
    re_consecutive: regex::Regex,
}


fn filter_path<'p, P, F>(l: &'p [P], f: F) -> Vec<&'p path::Path>
        where P: AsRef<path::Path>, F: Fn(&&path::Path) -> bool {
    l.iter().map(|p| p.as_ref()).filter(f).collect()
}


fn filter_path_with_re<'p, P>(l: &'p [P], re: &regex::Regex) -> Vec<&'p path::Path>
        where P: AsRef<path::Path> {
    filter_path(l, |p| re.is_match(p.to_string_lossy().to_mut()))
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


impl Matcher {
    pub fn new_smartcase(needles: Vec<&str>) -> Matcher {
        let ignore_case = detect_smartcase(&needles);
        Matcher::new(needles, ignore_case)
    }

    pub fn new(needles: Vec<&str>, ignore_case: bool) -> Matcher {
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
            // needles: needles,
            re_anywhere: re_anywhere,
            re_consecutive: re_consecutive,
        }
    }

    pub fn execute<'p, P>(&self, haystack: &'p [P]) -> Vec<&'p path::Path>
            where P: AsRef<path::Path> {
        let mut result = vec![];
        result.extend(filter_path_with_re(haystack, &self.re_consecutive));
        // TODO: fuzzy matcher
        result.extend(filter_path_with_re(haystack, &self.re_anywhere));
        result
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


    #[test]
    fn test_match_anywhere() {
        let needles = vec!["foo", "baz"];
        let re = re_based::prepare_regex(
            &needles,
            re_based::re_match_anywhere,
            false,
            );

        let haystack = vec![
            path::Path::new("/foo/bar/baz"),
            path::Path::new("/baz/foo/bar"),
            path::Path::new("/foo/baz"),
        ];

        assert_eq!(
            filter_path_with_re(&haystack, &re),
            [
                path::Path::new("/foo/bar/baz"),
                path::Path::new("/foo/baz"),
            ]);
    }


    #[test]
    fn test_match_consecutive() {
        let needles = vec!["foo", "baz"];
        let re = re_based::prepare_regex(
            &needles,
            re_based::re_match_consecutive,
            false,
            );

        let haystack = vec![
            path::Path::new("/foo/bar/baz"),
            path::Path::new("/foo/baz/moo"),
            path::Path::new("/moo/foo/baz"),
            path::Path::new("/foo/baz"),
        ];

        assert_eq!(
            filter_path_with_re(&haystack, &re),
            [
                path::Path::new("/moo/foo/baz"),
                path::Path::new("/foo/baz"),
            ]);
    }


}
