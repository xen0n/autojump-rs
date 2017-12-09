use std::path;


fn sanitize_one_needle(needle: &str) -> &str {
    if needle.len() == 1 {
        if needle.chars().next().unwrap() == path::MAIN_SEPARATOR {
            return needle;
        }
    }

    needle.trim_right_matches(path::MAIN_SEPARATOR)
}


pub fn sanitize<'a, S>(needles: &'a [S]) -> Vec<&'a str>
where
    S: AsRef<str>,
{
    needles
        .iter()
        .map(|s| s.as_ref())
        .map(sanitize_one_needle)
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;


    #[cfg(not(windows))]
    #[test]
    fn test_sanitize_one_needle() {
        assert_eq!(sanitize_one_needle(""), "");
        assert_eq!(sanitize_one_needle("/"), "/");
        assert_eq!(sanitize_one_needle("/a"), "/a");
        assert_eq!(sanitize_one_needle("a"), "a");
        assert_eq!(sanitize_one_needle("a/"), "a");
        assert_eq!(sanitize_one_needle("a//"), "a");
    }


    #[cfg(windows)]
    #[test]
    fn test_sanitize_one_needle() {
        assert_eq!(sanitize_one_needle(""), "");
        assert_eq!(sanitize_one_needle("\\"), "\\");
        assert_eq!(sanitize_one_needle("\\a"), "\\a");
        assert_eq!(sanitize_one_needle("a"), "a");
        assert_eq!(sanitize_one_needle("a\\"), "a");
        assert_eq!(sanitize_one_needle("a\\\\"), "a");
    }


    #[cfg(not(windows))]
    #[test]
    fn test_sanitize() {
        let a: Vec<&str> = vec![];
        let b: Vec<&str> = vec![];
        assert_eq!(sanitize(&a), b);

        assert_eq!(sanitize(&[""]), [""]);
        assert_eq!(sanitize(&["foo"]), ["foo"]);
        assert_eq!(sanitize(&["foo", "/bar"]), ["foo", "/bar"]);
        assert_eq!(sanitize(&["foo", "/"]), ["foo", "/"]);
        assert_eq!(sanitize(&["foo", "bar/"]), ["foo", "bar"]);
    }


    #[cfg(windows)]
    #[test]
    fn test_sanitize() {
        let a: Vec<&str> = vec![];
        let b: Vec<&str> = vec![];
        assert_eq!(sanitize(&a), b);

        assert_eq!(sanitize(&[""]), [""]);
        assert_eq!(sanitize(&["foo"]), ["foo"]);
        assert_eq!(sanitize(&["foo", "\\bar"]), ["foo", "\\bar"]);
        assert_eq!(sanitize(&["foo", "\\"]), ["foo", "\\"]);
        assert_eq!(sanitize(&["foo", "bar\\"]), ["foo", "bar"]);
    }
}
