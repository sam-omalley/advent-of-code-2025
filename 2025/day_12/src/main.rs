use common::Config;
use day_12::*;
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

    let mut shape = Vec::new();
    let mut shapes = Vec::new();
    let mut spaces = Vec::new();

    for line in contents.lines().map(str::trim) {
        if line.contains("x") {
            spaces.push(Space::new(line));
        } else if line.is_empty() {
            shapes.push(Shape::new(&shape));
            shape.clear();
        } else if line.contains(".") || line.contains("#") {
            shape.push(line);
        }
    }

    for (idx, shape) in shapes.iter().enumerate() {
        println!("Shape: {idx}");
        println!("{shape}");
        println!();
    }

    let mut easy_fit = 0;
    let mut easy_reject = 0;
    let mut other = 0;
    let mut total = 0;
    for space in spaces {
        println!(
            "Space ({}x{}) [{}]:",
            space.width,
            space.length,
            space.area()
        );
        //println!("{space}");
        //println!();

        let (min, max) = space.get_bounds(&shapes);
        println!("{min} -> {max}");

        if max <= space.area() {
            easy_fit += 1;
        } else if min > space.area() {
            easy_reject += 1;
        } else {
            other += 1;
        }
        total += 1;
    }

    println!("Day 12 Part 1");
    println!(" - Easy Reject: {easy_reject}");
    println!(" - Easy Fit: {easy_fit}");
    println!(" - Other: {other}");
    println!(" - Total: {total}");

    Ok(())
}
