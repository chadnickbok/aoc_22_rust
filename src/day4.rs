use crate::utils;
use std::str::FromStr;
use std::error::Error as StdError;

struct Range {
    l: usize,
    r: usize,
}

impl Range {
    fn new(raw_range: &str) -> Result<Range, utils::AocError> {
        let i = raw_range.find("-").expect("Invalid range");

        match (usize::from_str(&raw_range[..i]), usize::from_str(&raw_range[i + 1..])) {
            (Ok(l), Ok(r)) => return Ok(Range{l, r}),
            _ => Err(utils::AocError::new("bad string")),
        }
    }

    fn contains(&self, r2: &Range) -> bool {
        if self.l <= r2.l && self.r >= r2.r {
            return true;
        }
        false
    }

    fn overlaps(&self, r2: &Range) -> bool {
        if self.r < r2.l {
            return false;
        }

        if self.l > r2.r {
            return false;
        }

        return true;
    }
}


pub fn star1(filename: &str) -> Result<usize, utils::AocError> {
    let lines = utils::read_lines(filename).expect("failed to read lines from file");
    let mut total = 0;

    for raw_line in lines {
        let line = raw_line.expect("failed to red line");
        let i = line.find(",").expect("invalid range pair");
        let r1 = Range::new(&line[..i]).expect("bad range 1");
        let r2 = Range::new(&line[i+1..]).expect("bad range 2");

        if r1.contains(&r2) || r2.contains(&r1) {
            total += 1;
        }
    }

    Ok(total)
}

pub fn star2(filename: &str) -> Result<usize, utils::AocError> {
    let lines = utils::read_lines(filename).expect("failed to read lines from file");
    let mut total = 0;

    for raw_line in lines {
        let line = raw_line.expect("failed to red line");
        let i = line.find(",").expect("invalid range pair");
        let r1 = Range::new(&line[..i]).expect("bad range 1");
        let r2 = Range::new(&line[i+1..]).expect("bad range 2");

        if r1.overlaps(&r2) {
            total += 1;
        }
    }

    Ok(total)
}