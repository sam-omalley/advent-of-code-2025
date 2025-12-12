use common::Config;
use day_08::*;
use itertools::Itertools;
use std::collections::HashSet;
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

    let mut circuits = Vec::<HashSet<Point>>::default();
    let mut last_connection = (Point::default(), Point::default());

    'outer: for (p1, p2, distance) in cloud
        .points
        .iter()
        .combinations(2)
        .map(|pair| {
            let p1 = pair.first().unwrap();
            let p2 = pair.last().unwrap();
            (*p1, *p2, p1.distance2(p2))
        })
        .sorted_by(|(_, _, d1), (_, _, d2)| d1.partial_cmp(d2).unwrap())
        .take(if config.max > 0 {
            config.max
        } else {
            usize::MAX
        })
    {
        if circuits.len() == 1 && circuits[0].len() == cloud.points.len() {
            println!(
                "Part 2: {last_connection:?} = {}",
                (last_connection.0.0 * last_connection.1.0)
            );
            break;
        }
        last_connection = (p1.clone(), p2.clone());
        let mut candidate_circuits = HashSet::<usize>::default();
        for (idx, circuit) in circuits.iter().enumerate() {
            if circuit.contains(p1) || circuit.contains(p2) {
                candidate_circuits.insert(idx);
            }

            if circuit.contains(p1) && circuit.contains(p2) {
                println!(
                    "Skipped edge because it is in circuit #{idx}: {p1:?} -> {p2:?} - len: {distance}"
                );
                //println!("Grow junction box: {circuit:?}, {p1:?} {p2:?}");
                continue 'outer;
            }
        }

        if candidate_circuits.is_empty() {
            println!(
                "New circuit #{}: {p1:?} -> {p2:?} - len: {distance}",
                circuits.len()
            );
            let mut junction = HashSet::new();
            junction.insert(p1.clone());
            junction.insert(p2.clone());
            circuits.push(junction);
        } else if candidate_circuits.len() == 2 {
            let mut iter = candidate_circuits.iter();

            let a_idx = iter.next().unwrap();
            let b_idx = iter.next().unwrap();
            println!("Merged circuits #{a_idx} and #{b_idx}");

            let b = circuits[*b_idx].clone();
            circuits[*a_idx].extend(b);
            circuits.remove(*b_idx);

            continue 'outer;
        } else if candidate_circuits.len() == 1 {
            let mut iter = candidate_circuits.iter();
            let a_idx = iter.next().unwrap();
            println!("Added to #{a_idx} circuit: {p1:?} -> {p2:?} - len: {distance}");
            circuits[*a_idx].insert(p1.clone());
            circuits[*a_idx].insert(p2.clone());
            continue 'outer;
        } else {
            panic!("This shouldn't happen: {}", candidate_circuits.len());
        }
    }

    if config.max > 0 {
        let mut lengths: Vec<usize> = circuits.iter().map(|c| c.len()).collect();
        lengths.sort_unstable_by(|a, b| b.cmp(a));
        println!("Part 1: {}", lengths.iter().take(3).product::<usize>());
    }

    Ok(())
}
