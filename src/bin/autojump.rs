extern crate autojump;


/// Get a version string suitable for the CLI display.
fn get_version_str() -> String {
    let mut tmp = String::new();
    tmp.push_str("autojump v");
    tmp.push_str(autojump::VERSION_TRACK);
    tmp.push_str("\nautojump-rs v");
    tmp.push_str(autojump::VERSION);

    tmp
}

fn main() {
    autojump::main(get_version_str());
}
