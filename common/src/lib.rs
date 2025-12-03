#[derive(Debug)]
pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        let usage = "usage: <program> <filepath>";

        // Consume argv[0]
        args.next();

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err(usage),
        };

        if args.next().is_some() {
            return Err(usage);
        }

        Ok(Self {
            file_path,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_path() {
        let result = Config::build(vec!["app".to_string(), "input.txt".to_string()].into_iter());
        assert!(matches!(result, Ok(_)));

        assert_eq!(result.unwrap().file_path, "input.txt");
    }

    #[test]
    fn bad_path() {
        let result = Config::build(vec!["app".to_string()].into_iter());
        assert!(matches!(result, Err(_)));

        let result = Config::build(vec![
            "app".to_string(),
            "input.txt".to_string(),
            "other".to_string(),
        ].into_iter());
        assert!(matches!(result, Err(_)));
    }
}
