use std::path;

use regex;


pub fn prepare_regex<F>(needles: &[&str], f: F, ignore_case: bool) -> regex::Regex
where
    F: Fn(&[&str]) -> String,
{
    let re = {
        let mut tmp = String::new();
        tmp.push_str(if ignore_case { "(?iu)" } else { "(?u)" });
        tmp.push_str(&f(needles));
        tmp
    };
    regex::Regex::new(&re).unwrap()
}


/// Port of Python's `re.escape()`, except that '/' is passed as-is.
fn re_escape(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '0'..='9' | 'A'..='Z' | 'a'..='z' | '_' | '/' => result.push(ch),
            '\\' => result.push_str(r"\\"),
            _ => {
                result.push_str(r"\x");
                // skip the r"\u" prefix and take the remaining "{xxxx}" part
                for escape_ch in ch.escape_unicode().skip(2) {
                    result.push(escape_ch);
                }
            }
        }
    }
    result
}


pub fn re_match_anywhere(needles: &[&str]) -> String {
    let mut result = String::new();
    result.push_str(r".*");
    for s in needles {
        result.push_str(&re_escape(s));
        result.push_str(r".*");
    }
    result
}


pub fn re_match_consecutive(needles: &[&str]) -> String {
    let sep = {
        let mut tmp = String::with_capacity(1);
        tmp.push(path::MAIN_SEPARATOR);
        re_escape(&tmp)
    };
    let no_sep = format!(r"[^{}]*", sep);

    let mut result = String::new();
    for (i, s) in needles.iter().enumerate() {
        if i > 0 {
            result.push_str(&no_sep);
            result.push_str(&sep);
            result.push_str(&no_sep);
        }
        result.push_str(&re_escape(s));
    }
    result.push_str(&no_sep);
    result.push('$');
    result
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_re_escape() {
        macro_rules! assert_re_escape {
            ($x: expr, $y: expr) => {
                assert_eq!(re_escape($x), $y);
            };
        }

        assert_re_escape!("", "");
        assert_re_escape!("test", "test");
        assert_re_escape!("a/b/c", "a/b/c");
        assert_re_escape!("test\0test", r"test\x{0}test");
        assert_re_escape!("测试", r"\x{6d4b}\x{8bd5}");
    }
}
