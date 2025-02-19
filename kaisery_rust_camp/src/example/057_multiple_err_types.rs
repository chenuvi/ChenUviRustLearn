#[allow(unused_imports)]
use std::{error::Error, fs, num::ParseIntError};

enum ParseFileError {
    File,
    Parse(ParseIntError),
}

#[allow(dead_code, unused_variables)]
pub fn run() {
    let i = parse_file("example.txt");
    match i {
        Ok(i) => print!("{i}"),
        Err(e) => match e {
            ParseFileError::File => {}
            ParseFileError::Parse(e) => {}
        },
    }
}

#[allow(dead_code, unused_variables)]
fn parse_file(filename: &str) -> Result<i32, ParseFileError> {
    let s = fs::read_to_string(filename).map_err(|_e| ParseFileError::File)?;
    let i = s.parse().map_err(|e| ParseFileError::Parse(e))?;
    Ok(i)
}

// fn parse_file(filename: &str) -> Result<i32, Box<dyn Error>> {
//     let s = fs::read_to_string(filename)?;
//     let i = s.parse()?;
//     Ok(i)
// }
