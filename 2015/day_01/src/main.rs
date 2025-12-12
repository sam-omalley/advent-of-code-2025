use common::Config;
use day_01_2015::*;
use std::{env, error, fs};

fn main() -> Result<(), Box<dyn error::Error>> {
    let config = Config::build(env::args())?;
    run(config)?;

    Ok(())
}

fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("Part 1: {}", calculate_floor(contents.trim()));
    println!("Part 2: {}", first_basement_idx(contents.trim()));
    
    Ok(())
}
