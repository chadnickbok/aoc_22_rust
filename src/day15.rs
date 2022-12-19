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

fn can_hold_beacon(pos: (i64, i64), beacons: &HashSet<(i64, i64)>, sensors: &Vec<Sensor>) -> bool {
    if beacons.contains(&pos) {
        return false;
    }

    for sensor in sensors.iter() {
        let cur_distance = distance(pos, sensor.pos);
        if cur_distance == 0 {
            return false;
        }

        if cur_distance <= sensor.distance {
            return false;
        }
    }

    return true;
}

const MAX_X: i64 = 4000000;
const MAX_Y: i64 = 4000000;

fn sensor_edges(s: &Sensor) -> Vec<(i64, i64)> {
    let mut edges = Vec::new();

    let mut x = s.pos.0 - (s.distance + 1) as i64;
    let mut ya = s.pos.1;
    let mut yb = s.pos.1;

    loop {
        if x >= 0 && x < MAX_X {
            if ya >= 0 && ya < MAX_Y {
                edges.push((x, ya));
            }
            if yb >= 0 && yb < MAX_Y {
                edges.push((x, yb));
            }
        }

        x = x + 1;
        if x > (s.pos.0 + (s.distance + 1) as i64) {
            break;
        }

        if x <= s.pos.0 {
            ya = ya + 1;
            yb = yb - 1;
        } else {
            ya = ya - 1;
            yb = yb + 1;
        }
    }

    return edges;
}

pub fn star2(filename: &str) -> Result<i64> {
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

    for s in &sensors {
        let edges = sensor_edges(s);
        for e in &edges {
            if can_hold_beacon(*e, &beacons, &sensors) {
                println!("found beacon at {} {} {}", e.0, e.1, e.0 * 4000000 + e.1);
                return Ok(e.0 * 4000000 + e.1);
            }
        }
    }

    println!("did not find beacon");

    return Ok(0);
}