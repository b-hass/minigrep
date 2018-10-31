use std::fs;
use std::env;
use std::error::Error;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "bett";
        let contents = "\
Rust is
much better than
C++.";
        assert_eq!(
            vec!("much better than"),
            search(query, contents)
        );
    }
}

#[test]
fn case_insensitive() {
    let query = "BeTt";
    let contents = "\
Rust is
much better than
C++.";
    assert_eq!(
        vec!("much better than"),
        search_case_insensitive(query, contents)
    );
}

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        
        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = if args.get(3).is_some() {
            args[3] == "--case-insensitive" || args[3] == "-cs"
        } else {
            env::var("CASE_INSENSITIVE").is_err()
        };
        
        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;
    
    let results = if config.case_sensitive {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    
    for line in results {
        println!("{}", line);
    }
      
    Ok(())
}

fn search <'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}