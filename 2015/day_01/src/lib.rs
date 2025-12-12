pub fn calculate_floor(s: &str) -> i64 {
    let mut counter = 0;
    for char in s.chars() {
        counter += if char == '(' {
            1
        } else if char == ')' {
            -1
        } else {
            0
        }
    }

    counter
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
    }
}
