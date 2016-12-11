use std::env;
use std::path;

use autojump::Config;
use autojump_data;
use autojump_data::Entry;


const DEFAULT_INCREASE_WEIGHT: f64 = 10.0;
// const DEFAULT_DECREASE_WEIGHT: f64 = 15.0;


fn increase_weight(old_w: f64, inc_w: f64) -> f64 {
    (old_w.powi(2) + inc_w.powi(2)).sqrt()
}


fn decrease_weight(old_w: f64, dec_w: f64) -> f64 {
    let result = old_w - dec_w;

    if result < 0.0 {
        0.0
    } else {
        result
    }
}


fn do_increase<P>(entries: &mut Vec<Entry>, p: P, w: f64) -> Entry
        where P: AsRef<path::Path> {
    let p = p.as_ref();

    // don't process $HOME
    if let Some(home) = env::home_dir() {
        if p == home {
            // synthesize a fake entry with weight zeroed
            return Entry::new(p, 0.0);
        }
    }

    for ent in entries.iter_mut() {
        if ent.path == p {
            let new_weight = increase_weight(ent.weight, w);
            ent.weight = new_weight;
            return ent.clone();
        }
    }

    // add the path
    let entry = Entry::new(p, w);
    entries.push(entry.clone());
    entry
}


fn do_increase_and_save<P>(config: &Config, p: P, w: f64) -> Entry
        where P: AsRef<path::Path> {
    let mut entries = autojump_data::load(config);
    let entry = do_increase(&mut entries, p, w);
    autojump_data::save(config, &entries).unwrap();
    entry
}


fn do_decrease<P>(entries: &mut Vec<Entry>, p: P, w: f64) -> Entry
        where P: AsRef<path::Path> {
    let p = p.as_ref();
    for ent in entries.iter_mut() {
        if ent.path == p {
            let new_weight = decrease_weight(ent.weight, w);
            ent.weight = new_weight;
            return ent.clone();
        }
    }

    // TODO: original impl also adds an entry in case the requested path is
    // absent, but is it desirable?
    // For now let's mimic its behavior...
    let entry = Entry::new(p, 0.0);  // no need to compare
    entries.push(entry.clone());
    entry
}


fn do_decrease_and_save<P>(config: &Config, p: P, w: f64) -> Entry
        where P: AsRef<path::Path> {
    let mut entries = autojump_data::load(config);
    let entry = do_decrease(&mut entries, p, w);
    autojump_data::save(config, &entries).unwrap();
    entry
}


pub fn add<P>(config: &Config, p: P)
        where P: AsRef<path::Path> {
    do_increase_and_save(config, p, DEFAULT_INCREASE_WEIGHT);
}


pub fn increase(config: &Config, w: f64) {
    let p = env::current_dir().unwrap();
    let entry = do_increase_and_save(config, p, w);
    println!("{}", entry);
}


pub fn decrease(config: &Config, w: f64) {
    let p = env::current_dir().unwrap();
    let entry = do_decrease_and_save(config, p, w);
    println!("{}", entry);
}
