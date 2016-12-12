use std::env;


pub fn is_autojump_sourced() -> bool {
    match env::var("AUTOJUMP_SOURCED") {
        Ok(s) => s == "1",
        // The only accepted value is "1", which is definitely valid UTF-8,
        // so if the value failed UTF-8 conversion it must be invalid.
        Err(_) => false,
    }
}


pub fn surround_quotes(s: String) -> String {
    if in_bash() {
        let mut result = String::with_capacity(s.len() + 2);
        result.push('"');
        result.push_str(&s);
        result.push('"');
        result
    } else {
        s
    }
}


fn in_bash() -> bool {
    in_bash_internal(env::var("SHELL"))
}


fn in_bash_internal(r: Result<String, env::VarError>) -> bool {
    match r {
        Ok(v) => v.contains("bash"),
        Err(e) => {
            match e {
                env::VarError::NotPresent => false,
                env::VarError::NotUnicode(s) => {
                    let s = s.to_string_lossy();
                    s.contains("bash")
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_in_bash_internal_ok() {
        assert_eq!(in_bash_internal(Ok(String::from("/bin/bash"))), true);
        assert_eq!(in_bash_internal(Ok(String::from("/bin/zsh"))), false);
    }


    #[test]
    fn test_in_bash_internal_err_absent() {
        assert_eq!(in_bash_internal(Err(env::VarError::NotPresent)), false);
    }


    #[cfg(unix)]
    #[test]
    fn test_in_bash_internal_err_invalid_unicode() {
        use std::ffi;
        use std::os::unix::ffi::OsStrExt;

        let invalid_unicode_factory = |s| {
            let mut tmp = ffi::OsString::new();
            tmp.push("/some/");
            // "测试" (test) is "\xe6\xb5\x8b\xe8\xaf\x95"
            // here \xaf is mutated to \xfa to simulate invalid OS strings
            tmp.push(ffi::OsStr::from_bytes(
                    &[0xe6, 0xb5, 0x8b, 0xe8, 0xfa, 0x95],
                    ));
            tmp.push("/prefix/bin/");
            tmp.push(s);
            Err(env::VarError::NotUnicode(tmp))
        };

        let invalid_unicode_bash = invalid_unicode_factory("bash");
        assert_eq!(in_bash_internal(invalid_unicode_bash), true);
        let invalid_unicode_zsh = invalid_unicode_factory("zsh");
        assert_eq!(in_bash_internal(invalid_unicode_zsh), false);
    }
}
