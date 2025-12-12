#[derive(Copy, Clone, Debug, Default)]
pub struct Shape {
    pub shape: [bool; 9],
}

impl Shape {
    pub fn new(s: &Vec<&str>) -> Self {
        let mut shape = Self { shape: [false; 9] };
        for (row, line) in s.iter().enumerate() {
            for (col, char) in line.chars().enumerate() {
                shape.shape[row * 3 + col] = char == '#'
            }
        }

        shape
    }
}

impl std::fmt::Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, val) in self.shape.iter().enumerate() {
            if *val {
                f.write_str("#")?;
            } else {
                f.write_str(".")?;
            }
            if idx % 3 == 2 && idx < self.shape.len() - 1 {
                f.write_str("\n")?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct Space {
    space: Vec<bool>,
    pub width: usize,
    pub length: usize,
    pub requirements: Vec<usize>,
}

impl Space {
    pub fn new(s: &str) -> Self {
        let mut res = Self::default();

        let mut iter = s.split(":");
        let size = iter.next().unwrap().trim();
        for requirement in iter.next().unwrap().split_whitespace() {
            res.requirements.push(requirement.parse().unwrap());
        }

        let mut iter = size.split("x");
        res.width = iter.next().unwrap().parse().unwrap();
        res.length = iter.next().unwrap().parse().unwrap();

        res.space.resize(res.width * res.length, false);

        res
    }

    pub fn area(&self) -> usize {
        self.width * self.length
    }

    pub fn get_bounds(&self, shapes: &[Shape]) -> (usize, usize) {
        let mut max_area = 0;
        let mut min_area = 0;

        for (idx, shape) in shapes.iter().enumerate() {
            min_area += self.requirements[idx] * shape.shape.iter().filter(|&c| *c).count();
            max_area += self.requirements[idx] * shape.shape.iter().count();
        }

        (min_area, max_area)

    }
}

impl std::fmt::Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, val) in self.space.iter().enumerate() {
            if *val {
                f.write_str("#")?;
            } else {
                f.write_str(".")?;
            }
            if idx % self.width == self.width - 1 && idx < self.space.len() - 1 {
                f.write_str("\n")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shape() {
        let shape = Shape::new(&vec!["###", "...", "##."]);
        assert_eq!(format!("{shape}"), "###\n...\n##.");
    }
}
