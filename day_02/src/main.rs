use std::io;
use std::io::IsTerminal;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // If stdin is a terminal (no pipe), exit early
    if io::stdin().is_terminal() {
        std::process::exit(1);
    }

    let mut line = String::new();
    io::stdin().read_line(&mut line)?;

    for interval in line.split(",") {
        if line.trim().is_empty() {
            continue;
        }

        let Some((a, b)) = interval.split_once("-") else {
            eprintln!("Error processing {interval}");
            std::process::exit(1);
        };

        println!("Interval: {interval}");

        println!("{a}      {b}");

        //let a: u64 = a.parse()?;
        ////let b: u64 = b.parse()?;

        //println!("{a}===={b}");

    }


    Ok(())
}
