use common::Config;
use day_07_2025::*;
use std::env;
use std::error;
use std::fs;
use std::time::Instant;

fn main() -> Result<(), Box<dyn error::Error>> {
    let config = Config::build(env::args())?;
    run(config)?;

    Ok(())
}

fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let manifold = Manifold::parse(&contents);

    let start = Instant::now();
    println!("Part 1: {}", manifold.get_num_splits());
    eprintln!("Time elapsed: {:?}", start.elapsed());

    let start = Instant::now();
    println!("Part 2: {}", manifold.get_num_timelines());
    eprintln!("Time elapsed: {:?}", start.elapsed());

    Ok(())
}
