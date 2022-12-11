use crate::utils;
use std::str::FromStr;


pub fn star1(filename: &str) -> Result<i32, utils::AocError> {
    let mut total = 0;
    let mut vals = Vec::new();
    let mut x = 1;

    let lines = utils::read_lines(filename).expect("failed to read lines from file");
    vals.push(x);
    for line in lines {
        let line = line.expect("bad line");

        let cmd: Vec<&str> = line.split_whitespace().collect();
        match cmd[0] {
            "noop" => vals.push(x),
            "addx" => {
                let count = i32::from_str(cmd[1]).expect("bad count");

                vals.push(x);
                vals.push(x);
                x = x + count;
            },
            _ => println!("Bad CMD: {}", cmd[0]),
        }
    }

    println!("{} {} {}", vals[59], vals[60], vals[61]);




    total = 20 * vals[20] + 60 * vals[60] + 100 * vals[100] + 140 * vals[140] + 180 * vals[180] + 220 * vals[220];

    Ok(total)
}


pub fn star2(filename: &str) -> Result<i32, utils::AocError> {
    let mut total = 0;
    let mut vals = Vec::new();
    let mut x = 1;

    let lines = utils::read_lines(filename).expect("failed to read lines from file");
    for line in lines {
        let line = line.expect("bad line");

        let cmd: Vec<&str> = line.split_whitespace().collect();
        match cmd[0] {
            "noop" => vals.push(x),
            "addx" => {
                let count = i32::from_str(cmd[1]).expect("bad count");

                vals.push(x);
                vals.push(x);
                x = x + count;
            },
            _ => println!("Bad CMD: {}", cmd[0]),
        }
    }

    for line in 0..6 {
        let mut pixels = String::new();
        for x in 0..40 {
            let cycle = line * 40 + x;
            let val = vals[cycle];
            if x as i32 >= val - 1 && x as i32 <= val + 1 {
                pixels = pixels + "#";
            } else {
                pixels = pixels + ".";
            }
        }
        println!("{}", pixels);
    }




    println!("{} {} {}", vals[59], vals[60], vals[61]);


    

    total = 20 * vals[20] + 60 * vals[60] + 100 * vals[100] + 140 * vals[140] + 180 * vals[180] + 220 * vals[220];

    Ok(total)
}