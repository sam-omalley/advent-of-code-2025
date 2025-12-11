use common::Config;
use day_11::*;
use petgraph::algo;
use std::collections::hash_map::RandomState;
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

    let (graph, origin, end) = parse_graph(&contents);

    println!("{origin:?} -> {end:?}");
    println!("{graph:?}");

    let result = algo::all_simple_paths::<Vec<_>, _, RandomState>(&graph, origin, end, 0, None);

    let mut part_1_counter = 0;
    for res in result {
        println!("{res:?}");
        part_1_counter += 1;
    }

    println!("Part 1: {part_1_counter}");

    Ok(())
}
