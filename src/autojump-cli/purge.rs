use autojump::Config;
use autojump_data;


pub fn purge(config: &Config) {
    let entries = autojump_data::load(config);
    let old_len = entries.len();
    let entries: Vec<_> = entries.into_iter()
        .filter(|ent| ent.path.exists())
        .collect();

    // TODO: save

    let new_len = entries.len();
    println!("Purged {} entries.", old_len - new_len);
}
