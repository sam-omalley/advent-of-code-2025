use std::collections::HashSet;

#[derive(Default)]
pub struct Manifold {
    pub start: Option<(usize, usize)>,
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
        }

        manifold
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
    }
}
