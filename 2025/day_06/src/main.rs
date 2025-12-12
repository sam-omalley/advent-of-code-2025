use common::Config;
use day_06::*;
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

    let mut inputs = Vec::<Vec<&str>>::new();
    let num_problems = contents
        .lines()
        .map(|x| x.trim())
        .find(|x| !x.is_empty())
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .len();
    inputs.resize(num_problems, Vec::new());

    let mut operations = vec![Operations::None; num_problems];

    for line in contents.lines().map(|x| x.trim()).filter(|x| !x.is_empty()) {
        for (index, token) in line.split_whitespace().enumerate() {
            if token.parse::<i64>().is_ok() {
                inputs[index].push(token);
            } else {
                operations[index] = token.parse()?;
            }
        }
    }

    let mut total_a = 0i64;
    for (column, operation) in inputs.into_iter().zip(operations.iter()) {
        for val in &column {
            print!("{val} ")
        }

        let res = operation.apply(column.iter().map(|x| x.parse().unwrap()));
        println!("{operation:?} = {res}");

        total_a += res;

    }

    println!("Part 1: {total_a}");

    let num_chars = contents.lines().next().unwrap().chars().count();
    let num_lines = contents.lines().count();

    let mut counter = 0;


    let mut total_b = 0i64;
    let mut values = Vec::<i64>::new();
    for idx in (0..num_chars).rev() {
        let mut val = String::new();
        for line in contents.lines().take(num_lines - 1) {
            val += &line.chars().nth(idx).unwrap().to_string();
        }

        if val.trim().is_empty() {
            let op = contents.lines().nth(num_lines - 1).unwrap().split_whitespace().rev().nth(counter).unwrap();
            let operation: Operations = op.parse()?;

            let res = operation.apply(values.into_iter());
            println!("{operation:?} = {res}");
            total_b += res;

            values = Vec::<i64>::new();
            counter += 1;
        } else {
            let val: i64 = val.trim().parse()?;
            print!("{val} ");
            values.push(val);
        }

    }

    let op = contents.lines().nth(num_lines - 1).unwrap().split_whitespace().rev().nth(counter).unwrap();
    let operation: Operations = op.parse()?;

    let res = operation.apply(values);
    println!("{operation:?} = {res}");
    total_b += res;

    println!("Part 2: {total_b}");

    Ok(())
}
