use common::Config;
use core::f64;
use day_08::*;
use itertools::Itertools;
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

    let mut cloud = PointCloud::default();
    for line in contents.lines() {
        cloud.add_str(line)?;
    }

    let mut min_idx = 0;
    let mut min_val = f64::INFINITY;
    for (idx, val) in cloud
        .points
        .iter()
        .combinations(2)
        .enumerate()
        .map(|(idx, pair)| (idx, pair.first().unwrap().distance(pair.last().unwrap())))
    {
        if val < min_val {
            min_val = val;
            min_idx = idx;
        }
    }
    println!(
        "{min_val}, {:?}",
        cloud.points.iter().combinations(2).nth(min_idx)
    );

    Ok(())
}
