pub fn calculate_floor(s: &str) -> i64 {
    let mut counter = 0;
    for char in s.chars() {
        counter += if char == '(' {
            1
        } else if char == ')' {
            -1
        } else {
            0
        };
    }

    counter
}

pub fn first_basement_idx(s: &str) -> usize {
    let mut counter = 0;
    for (idx, char) in s.chars().enumerate() {
        counter += if char == '(' {
            1
        } else if char == ')' {
            -1
        } else {
            0
        };

        if counter == -1 {
            return idx + 1;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn floors() {
        assert_eq!(calculate_floor("(())"), 0);
        assert_eq!(calculate_floor("()()"), 0);
        assert_eq!(calculate_floor("((("), 3);
        assert_eq!(calculate_floor("(()(()("), 3);
        assert_eq!(calculate_floor("))((((("), 3);
        assert_eq!(calculate_floor("())"), -1);
        assert_eq!(calculate_floor("))("), -1);
        assert_eq!(calculate_floor(")))"), -3);
        assert_eq!(calculate_floor(")())())"), -3);

        assert_eq!(first_basement_idx(")"), 1);
        assert_eq!(first_basement_idx("()())"), 5);
    }
}
