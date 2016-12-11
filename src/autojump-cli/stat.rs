use autojump::Config;
use autojump_data;


pub fn print_stat(config: &Config) {
    // TODO: sort by weight
    let entries = autojump_data::load(config);
    let mut weight_sum = 0.0f64;
    for ref entry in &entries {
        println!("{:.1}:\t{}", entry.weight, entry.path.to_string_lossy());
        // NOTE: This isn't exactly accurate due to floating-point nature,
        // but since this is only an estimate let's get over it!
        weight_sum += entry.weight;
    }

    println!("________________________________________\n");
    println!("{}:\t total weight", weight_sum);
    println!("{}:\t number of entries", entries.len());
    // TODO: current directory weight

    println!("\ndata:\t {}", config.data_path.to_string_lossy());
}
