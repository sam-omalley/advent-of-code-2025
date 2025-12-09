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

    pub fn print(&self, width: usize, height: usize, rectangle: Option<Rectangle>) {
        for row in 0..height {
            for col in 0..width {
                let p = Point {
                    x: col as i64,
                    y: row as i64,
                };
                if let Some(r) = rectangle
                    && r.encloses(&p)
                {
                    print!("O");
                    continue;
                }

                if self.tiles.contains(&p) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    pub fn new(p1: Point, p2: Point) -> Self {
        Rectangle { p1, p2 }
    }

    pub fn encloses(&self, p: &Point) -> bool {
        let min = self.min();
        let max = self.max();
        min.x <= p.x && p.x <= max.x && min.y <= p.y && p.y <= max.y
    }

    pub fn min(&self) -> Point {
        Point {
            x: self.p1.x.min(self.p2.x),
            y: self.p1.y.min(self.p2.y),
        }
    }

    pub fn max(&self) -> Point {
        Point {
            x: self.p1.x.max(self.p2.x),
            y: self.p1.y.max(self.p2.y),
        }
    }

    pub fn area(&self) -> i64 {
        let min = self.min();
        let max = self.max();

        (max.x - min.x + 1) * (max.y - min.y + 1)
    }

    pub fn intersects(&self, segment: &Rectangle) -> bool {
        let r_min = self.min();
        let r_max = self.max();
        let s_min = segment.min();
        let s_max = segment.max();

        !(r_min.x >= s_max.x || r_max.x <= s_min.x || r_min.y >= s_max.y || r_max.y <= s_min.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tile_map() {
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

    #[test]
    fn rectangle_area() {
        let r = Rectangle::new(Point::new(2, 5), Point::new(11, 1));

        assert_eq!(r.area(), 50);
    }
}
