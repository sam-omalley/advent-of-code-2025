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
    for line in contents.lines().map(str::trim) {
        if line.contains("x") {
            let mut iter = line.split(":");
            let size = iter.next().unwrap().trim();
            let requirements = iter.next().unwrap().trim();

            println!("Size: {size}");
            println!("Requirements: {requirements}");
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

    Ok(())
}
