use common::Config;
use day_03::*;
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

    let mut total_a: u64 = 0;
    let mut total_b: u64 = 0;

    for line in contents
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
    {
        let bb = BatteryBank::parse(line);
        total_a += 0;
        total_b += bb.get_max(12);
    }

    println!("Part 1: {total_a}, Part 2: {total_b}");

    Ok(())
}
