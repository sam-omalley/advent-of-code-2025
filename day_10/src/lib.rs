use std::fmt;

#[derive(Default, Clone, Copy)]
pub struct BitMask(pub u32);

impl BitMask {
    pub fn parse_indicators(s: &str) -> Self {
        let mut chars = s.chars().rev();

        // Pop the front and back [...] brackets.
        chars.next();
        chars.next_back();

        let mut mask: u32 = 0;

        for char in chars {
            if char == '.' {
                mask <<= 1;
            } else if char == '#' {
                mask <<= 1;
                mask += 1;
            } else {
                panic!("Bad input");
            }
        }

        Self(mask)
    }

    pub fn parse_button(s: &str) -> Self {
        let trimmed = &s[1..s.len()-1];

        let mut mask: u32 = 0;
        for token in trimmed.split(",") {

            let val: u32 = token.parse().unwrap();
            mask |= 1 << val;
        }

        Self(mask)
    }
}

impl fmt::Display for BitMask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:016b}", self.0)
    }
}
impl fmt::Debug for BitMask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = BitMask::parse_button("(0,2)");
        let b = BitMask::parse_button("(0,1)");
        let indicator = BitMask::parse_indicators("[.##.]");

        assert_eq!(a.0 ^ b.0, indicator.0);
    }
}
