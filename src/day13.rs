use crate::utils;
use serde_json::{Result as JSONResult, Value};
use anyhow::{Result, anyhow};

fn in_order(left: &Value, right: &Value) -> Option<bool> {
    if left.is_number() && right.is_number() {
        let l = left.as_u64().unwrap();
        let r = right.as_u64().unwrap();
        println!("checking two numbers: {} {}", l, r);

        if l < r {
            println!("left is smaller than right: {} {}", l, r);
            return Some(true);
        } else if l > r {
            println!("left is bigger than right: {} {}", l, r);
            return Some(false);
        } else {
            return None;
        }
    } else if left.is_array() && right.is_array() {
        println!("Checking two arrays");
        let l = left.as_array().unwrap();
        let r = right.as_array().unwrap();

        for i in 0..l.len(){
            if i == r.len() {
                println!("Right is smaller than left");
                return Some(false);
            }
            match in_order(&l[i], &r[i]) {
                Some(x) => return Some(x),
                None => continue,
            }
        }

        if l.len() < r.len() {
            println!("left side is smaller, so inputs are in the right order");
            return Some(true);
        }
        return None;
    } else if left.is_number() {
        let l_vec = vec![left.clone()];
        let l_val = Value::Array(l_vec);
        return in_order(&l_val, right);
    } else if right.is_number() {
        let r_vec = vec![right.clone()];
        let r_val = Value::Array(r_vec);
        return in_order(left, &r_val);
    }

    return None;
}

pub fn star1(filename: &str) -> Result<i32> {
    let lines = utils::read_lines(filename).expect("failed to read lines from file");
    let mut lineVec = Vec::new();
    let mut total = 0;

    for line in lines {
        let line = line.expect("bad line");
        lineVec.push(line);
    }

    for i in 0..(lineVec.len() / 3) {
        let v1: Value = serde_json::from_str(lineVec[i * 3].as_str())?;
        let v2: Value = serde_json::from_str(lineVec[i * 3 + 1].as_str())?;

        println!("checking order for: {}", i + 1);
        let order = in_order(&v1, &v2);
        if order.is_none() || (order.is_some() && order.expect("whut")) {
            println!("In order: {}", i + 1);
            total += i as i32 + 1;
        } else {
            println!("Not in order: {}", i + 1);
        }
    }

    return Ok(total);
}

pub fn star2(filename: &str) -> Result<i32> {
    let lines = utils::read_lines(filename).expect("failed to read lines from file");
    let mut lineVec: Vec<Value> = Vec::new();
    let mut total = 0;

    for line in lines {
        let line = line.expect("bad line");
        if line.len() < 2 {
            continue;
        }
        lineVec.push(serde_json::from_str(&line).unwrap());
    }

    lineVec.push(serde_json::from_str("[[2]]").unwrap());
    lineVec.push(serde_json::from_str("[[6]]").unwrap());


    lineVec.sort_by(|l, r| match in_order(l, r) {
        Some(true) => std::cmp::Ordering::Less,
        Some(false) => std::cmp::Ordering::Greater,
        None => std::cmp::Ordering::Equal,
    });

    for i in 0..lineVec.len() {
        let cur_line = lineVec[i].to_string();
        println!("{}", &cur_line);
        if lineVec[i].to_string() == "[[2]]" {
            println!("Found 2: {}", i + 1);
            total = i as i32 + 1;
        } else if lineVec[i].to_string() == "[[6]]" {
            println!("Found 6: {}", i + 1);
            total = total * (i as i32 + 1);
        }
    }
    return Ok(total);
}