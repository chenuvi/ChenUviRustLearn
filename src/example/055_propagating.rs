use std::fs::File;
use std::io::{Error, Read};

#[allow(unused_variables, dead_code)]
pub fn run() {
    let contents = read_file("example.txt");
}

fn read_file(filename: &str) -> Result<String, Error> {
    let mut contents = String::new();
    File::open(filename)?.read_to_string(&mut contents)?;
    Ok(contents)
}
