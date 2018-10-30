use std::env;
mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, filename) = parse_args(&args);
   
    lib::read_file(filename);
}

fn parse_args(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}