pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        } else if args.len() > 2 {
            return Err("Too many arguments");
        }

        let file_path = args[1].clone();

        Ok(Config { file_path })
    }
}

enum Move {
    Left(i32),
    Right(i32),
}

impl Move {
    fn parse(code: &str) -> Self {
        let code = code.trim();

        let delta = code[1..].parse::<i32>().unwrap();
        if code.starts_with("L") {
            Move::Left(delta)
        } else {
            Move::Right(delta)
        }
    }

    fn apply(&self, position: i32) -> i32 {
        let delta: i32 = match *self {
            Move::Left(clicks) => -clicks,
            Move::Right(clicks) => clicks,
        };

        // Rust modulo keeps the sign of the input, so we need to use mathematical modulo.
        (position + delta).rem_euclid(100)
    }
}

pub fn get_password(input: &str) -> i32 {
    let mut password: i32 = 0;
    let mut position: i32 = 50;

    for line in input.lines() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let movement = Move::parse(line);

        position = movement.apply(position);

        if position == 0 {
            password += 1;
        }
    }

    password
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dial() {
        assert_eq!(82, Move::Left(68).apply(50));
        assert_eq!(52, Move::Left(30).apply(82));
        assert_eq!(0, Move::Right(48).apply(52));
        assert_eq!(95, Move::Left(5).apply(0));
        assert_eq!(55, Move::Right(60).apply(95));
        assert_eq!(0, Move::Left(55).apply(55));
        assert_eq!(99, Move::Left(1).apply(0));
        assert_eq!(0, Move::Left(99).apply(99));
        assert_eq!(14, Move::Right(14).apply(0));
        assert_eq!(32, Move::Left(82).apply(14));
    }

    #[test]
    fn test_script() {
        let input = "
            L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82";

        assert_eq!(3, get_password(input));
    }
}
