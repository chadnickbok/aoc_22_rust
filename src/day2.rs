use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

use crate::utils;

pub fn get_game_score_1(l: char, r: char) -> usize {
    let mut score = 0;

    match format!("{}{}", l, r).as_str() {
        "AX" => return 1 + 3, // Rock, Rock (Draw)m
        "AY" => return 2 + 6, // Rock, Paper (Win)
        "AZ" => return 3 + 0, // Rock, Scissors (Lose)
        "BX" => return 1 + 0, // Paper, Rock (Lose)
        "BY" => return 2 + 3, // Paper, Paper (Draw)
        "BZ" => return 3 + 6, // Paper, Scissors (Win)
        "CX" => return 1 + 6, // Scissors, Rock (Win)
        "CY" => return 2 + 0, // Scissors, Paper (Lose)
        "CZ" => return 3 + 3, // Scissors, Scissors (Draw)
        _ => return 0,
    }
}

pub fn get_game_score_2(l: char, r: char) -> usize {
    let mut score = 0;

    match format!("{}{}", l, r).as_str() {
        "AX" => return 3 + 0, // Rock, Scissors (Lose)
        "AY" => return 1 + 3, // Rock, Rock (Draw)
        "AZ" => return 2 + 6, // Rock, Paper (Win)
        "BX" => return 1 + 0, // Paper, Rock (Lose)
        "BY" => return 2 + 3, // Paper, Paper (Draw)
        "BZ" => return 3 + 6, // Paper, Scissors (Win)
        "CX" => return 2 + 0, // Scissors, Paper (Lose)
        "CY" => return 3 + 3, // Scissors, Scissors (Draw)
        "CZ" => return 1 + 6, // Scissors, Rock (Win)
        _ => return 0,
    }
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
            total += get_game_score_1(lc, rc);
        } else {
            println!("bad line");
        }
    }

    total
}

pub fn star2(filename: &str) -> usize {
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
            total += get_game_score_2(lc, rc);
        } else {
            println!("bad line");
        }
    }

    total
}