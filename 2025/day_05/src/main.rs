use common::Config;
use day_05_2025::*;
use std::env;
use std::error;
use std::fs;
use std::ops::RangeInclusive;

fn main() -> Result<(), Box<dyn error::Error>> {
    let config = Config::build(env::args())?;
    run(config)?;

    Ok(())
}

fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let mut values = Vec::<u64>::new();
    let mut intervals = Vec::<RangeInclusive<u64>>::new();
    for line in contents.lines().map(|x| x.trim()).filter(|x| !x.is_empty()) {
        let mut iter = line.split("-");

        let first: u64 = iter.next().unwrap().parse()?;
        let second = iter.next();

        match second {
            Some(val) => {
                let end: u64 = val.parse()?;
                intervals.push(first..=end);
            }
            None => {
                values.push(first);
            }
        }
    }

    let intervals = squash_intervals(intervals);
    let count = count_members(&intervals, &values);
    let possible = count_possible_intervals(&intervals);

    println!("Part 1: {count}");
    println!("Part 2: {possible}");

    Ok(())
}
