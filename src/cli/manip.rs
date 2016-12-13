use std::env;
use std::path;

use super::super::Config;
use super::super::data;
use super::super::data::Entry;


const DEFAULT_INCREASE_WEIGHT: isize = 10;
const DEFAULT_DECREASE_WEIGHT: isize = 15;


fn increase_weight(old_w: f64, inc_w: f64) -> f64 {
    (old_w.powi(2) + inc_w.powi(2)).sqrt()
}


fn decrease_weight(old_w: f64, dec_w: f64) -> f64 {
    let result = old_w - dec_w;

    if result < 0.0 { 0.0 } else { result }
}


fn do_increase<P>(entries: &mut Vec<Entry>, p: P, w: f64) -> Entry
    where P: AsRef<path::Path>
{
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
    where P: AsRef<path::Path>
{
    let mut entries = data::load(config);
    let entry = do_increase(&mut entries, p, w);
    data::save(config, &entries).unwrap();
    entry
}


fn do_decrease<P>(entries: &mut Vec<Entry>, p: P, w: f64) -> Entry
    where P: AsRef<path::Path>
{
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
    where P: AsRef<path::Path>
{
    let mut entries = data::load(config);
    let entry = do_decrease(&mut entries, p, w);
    data::save(config, &entries).unwrap();
    entry
}


pub fn add<P>(config: &Config, p: P)
    where P: AsRef<path::Path>
{
    do_increase_and_save(config, p, DEFAULT_INCREASE_WEIGHT as f64);
}


pub fn increase(config: &Config, w: Option<isize>) {
    let w = w.unwrap_or(DEFAULT_INCREASE_WEIGHT) as f64;
    let p = env::current_dir().unwrap();
    let entry = do_increase_and_save(config, p, w);
    println!("{}", entry);
}


pub fn decrease(config: &Config, w: Option<isize>) {
    let w = w.unwrap_or(DEFAULT_DECREASE_WEIGHT) as f64;
    let p = env::current_dir().unwrap();
    let entry = do_decrease_and_save(config, p, w);
    println!("{}", entry);
}
