use crate::utils;
use std::collections::VecDeque;
use std::collections::HashSet;
use anyhow::{Result, anyhow};

fn is_unique(v: &VecDeque<char>) -> bool {
    let mut s = HashSet::new();
    for i in v {
        if !s.insert(i) {
            return false;
        }
    }
    true
}

pub fn star1(filename: &str) -> Result<usize> {
    let mut lines = utils::read_lines(filename).expect("failed to read lines from file");
    let raw_line = lines.nth(0).expect("bad input file");
    let line = raw_line.expect("bad line");

    if line.len() < 4 {
        return Err(anyhow!("bad line len"));
    }

    let mut i = 0;
    let mut prev_chars: VecDeque<char> = VecDeque::new();
    for c in line.chars() {
        i += 1;
        prev_chars.push_back(c);
        if prev_chars.len() < 14 {
            continue;
        }

        if is_unique(&prev_chars) {
            println!("Unique: {:?}", prev_chars);
            return Ok(i);
        }

        prev_chars.pop_front();
    }

    Err(anyhow!("bad input"))
}