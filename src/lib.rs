use std::fs;
use std::error::Error;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "bett";
        let contents = "\
Rust is
much better than
C++.";
        assert_eq!(
            vec!("much", "better", "than"),
            search(query, contents)
        );
    }
}

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
       let contents = fs::read_to_string(&config.filename)?;

       Ok(())
}

fn search <'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}