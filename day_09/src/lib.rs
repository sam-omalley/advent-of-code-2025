use std::error::Error;

#[derive(Clone, Default, Debug)]
pub struct Tiles {
    pub tiles: Vec<Point>,
}

impl Tiles {
    pub fn parse(val: &str) -> Result<Self, Box<dyn Error>> {
        let mut tiles = Tiles::default();

        for line in val.lines().map(str::trim) {
            if !line.is_empty() {
                tiles.add_str(line)?;
            }
        }
        Ok(tiles)
    }

    pub fn add_str(&mut self, val: &str) -> Result<(), Box<dyn Error>> {
        let parts: Vec<&str> = val.split(',').collect();

        if parts.len() == 2 {
            let p = Point {
                x: parts[0].parse()?,
                y: parts[1].parse()?,
            };

            self.tiles.push(p);
        } else {
            println!("String does not contain exactly 2 comma-separated values");
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let tiles = Tiles::parse(
            "
            7,1
            11,1
            11,7
            9,7
            9,5
            2,5
            2,3
            7,3
            ",
        );

        assert!(tiles.is_ok());

        assert_eq!(
            tiles.unwrap().tiles,
            vec![
                Point { x: 7, y: 1 },
                Point { x: 11, y: 1 },
                Point { x: 11, y: 7 },
                Point { x: 9, y: 7 },
                Point { x: 9, y: 5 },
                Point { x: 2, y: 5 },
                Point { x: 2, y: 3 },
                Point { x: 7, y: 3 },
            ]
        );
    }
}
