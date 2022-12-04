use std::env;

mod utils;
mod day1;
mod day2;
mod day3;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} INPUTFILE", args[0]);
        std::process::exit(1);
    }

    let result = day3::star1(&args[1]);
    println!("day3: {}", result)
}
