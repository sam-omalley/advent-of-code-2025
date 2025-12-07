use common::Config;
use day_07::*;
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

    let manifold = Manifold::parse(&contents);

    for s in manifold.splitters.iter() {
        println!("Splitter: {s:?}");
    }

    Ok(())
}
