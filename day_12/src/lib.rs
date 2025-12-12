#[derive(Copy, Clone, Debug, Default)]
pub struct Shape {
    shape: [bool; 9],
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shape() {
        let shape = Shape::new(&vec!["###", "...", "##."]);
        assert_eq!(format!("{shape}"), "###\n...\n##.");
    }
}
