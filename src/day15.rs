use crate::utils;
use anyhow::{Result, anyhow};
use std::str::FromStr;
use std::cmp;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
struct Sensor {
    pos: (i64, i64),
    distance: usize,
}

fn distance(a: (i64, i64), b: (i64, i64)) -> usize {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as usize
}

pub fn star1(filename: &str) -> Result<i32> {
    let lines = utils::read_lines(filename).expect("failed to read lines from file");


    let mut beacons = HashSet::new();

    let mut sensors = Vec::new();
    for line in lines {
        let line = line.expect("bad line");
        let input: Vec<&str> = line.split_whitespace().collect();
        
        let sx_str = input[2].replace("x=", "").replace(",", "");
        let sX = i64::from_str(&sx_str).expect("bad int");
        
        let sy_str = input[3].replace("y=", "").replace(":", "");
        let sY = i64::from_str(&sy_str).expect("bad int");
        
        let bx_str = input[8].replace("x=", "").replace(",", "");
        let bX = i64::from_str(&bx_str).expect("bad int");
        
        let by_str = input[9].replace("y=", "");
        let bY = i64::from_str(&by_str).expect("bad int");

        let d = distance((sX, sY), (bX, bY));

        sensors.push(Sensor{pos: (sX, sY), distance: d});
        beacons.insert((bX, bY));
    }

    let mut total = 0;
    'outer: for i in 0..11000000 {
        let mut can_hold_beacon = true;
        let cur_pos = (i - 4000000, 2000000);
        if beacons.contains(&cur_pos) {
            continue;
        }

        for sensor in sensors.iter() {
            let cur_distance = distance(cur_pos, sensor.pos);
            if cur_distance == 0 {
                println!("found beacon");
                continue 'outer;
            }
            if cur_distance <= sensor.distance {
                can_hold_beacon = false;
                break;
            }
        }
    
        if !can_hold_beacon {
            total = total + 1;
        }
    }

    return Ok(total);
}