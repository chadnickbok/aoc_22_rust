use std::io::{self, BufRead};
use std::str::FromStr;

use crate::utils;

pub fn star1(filename: &str) -> usize {
    let mut elves: Vec<Vec<usize>> = Vec::new();
    let mut elf = Vec::new();

    let lines = utils::read_lines(filename).expect("failed to read lines from file");

    // Consumes the iterator, returns an (Optional) String
    for line in lines {
        if let Ok(raw_cals) = line {
            match (usize::from_str(&(raw_cals.trim()))) {
                (Ok(cals)) => {
                    elf.push(cals);
                }
                _ => {
                    elves.push(elf.to_owned());
                    elf.clear();
                }
            }
        }
    }
    elves.push(elf.to_owned());

    println!("There are {} elves", elves.len());

    let mut cals_vec: Vec<usize> = Vec::new();
    for elf in elves {
        cals_vec.push(elf.iter().sum());
    }
    cals_vec.sort();
    let len: usize = cals_vec.len();
    let top_three = &cals_vec[len-3..len];
    let iterator = top_three.iter();
    iterator.sum()
}

