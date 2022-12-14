use std::fs::File;
use std::io::{self, BufRead};
use thiserror::Error;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Error, Debug)]
pub enum AocError {
    #[error("a bad error")]
    AnError(String),
    #[error("an unknown err")]
    Unknown,
}

/*
impl AocError {
    pub fn from<T: std::error::Error>(t: T) -> AocError {
        AocError{ err: format!("{}", t) }
    }

    pub fn new<T: Into<String>>(t: T) -> AocError {
        AocError{ err: t.into() }
    }
}
*/