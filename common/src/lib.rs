pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config, String> {
        match args.as_slice() {
            [_, file_path] => Ok(Self {
                file_path: file_path.to_string(),
            }),
            _ => Err("usage: <program> <file>".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_path() {
        let result = Config::build(&vec!["app".to_string(), "input.txt".to_string()]);
        assert!(matches!(result, Ok(_)));

        assert_eq!(result.unwrap().file_path, "input.txt");
    }

    #[test]
    fn bad_path() {
        let result = Config::build(&vec!["app".to_string()]);
        assert!(matches!(result, Err(_)));

        let result = Config::build(&vec![
            "app".to_string(),
            "input.txt".to_string(),
            "other".to_string(),
        ]);
        assert!(matches!(result, Err(_)));
    }
}
