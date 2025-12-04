pub struct BatteryBank(Vec<u8>);

impl BatteryBank {
    pub fn parse(line: &str) -> Self {
        Self(
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .map(|x| x as u8)
                .collect(),
        )
    }

    pub fn get_max(&self, digits: u32) -> u64 {
        let digits = digits as usize;
        let mut answer: u64 = 0;

        let mut bb = self.0.clone();

        for digit in 0..digits {
            let skip_idx = bb.len() - (digits - digit - 1);
            let max = *bb[..skip_idx].iter().max().unwrap();
            let max_idx = bb.iter().position(|&x| x == max).unwrap();

            bb.drain(..=max_idx);
            answer += max as u64 * 10u64.pow(digits as u32 - digit as u32 - 1);
        }

        println!("{answer}");
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn battery_bank_parse() {
        let bb = BatteryBank::parse("987654321111111");
        assert_eq!(bb.0, vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]);

        let bb = BatteryBank::parse("811111111111119");
        assert_eq!(bb.0, vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]);

        let bb = BatteryBank::parse("234234234234278");
        assert_eq!(bb.0, vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]);

        let bb = BatteryBank::parse("818181911112111");
        assert_eq!(bb.0, vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]);
    }

    #[test]
    fn get_max() {
        let bb = BatteryBank::parse("987654321111111");
        assert_eq!(bb.get_max(2), 98);
        assert_eq!(bb.get_max(12), 987654321111);

        let bb = BatteryBank::parse("811111111111119");
        assert_eq!(bb.get_max(2), 89);
        assert_eq!(bb.get_max(12), 811111111119);

        let bb = BatteryBank::parse("234234234234278");
        assert_eq!(bb.get_max(2), 78);
        assert_eq!(bb.get_max(12), 434234234278);

        let bb = BatteryBank::parse("818181911112111");
        assert_eq!(bb.get_max(2), 92);
        assert_eq!(bb.get_max(12), 888911112111);
    }
}
