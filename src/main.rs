use std::env;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} INPUTFILE", args[0]);
        std::process::exit(1);
    }

    if let Ok(result) = day16::calc_best_twin_path(&args[1]) {
        println!("day16 star1: {}", result)
    }
}
