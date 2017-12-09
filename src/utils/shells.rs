use std::env;


#[cfg(not(windows))]
pub fn is_autojump_sourced() -> bool {
    match env::var("AUTOJUMP_SOURCED") {
        Ok(s) => s == "1",
        // The only accepted value is "1", which is definitely valid UTF-8,
        // so if the value failed UTF-8 conversion it must be invalid.
        Err(_) => false,
    }
}
