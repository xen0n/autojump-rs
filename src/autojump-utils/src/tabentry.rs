#[derive(PartialEq, Eq, Debug)]
pub struct TabEntryInfo<'a> {
    pub needle: Option<&'a str>,
    pub index: Option<usize>,
    pub path: Option<&'a str>,
}


/// Given a tab entry in the following format return needle, index, and path:
///
/// ```ignore
/// [needle]__[index]__[path]
/// ```
pub fn get_tab_entry_info<'a>(entry: &'a str) -> TabEntryInfo<'a> {
    get_tab_entry_info_internal(entry, "__")
}


fn get_tab_entry_info_internal<'a>(entry: &'a str, separator: &'a str) -> TabEntryInfo<'a> {
    let mut needle = None;
    let mut index = None;
    let mut path = None;

    // extra scope needed to make `parse_index` and final return play well
    // together
    {
        // "0" => Some(0)
        // "x" => None
        // "01" => Some(0) (first digit)
        let mut parse_index = |index_s: &str| {
            if let Some(ch) = index_s.chars().next() {
                if let Some(index_u32) = ch.to_digit(10) {
                    index = Some(index_u32 as usize);
                }
            }
        };

        if let Some(i) = entry.find(separator) {
            let (needle_s, remaining) = entry.split_at(i);
            let (_, remaining) = remaining.split_at(separator.len());

            needle = Some(needle_s);

            // It seems the index part is a single digit according to the
            // original implementation.
            if let Some(i) = remaining.find(separator) {
                // Path part is present.
                let (index_s, path_s) = remaining.split_at(i);
                let (_, path_s) = path_s.split_at(separator.len());

                // Parse the index part.
                parse_index(index_s);

                // Pass-through the path part.
                path = Some(path_s);
            } else {
                // Only the index part is present.
                parse_index(remaining);
            }
        } else {
            // No separators at all, the original implementation returned all
            // None's.
        }
    }

    TabEntryInfo {
        needle: needle,
        index: index,
        path: path,
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    macro_rules! assert_tab_entry_info {
        ($input: expr, $needle: expr, $index: expr, $path: expr) => {
            let expected = TabEntryInfo {
                needle: $needle,
                index: $index,
                path: $path,
            };
            assert_eq!(get_tab_entry_info($input), expected);
        };
    }


    #[test]
    fn test_tab_entry_info_parse_wellformed() {
        assert_tab_entry_info!("", None, None, None);
        assert_tab_entry_info!("a__0", Some("a"), Some(0), None);
        assert_tab_entry_info!("a__0__b", Some("a"), Some(0), Some("b"));
    }


    #[test]
    fn test_tab_entry_info_parse_malformed() {
        assert_tab_entry_info!("a", None, None, None);
        assert_tab_entry_info!("a__", Some("a"), None, None);
        assert_tab_entry_info!("a__x", Some("a"), None, None);
        assert_tab_entry_info!("a____b", Some("a"), None, Some("b"));
    }

    #[test]
    fn test_tab_entry_info_parse_malformed_deviations() {
        // Original impl: Some("a"), Some(0), None
        assert_tab_entry_info!("a__01__b", Some("a"), Some(0), Some("b"));
    }
}
