use common::Config;
use day_04::*;
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

    let width = contents.lines().next().unwrap().trim().len();

    let grid = Map::new(
        width,
        contents.lines().map(|x| x.trim()).filter(|x| !x.is_empty()),
    );

    let total = grid.get_rolls();
    println!("Part 1: {total}");

    let total = grid.get_rolls_exhaustive();
    println!("Part 2: {total}");

    Ok(())
}
