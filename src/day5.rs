
use crate::utils;
use std::str::FromStr;
use anyhow::{Result, anyhow};

fn print_stacks(stacks: &Vec<Vec<char>>) {
    for stack in stacks {
        print!("{:?}\n", stack);
    }
}

pub fn star1(filename: &str) -> Result<usize> {
    let mut stacks = vec![
        vec!['N', 'S', 'D', 'C', 'V', 'Q', 'T'],
        vec!['M', 'F', 'V'],
        vec!['F', 'Q', 'W', 'D', 'P', 'N', 'H', 'M'],
        vec!['D', 'Q', 'R', 'T', 'F'],
        vec!['R', 'F', 'M', 'N', 'Q', 'H', 'V', 'B'],
        vec!['C', 'F', 'G', 'N', 'P', 'W', 'Q'],
        vec!['W', 'F', 'R', 'L', 'C', 'T'],
        vec!['T', 'Z', 'N', 'S'],
        vec!['M', 'S', 'D', 'J', 'R', 'Q', 'H', 'N'],
    ];
    
    let lines = utils::read_lines(filename).expect("failed to read lines from file");
    for raw_line in lines {
        let line = raw_line.expect("ooops bad line");
        let instructions: Vec<&str> = line.split_whitespace().collect();
        if instructions.len() != 6 {
            println!("Error, bad input string");
            return Err(anyhow!("bad input"));
        }
        let count = usize::from_str(instructions[1]).expect("invalid count");
        let from = usize::from_str(instructions[3]).expect("invalid from") - 1;
        let to = usize::from_str(instructions[5]).expect("invalid to") - 1;

        println!("Move {} from {} to {}", count, from, to);

        let mut temp = Vec::new();

        for _ in 0..count {
            temp.push(stacks[from].pop().expect("bad pop"));
        }

        for _ in 0..count {
            stacks[to].push(temp.pop().expect("bad temp pop"));
        }

        print_stacks(&stacks);
    }

    for mut stack in stacks {
        match stack.pop() {
            Some(c) => println!("{}", c),
            None => (),
        }
    }

    Ok(0)
}