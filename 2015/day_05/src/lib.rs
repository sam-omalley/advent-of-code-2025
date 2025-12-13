use fancy_regex::Regex;

pub fn part_1(contents: &str) -> usize {
    let mut counter = 0;
    for line in contents.lines() {
        let line = line.trim();

        if is_nice_part_1(line) {
            counter += 1;
        }
    }

    counter
}

pub fn part_2(contents: &str) -> usize {
    let mut counter = 0;
    for line in contents.lines() {
        let line = line.trim();

        if is_nice_part_2(line) {
            counter += 1;
        }
    }

    counter
}

pub fn is_nice_part_1(s: &str) -> bool {
    let repeated_chars = Regex::new(r"([a-z])\1").unwrap().is_match(s).unwrap();
    let num_vowels = Regex::new(r"[aeiou]").unwrap().find_iter(s).count();
    let contains_forbidden = Regex::new(r"(ab|cd|pq|xy)").unwrap().is_match(s).unwrap();

    repeated_chars && num_vowels >= 3 && !contains_forbidden
}

/*

    It contains a pair of any two letters that appears at least twice in the string without overlapping, like xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).
    It contains at least one letter which repeats with exactly one letter between them, like xyx, abcdefeghi (efe), or even aaa.
 */
pub fn is_nice_part_2(s: &str) -> bool {
    let duplicate_pairs = Regex::new(r"([a-z]{2}).*\1").unwrap().is_match(s).unwrap();
    let triplet = Regex::new(r"([a-z]).\1").unwrap().is_match(s).unwrap();

    duplicate_pairs && triplet
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert!(is_nice_part_1("ugknbfddgicrmopn"));
        assert!(is_nice_part_1("aaa"));
        assert!(!is_nice_part_1("jchzalrnumimnmhp"));
        assert!(!is_nice_part_1("haegwjzuvuyypxyu"));
        assert!(!is_nice_part_1("dvszwmarrgswjxmb"));
    }

    #[test]
    fn part_2() {
        assert!(is_nice_part_2("qjhvhtzxzqqjkmpb"));
        assert!(is_nice_part_2("xxyxx"));
        assert!(!is_nice_part_2("uurcxstgmygtbstg"));
        assert!(!is_nice_part_2("ieodomkazucvgmuy"));
    }
}
