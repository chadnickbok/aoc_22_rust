use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} INPUTFILE", args[0]);
        std::process::exit(1);
    }

    let mut elves: Vec<Vec<usize>> = Vec::new();
    let mut elf = Vec::new();

    let lines = read_lines(&args[1]).expect("lol no way");

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
    let total: usize = iterator.sum();
    println!("{}", total);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}