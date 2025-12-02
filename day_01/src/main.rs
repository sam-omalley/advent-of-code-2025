use common::Config;
use day_01::get_password;
use std::{env, error, fs};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args)?;
    run(config)?;

    Ok(())
}

fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let (password, num_spins) = get_password(&contents);
    println!("The day-1-part-1 result is: {password}");
    println!("The day-1-part-2 result is {num_spins}");

    Ok(())
}
