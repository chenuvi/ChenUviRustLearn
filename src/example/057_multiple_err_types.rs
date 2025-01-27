use std::{error::Error, fs};

#[allow(dead_code, unused_variables)]
pub fn run() {
    let i = parse_file("example.txt");
}

fn parse_file(filename: &str) -> Result<i32, Box<dyn Error>> {
    let s = fs::read_to_string(filename)?;
    let i = s.parse()?;
    Ok(i)
}
