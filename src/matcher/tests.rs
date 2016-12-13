use std::path;

use super::*;
use super::fuzzy::*;
use super::re_based::*;


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


macro_rules! assert_re {
    ($fn_name: ident, $x: tt, $y: expr) => {
        assert_eq!($fn_name(&vec! $x), $y);
    };
}


#[cfg(unix)]
#[test]
fn test_re_match_anywhere() {
    macro_rules! a {
        ($x: tt, $y: expr) => {
            assert_re!(re_match_anywhere, $x, $y);
        };
    }

    a!(["foo"], r".*foo.*");
    a!(["foo", "baz"], r".*foo.*baz.*");
    a!(["测试", "baz"], r".*\x{6d4b}\x{8bd5}.*baz.*");
}


#[cfg(unix)]
#[test]
fn test_re_match_consecutive() {
    macro_rules! a {
        ($x: tt, $y: expr) => {
            assert_re!(re_match_consecutive, $x, $y);
        };
    }

    a!(["foo"], r"foo[^/]*$");
    a!(["foo", "baz"], r"foo[^/]*/[^/]*baz[^/]*$");
    a!(["测试", "baz"], r"\x{6d4b}\x{8bd5}[^/]*/[^/]*baz[^/]*$");
}


#[cfg(unix)]
#[test]
fn test_fuzzy() {
    macro_rules! a {
        ($needle: expr, [$($x: expr, )*], [$($y: expr, )*]) => {
            let matcher = FuzzyMatcher::defaults($needle);
            let haystack: Vec<&path::Path> = vec![$(path::Path::new($x), )*];
            let expected: Vec<&path::Path> = vec![$(path::Path::new($y), )*];
            let actual: Vec<_> = matcher.filter_path(&haystack).collect();
            assert_eq!(expected.len(), actual.len());
            for (i, j) in expected.into_iter().zip(actual.into_iter()) {
                assert_eq!(&i, j);
            }
        };
    }

    a!("foo", [], []);
    a!(
        "foo",
        [
            "/fow/bar",
            "/bar/foo",
            "/bar/fooow",
            "/fuzzy",
            "/moo/foo/baz",
            "/foo/ooofoo",
        ],
        [
            "/bar/foo",
            "/bar/fooow",
            "/foo/ooofoo",
        ]);
}


#[test]
fn test_matcher() {
    let needles = vec!["foo", "baz"];
    let matcher = Matcher::new(needles, false);

    let haystack = vec![
        path::Path::new("/foo/bar/baz"),
        path::Path::new("/moo/foo/baz"),
        path::Path::new("/baz/foo/bar"),
        path::Path::new("/moo/baz/foo"),
        path::Path::new("/foo/baz"),
    ];

    let actual: Vec<_> = matcher.execute(&haystack).collect();
    let expected = vec![
        // consecutive matcher
        path::Path::new("/moo/foo/baz"),
        path::Path::new("/foo/baz"),
        // fuzzy matcher
        path::Path::new("/foo/bar/baz"),
        path::Path::new("/moo/foo/baz"),
        path::Path::new("/baz/foo/bar"),
        path::Path::new("/foo/baz"),
        // anywhere matcher
        path::Path::new("/foo/bar/baz"),
        path::Path::new("/moo/foo/baz"),
        path::Path::new("/foo/baz"),
    ];
    assert_eq!(actual.len(), expected.len());
    for (i, j) in expected.into_iter().zip(actual.into_iter()) {
        assert_eq!(&i, j);
    }
}
