use common::Config;
use day_10::*;
use std::env;
use std::error;
use std::fs;
use itertools::Itertools;

fn main() -> Result<(), Box<dyn error::Error>> {
    let config = Config::build(env::args())?;
    run(config)?;

    Ok(())
}

fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let mut counter: usize = 0;
    for line in contents.lines().map(str::trim) {
        let mut tokens = line.split_whitespace();

        let indicator = tokens.next().unwrap();
        let indicator_mask = BitMask::parse_indicators(indicator);
        println!("Indicator Mask: {indicator_mask}");
        println!("Indicator: {indicator}");
        let power = tokens.next_back().unwrap();


        let mut buttons = Vec::<BitMask>::default();
        for button in tokens {
            println!("{button} -> {}", BitMask::parse_button(button));
            buttons.push(BitMask::parse_button(button));
        }

        println!("Power: {power}");

        let mut combinations: usize = 0;
        'outer: loop {
            for combos in buttons.iter().combinations(combinations) {
                let result = combos.iter().copied().fold(BitMask::default(), |acc, x| BitMask(acc.0 ^ x.0));

                if result.0 == indicator_mask.0 {
                    println!("Found answer ({combinations}): {combos:?}");
                    counter += combinations;
                    break 'outer;
                }
            }
            combinations += 1;
        }
    }

    println!("Part 1: {counter}");

    Ok(())
}
