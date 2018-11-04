use std::fs;
use std::env;
use std::error::Error;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "cool";
        let contents = "\
Rust is
much cooler than
C++.";
        assert_eq!(
            vec!("much cooler than"),
            search(query, contents)
        );
    }
}

#[test]
fn case_insensitive() {
    let query = "CoOL";
    let contents = "\
Rust is
much cooler than
C++.";
    assert_eq!(
        vec!("much cooler than"),
        search_case_insensitive(query, contents)
    );
}

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get query string.")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get filename string.")
        };

        let case_sensitive = match args.next() {
            Some(i) => i == "--case-insensitive" || i == "-cs",
            None => env::var("CASE_INSENSITIVE").is_err()
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
   contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    
    contents.lines()
        .filter(|line| line.contains(&query))
        .collect()
}