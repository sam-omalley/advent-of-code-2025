use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum Operations {
    None,
    Multiply,
    Add,
}

impl Operations {
    pub fn apply<I>(&self, iter: I) -> i64
    where
        I: IntoIterator<Item = i64>,
    {
        match self {
            Operations::Multiply => iter.into_iter().product(),
            Operations::Add => iter.into_iter().sum(),
            Operations::None => 0,
        }
    }
}

impl FromStr for Operations {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "*" => Ok(Operations::Multiply),
            "+" => Ok(Operations::Add),
            _ => Err(format!("Unknown operation: {}", s)),
        }
    }
}
