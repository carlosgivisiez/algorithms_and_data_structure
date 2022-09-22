use std::{error::{Error, self}, fmt};

mod algorithms;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.algorithm.to_lowercase().as_str() {
        "palindrome" => algorithms::palindrome::execute(),
        "caesar_cipher" => algorithms::caesar_cipher::execute(),
        "random_change" => algorithms::random_change::execute(),
        _ => Err(Box::new(AlgorithmUnknown))
    }
}

pub struct Config {
    pub algorithm: String
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 2 {
            return Err("Algorithm name not provided");
        }

        Ok(Config { algorithm: args[1].clone() })
    }
}

#[derive(Debug, Clone)]
struct AlgorithmUnknown;

impl fmt::Display for AlgorithmUnknown {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Algorithm name unknown")
    }
}

impl error::Error for AlgorithmUnknown {}
