use std::env;

mod utils;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} INPUTFILE", args[0]);
        std::process::exit(1);
    }

    if let Ok(result) = day5::star1(&args[1]) {
        println!("day5 star1: {}", result)
    }
}
