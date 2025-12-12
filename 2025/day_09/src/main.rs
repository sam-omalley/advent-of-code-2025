use common::Config;
use day_09_2025::*;
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

    let tiles = Tiles::parse(&contents)?;
    //tiles.print(14, 9, None);
    tiles.print(
        14,
        9,
        Rectangle::new(Point::new(2, 5), Point::new(11, 1)).into(),
    );

    let mut smallest_p = Point::new(i64::MAX, i64::MAX);
    let mut biggest_p = Point::new(i64::MIN, i64::MIN);

    for p in &tiles.tiles {
        smallest_p = smallest_p.min(*p);
        biggest_p = biggest_p.max(*p);
    }

    println!("{smallest_p:?} -> {biggest_p:?}");

    let mut rectangles: Vec<Rectangle> = tiles
        .tiles
        .iter()
        .combinations(2)
        .map(|v| Rectangle::new(**v.first().unwrap(), **v.last().unwrap()))
        .collect();
    rectangles.sort_by_key(|r| r.area());

    println!("Part 1 area: {}", rectangles.iter().last().unwrap().area());

    'outer: for r in rectangles.iter().rev() {
        for (p1, p2) in tiles
            .tiles
            .iter()
            .zip(tiles.tiles.iter().cycle().skip(1))
            .take(tiles.tiles.len())
        {
            if r.intersects(&Rectangle::new(*p1, *p2)) {
                continue 'outer;
            }
        }
        println!("Part 2: {r:?} => {}", r.area());
        break;
    }

    Ok(())
}
