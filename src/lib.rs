use std::fs;

pub fn read_file(filename: &str) {
    let contents = fs::read_to_string(&filename)
        .expect("Failed to read file.");

    println!("{}", contents);
}