use common::Config;
use day_11::*;
use petgraph::algo;
use petgraph::visit::{Reversed, Topo};
use std::collections::HashSet;
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

    let (graph, lookup) = parse_graph(&contents);

    if lookup.contains_key("you") {
        let origin = lookup["you"];
        let end = lookup["out"];

        let result = algo::all_simple_paths::<Vec<_>, _, RandomState>(&graph, origin, end, 0, None);

        let part_1_counter = result.count();
        println!("Part 1: {part_1_counter}");
    }

    if lookup.contains_key("svr") {

        let svr = lookup["svr"];
        let dac = lookup["dac"];
        let fft = lookup["fft"];
        let out = lookup["out"];

        let svr_to_dac = count_paths_dag(&graph, svr, dac);
        let svr_to_fft = count_paths_dag(&graph, svr, fft);
        let fft_to_dac = count_paths_dag(&graph, fft, dac);
        let dac_to_fft = count_paths_dag(&graph, dac, fft);
        let dac_to_out = count_paths_dag(&graph, dac, out);
        let fft_to_out = count_paths_dag(&graph, fft, out);

        println!("Total: {}", (svr_to_dac * dac_to_fft * fft_to_out) + (svr_to_fft * fft_to_dac * dac_to_out));
    }

    Ok(())
}
