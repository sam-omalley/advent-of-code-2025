use common::Config;
use day_02::*;
use std::env;
use std::error;
use std::fs;

fn main() -> Result<(), Box<dyn error::Error>> {
    let config = Config::build(env::args())?;
    run(config)?;

    Ok(())
}

fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let mut total_a = 0;
    let mut total_b = 0;

    for interval in contents.split(",") {
        let interval = interval.trim();
        if interval.is_empty() {
            continue;
        }

        let Some((a, b)) = interval.split_once("-") else {
            eprintln!("Error processing {interval}");
            std::process::exit(1);
        };

        let a: u64 = a.parse()?;
        let b: u64 = b.parse()?;
        total_a += check_all_naughty_pairs(a, b);
        total_b += check_all_naughty_patterns(a, b);
    }
    println!("Total Part A: {total_a}, Total Part B: {total_b}");

    Ok(())
}
