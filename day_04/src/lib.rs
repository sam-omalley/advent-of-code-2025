pub struct Map {
    grid: Vec<Vec<u8>>,
}

impl Map {
    pub fn new<'a>(width: usize, lines: impl Iterator<Item = &'a str>) -> Self {
        let mut grid = Vec::<Vec<u8>>::new();

        let row: Vec<u8> = vec![0; width + 2];
        grid.push(row.clone());

        for line in lines {
            let mut row = Vec::<u8>::new();

            row.push(0);
            for c in line.chars() {
                if c == '.' {
                    row.push(0);
                } else if c == '@' {
                    row.push(1);
                } else {
                    panic!("Unexpected char: {c}")
                }
            }
            row.push(0);
            grid.push(row);
        }
        grid.push(row.clone());

        Self { grid }
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.grid[y][x]
    }

    pub fn min_x(&self) -> usize {
        1
    }

    pub fn min_y(&self) -> usize {
        1
    }

    pub fn max_x(&self) -> usize {
        self.grid.first().unwrap_or(&vec![]).len() - 2
    }

    pub fn max_y(&self) -> usize {
        self.grid.len() - 2
    }

    pub fn get_rolls(&self) -> u32 {
        let mut total: u32 = 0;
        for y in self.min_y()..=self.max_y() {
            for x in self.min_x()..=self.max_x() {
                if self.get(x, y) == 0 {
                    continue;
                }
                let count = self.get(x - 1, y - 1)
                    + self.get(x, y - 1)
                    + self.get(x + 1, y - 1)
                    + self.get(x + 1, y)
                    + self.get(x + 1, y + 1)
                    + self.get(x, y + 1)
                    + self.get(x - 1, y + 1)
                    + self.get(x - 1, y);

                if count < 4 {
                    total += 1;
                }
            }
        }

        total
    }

    pub fn print(&self) {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[test]
    fn map() {
        let width = SAMPLE.lines().next().unwrap().trim().len();
        let map = Map::new(
            width,
            SAMPLE.lines().map(|x| x.trim()).filter(|x| !x.is_empty()),
        );
        assert_eq!(map.get(0, 0), 0);
        assert_eq!(map.get(7, 0), 0);
        assert_eq!(map.get(2, 1), 0);
        assert_eq!(map.get(3, 1), 1);
        assert_eq!(map.get(4, 1), 1);
        assert_eq!(map.get(5, 1), 0);

        map.print();
        assert_eq!(map.get_rolls(), 13);
    }
}
