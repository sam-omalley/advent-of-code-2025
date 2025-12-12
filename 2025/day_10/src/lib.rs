use std::fmt;
use z3::{Optimize, SatResult, ast::Int};

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
        let trimmed = &s[1..s.len() - 1];

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

#[derive(Clone, Debug)]
pub struct Machine {
    requirements: Vec<u32>,
    buttons: Vec<Vec<usize>>,
}

pub fn get_csv(s: &str) -> Vec<&str> {
    // Slice off the brackets, and return csv iterator
    s[1..s.len() - 1].split(",").collect()
}

impl Machine {
    pub fn parse(line: &str) -> Self {
        let mut tokens = line.split_whitespace();

        // Pop front. It's unneeded.
        tokens.next().unwrap();

        // Get requirements from the end of the line.
        let requirements = {
            let s = tokens.next_back().unwrap();

            let mut requirements = Vec::<u32>::new();
            for token in get_csv(s) {
                requirements.push(token.parse().unwrap());
            }
            requirements
        };

        let buttons = {
            let mut buttons = Vec::<Vec<usize>>::new();
            for token in tokens {
                let mut wire = Vec::<usize>::new();
                for val in get_csv(token) {
                    wire.push(val.parse().unwrap());
                }
                buttons.push(wire);
            }
            buttons
        };

        Self {
            requirements,
            buttons,
        }
    }

    pub fn solve(&self) -> u64 {
        let opt = Optimize::new();

        let mut variables = Vec::new();
        for (idx, _) in self.buttons.iter().enumerate() {
            let var = Int::new_const(format!("x{}", idx));
            opt.assert(&var.ge(Int::from_u64(0)));

            variables.push(var);
        }

        for (requirement_idx, requirement) in self.requirements.iter().enumerate() {
            let mut function = Vec::new();
            for (button_idx, button) in self.buttons.iter().enumerate() {
                if button.contains(&requirement_idx) {
                    function.push(&variables[button_idx]);
                }
            }

            opt.assert(&function.iter().fold(Int::from_u64(0), |acc, &x| &acc + x).eq(Int::from_u64(*requirement as u64)));
        }

        let total = variables.iter().fold(Int::from_u64(0), |acc, x| &acc + x);
        opt.minimize(&total);

        let mut min_total = 0;
        // Solve
        match opt.check(&[]) {
            SatResult::Sat => {
                let model = opt.get_model().unwrap();
                min_total = model.eval(&total, true).unwrap().as_u64().unwrap();
                eprintln!("min total = {min_total}");
                for (i, v) in variables.iter().enumerate() {
                    eprintln!("x{} = {}", i, model.eval(v, true).unwrap());
                }
            }
            SatResult::Unsat => println!("unsat"),
            SatResult::Unknown => println!("unknown"),
        }
        min_total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let a = BitMask::parse_button("(0,2)");
        let b = BitMask::parse_button("(0,1)");
        let indicator = BitMask::parse_indicators("[.##.]");

        assert_eq!(a.0 ^ b.0, indicator.0);
    }

    #[test]
    fn part_2() {
        let machine = Machine::parse("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
        assert_eq!(machine.solve(), 10);

        let machine = Machine::parse("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}");
        assert_eq!(machine.solve(), 12);

        let machine = Machine::parse("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}");
        assert_eq!(machine.solve(), 11);
    }
}

// #######################################################################
// [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
//
//     03 05 04 07
//     --+--+--+--
// (2)   |1 |  |3    = 5 + 7 = 12
// (2)   |  |2 |3    = 4 + 7 = 11
// (2) 0 |1 |  |     = 3 + 5 = 8
// (2) 0 |  |2 |     = 3 + 4 = 7
// (1)   |  |  |3    = 7     = 7
// (1)   |  |2 |     = 4     = 4
//     -----------
//     2  2  3  3
//
//                      0  1  2  3
//                   = 03|05|04|07
// 5 * (1,3)         = 03|00|04|02
// 2 * (2,3)         = 03|00|02|00
// 0 * (0,1)
// 0 * (3)
// 2 * (2)           = 03|00|00|00
// FAIL

// Now lets try starting with the globally most constrained value
// Ordering within options still by influence.
//                      0  1  2  3
//                   = 03|05|04|07
// 3 * (0,1)         = 00|02|04|07
// 2 * (1,3)         = 00|00|02|07
// 2 * (2,3)         = 00|00|00|04
// 4 * (3)           = 00|00|00|00
// 11 Steps
//
// I think recursion is needed. Lets start with combinations of sets that reduce the tightest constraint:
//                      0  1  2  3
//                   = 03|05|04|07
//  tightest_rules = [(0,1), (0,2)].combinations(3)
// best_rule = min(for rule in tightest_rules {
//     counter.reduce(rule)
//  })
//  3 + calc([(3),(2),(2,3)] {0,5,4,7} //
// Each recursive call should reduce the possible calls.
//  calc((0,1), (3,5,4,7))
//  calc([(0,1), (0,2), (3,5,4,7)].combinations(3))

// #######################################################################
// [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}

// #######################################################################
// [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
//
//     10 11 11 05 10 05
//     --+--+--+--+--+--
// (5) 0 |1 |2 |3 |4 |   = 10 + 11 + 11 + 5 + 10       = 47
// (3) 0 |  |  |3 |4 |   = 10 + 5 + 10                 = 25
// (5) 0 |1 |2 |  |4 |5  = 10 + 11 + 5 + 10 + 5        = 41
// (2)   |1 |2 |  |  |   = 11 + 11                     = 22
//     -----------------
//     3 |3 |3 |2 |3 |1
//                        0  1  2  3  4  5
//  5 * (0,1,2,3,4)    = 05|06|06|00|05|05
//  5 * (0,1,2,4,5)    = 00|01|01|00|00|00
//  0 * (0,3,4)        =
//  1   (1,2)          = 00|00|00|00|00|00
//

// (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
// 7 + 4 + 5 + 3 = 19
// 19 / 2 = 9.5 = 10

// (3)       <= 7
// (1,3)     <= 5
// (2)       <= 4
// (2,3)     <= 4
// (0,2)     <= 3
// (0,1)     <= 3
// {3,5,4,7}

// 6 numbers. Largest set is size 2. Min 10

// [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}

// {}

// [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}

// {10,11,11,5,10,5}
// (0,1,2,3,4)   <= 5
// (0,1,2,4,5)   <= 5
// (0,3,4)       <= 10
// (1,2)         <= 11
//
//
//
// Ah F**k it. Z3 solver to minimise this system of equations. Let's go!
