use fancy_regex::Regex;

pub fn part_1(contents: &str) -> usize {
    let mut counter = 0;
    for line in contents.lines() {
        let line = line.trim();

        if is_nice(line) {
            counter += 1;
        }
    }

    counter
}

pub fn part_2(contents: &str) -> usize {
    0
}

/*

It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.

*/
pub fn is_nice(s: &str) -> bool {
    let repeated_chars = Regex::new(
        r"([a-z])\1",
    )
    .unwrap()
    .is_match(s).unwrap();
    let num_vowels = Regex::new(r"[aeiou]").unwrap().find_iter(s).count();
    let contains_forbidden = Regex::new(r"(ab|cd|pq|xy)").unwrap().is_match(s).unwrap();

    repeated_chars && num_vowels >= 3 && !contains_forbidden
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(is_nice("ugknbfddgicrmopn"));
        assert!(is_nice("aaa"));
        assert!(!is_nice("jchzalrnumimnmhp"));
        assert!(!is_nice("haegwjzuvuyypxyu"));
        assert!(!is_nice("dvszwmarrgswjxmb"));
    }
}
