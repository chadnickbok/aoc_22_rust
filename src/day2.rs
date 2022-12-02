use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

use crate::utils;

pub fn get_game_score(l: char, r: char) -> usize {
    let mut score = 0;

    if r == 'X' {
        score = 1;
    } else if r == 'Y' {
        score = 2;
    } else {
        score = 3;
    }

    // Rock
    if r == 'X' {
        if l == 'A' {
            score += 3; // Draw
        } else if l == 'C' {
            score += 6; // Rock beats scissors
        }
    }

    // Paper
    if r == 'Y' {
        if l == 'A' {
            score += 6; // Paper beats rock
        } else if l == 'B' {
            score += 3; // Draw
        }
    }

    // Scissors
    if r == 'Z' {
        if l == 'B' {
            score += 6; // Scissors beat paper
        } else if l == 'C' {
            score += 3; // Draw
        }
    }

    score
}

pub fn star1(filename: &str) -> usize {
    let lines = utils::read_lines(filename).expect("failed to read lines from file");
    let mut total = 0;

    for line in lines {
        if let Ok(l) = line {
            if l.len() != 3 {
                println!("Bad line: {}", l);
                break;
            }
            let lc = l.chars().nth(0).unwrap_or_default();
            let rc = l.chars().nth(2).unwrap_or_default();
            total += get_game_score(lc, rc);
        } else {
            println!("bad line");
        }
    }

    total
}