use std::env;

mod utils;
mod day1;
mod day2;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} INPUTFILE", args[0]);
        std::process::exit(1);
    }

    let result = day2::star1(&args[1]);
    println!("day2: {}", result)
}
