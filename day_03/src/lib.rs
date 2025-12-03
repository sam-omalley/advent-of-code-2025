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

    pub fn get_max(&self) -> u64 {
        let max_start = *self.0[..self.0.len() - 1].iter().max().unwrap();
        let idx = self.0.iter().position(|&x| x == max_start).unwrap();
        let max_end = *self.0[idx + 1 ..].iter().max().unwrap();

        (max_start as u64) * 10  + (max_end as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn battery_bank_parse() {
        let bb = BatteryBank::parse("987654321111111");
        assert_eq!(bb.0, vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]);
        // let bb = BatteryBank::parse("811111111111119");
        // let bb = BatteryBank::parse("234234234234278");
        // let bb = BatteryBank::parse("818181911112111");
    }

    #[test]
    fn get_max() {
        let bb = BatteryBank::parse("987654321111111");
        assert_eq!(bb.get_max(), 98);
    }
}
