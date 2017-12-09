use super::super::Config;
use super::super::data;


pub fn purge(config: &Config) {
    let entries = data::load(config);
    let old_len = entries.len();
    let entries: Vec<_> = entries
        .into_iter()
        .filter(|ent| ent.path.exists())
        .collect();

    data::save(config, &entries).unwrap();

    let new_len = entries.len();
    println!("Purged {} entries.", old_len - new_len);
}
