use std::collections::HashSet;

#[derive(Default)]
pub struct Manifold {
    pub start: Option<(usize, usize)>,
    depth: usize,
    pub splitters: HashSet<(usize, usize)>,
}

impl Manifold {
    pub fn parse(contents: &str) -> Manifold {
        let mut manifold = Manifold::default();

        for (row, line) in contents.lines().enumerate() {
            for (col, char) in line.chars().enumerate() {
                if char == '^' {
                    manifold.splitters.insert((row, col));
                } else if char == 'S' {
                    manifold.start = Some((row, col));
                }
            }
            manifold.depth = row;
        }

        manifold
    }

    pub fn get_num_splits(&self) -> usize {
        let (start_row, start_col) = self.start.expect("Start _must_ be set");
        //self.get_num_splits_from(row, col).len()

        let mut count = 0;

        let mut beams = HashSet::<usize>::new();
        beams.insert(start_col);
        for row in start_row..self.depth {
            for beam in beams.clone() {
                if self.splitters.contains(&(row, beam)) {
                    count += 1;
                    beams.remove(&beam);
                    beams.insert(beam - 1);
                    beams.insert(beam + 1);
                }
            }
        }

        count
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifold() {
        let manifold = Manifold::parse(
            "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............",
        );

        assert_eq!(manifold.start, Some((0, 7)));
        assert_eq!(manifold.splitters.len(), 22);
        assert!(manifold.splitters.contains(&(8, 6)));

        assert_eq!(manifold.get_num_splits(), 21);
    }
}
