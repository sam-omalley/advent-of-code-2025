#[derive(Clone)]
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

    pub fn set(&mut self, x: usize, y: usize, val: u8) {
        self.grid[y][x] = val;
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

    pub fn count_neighbours(&self, x: usize, y: usize) -> u32 {
        if x < self.min_x() || x > self.max_x() || y < self.min_y() || y > self.max_y() {
            panic!("({x},{y}) outside bounds of grid");
        }
        let mut total: u32 = 0;

        #[rustfmt::skip]
        let offsets = [
            (-1, -1), (0, -1), (1, -1),
            (-1,  0),          (1, 0 ),
            (-1,  1), (0, 1 ), (1, 1 ),
        ];

        for (xoff, yoff) in offsets {
            total += self.get((x as i32 + xoff) as usize, (y as i32 + yoff) as usize) as u32;
        }

        total
    }

    pub fn get_rolls(&self, deep: bool) -> u32 {
        let mut total: u32 = 0;
        let mut made_changes = true;
        let mut result_map = self.clone();
        let mut first = true;

        while (first || deep) && made_changes {
            made_changes = false;
            first = false;
            let map = result_map.clone();

            for y in map.min_y()..=map.max_y() {
                for x in map.min_x()..=map.max_x() {
                    if map.get(x, y) == 0 {
                        continue;
                    }
                    let count = map.count_neighbours(x, y);

                    if count < 4 {
                        total += 1;
                        made_changes = true;
                        result_map.set(x, y, 0);
                    }
                }
            }
        }

        total
    }

    pub fn print(&self) {}
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
        assert_eq!(map.get_rolls(true), 13);
        assert_eq!(map.get_rolls(false), 43);
    }
}
