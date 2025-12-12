use common::Config;
use day_02_2015::*;
use std::{env, error, fs};

fn main() -> Result<(), Box<dyn error::Error>> {
    let config = Config::build(env::args())?;
    run(config)?;

    Ok(())
}

fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let mut total = 0;
    for line in contents.lines() {
        if line.contains("x") {
            let mut iter = line.split("x");

            let l: i64= iter.next().unwrap().parse().unwrap();
            let w: i64 = iter.next().unwrap().parse().unwrap();
            let h: i64 = iter.next().unwrap().parse().unwrap();

            total += (2 * l * w) + (2 * w * h) + (2 * h * l) + (l*w).min(w*h).min(h*l);
        }
    }

    println!("Part 1: {total}");

    Ok(())
}
