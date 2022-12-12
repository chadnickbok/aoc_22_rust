use crate::utils;
use std::collections::HashSet;
use std::collections::VecDeque;

fn canVisit(a: char, b: char) -> bool {
    let i = a as u32 >= (b as u32 - 1);
    println!("{} {} {}", a, b, i);
    i
}

fn findSteps(start: (usize, usize), end: (usize, usize), heightMap: &Vec<Vec<char>>) -> i32 {
    let mut visited = HashSet::new();
    let mut posQ: VecDeque<((usize, usize), i32)> = VecDeque::new();
    posQ.push_back((start, 0));

    loop {
        let front = posQ.pop_front();
        let curPath: ((usize, usize), i32);

        match front {
            Some(x) => curPath = x,
            None => return 600,
        }

        println!("Cur Path: {:?}", curPath);

        if curPath.0 == end {
            println!("Reached the end in {} moves", curPath.1);
            return curPath.1;
        }

        if !visited.insert(curPath.0) {
            continue;
        }

        let curHeight = heightMap[curPath.0.1][curPath.0.0];

        // Up
        if curPath.0.1 > 0 {
            let up = heightMap[curPath.0.1 - 1][curPath.0.0];
            if canVisit(curHeight, up) {
                posQ.push_back(((curPath.0.0, curPath.0.1 - 1), curPath.1 + 1));
            }
        }
        // Down
        if curPath.0.1 < heightMap.len() - 1 {
            let down = heightMap[curPath.0.1 + 1][curPath.0.0];
            if canVisit(curHeight, down) {
                posQ.push_back(((curPath.0.0, curPath.0.1 + 1), curPath.1 + 1));
            }
        }

        // Left
        if curPath.0.0 > 0 {
            let left = heightMap[curPath.0.1][curPath.0.0 - 1];

            if canVisit(curHeight, left) {
                posQ.push_back(((curPath.0.0 - 1, curPath.0.1), curPath.1 + 1));
            }
        }

        // Right
        if curPath.0.0 < heightMap[0].len() - 1 {
            let right = heightMap[curPath.0.1][curPath.0.0 + 1];

            if canVisit(curHeight, right) {
                posQ.push_back(((curPath.0.0 + 1, curPath.0.1), curPath.1 + 1));
            }
        }
    }
}

pub fn star1(filename: &str) -> Result<usize, utils::AocError> {
    let lines = utils::read_lines(filename).expect("failed to read lines from file");
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);

    let mut heightMap: Vec<Vec<char>> = Vec::new();
    for raw_line in lines {
        let line = raw_line.expect("oops bad line");
        let mut heights: Vec<char> = line.chars().collect();
        heightMap.push(heights);
    }

    for y in 0..heightMap.len() {
        for x in 0..heightMap[0].len() {
            if heightMap[y][x] == 'S' {
                start = (x, y);
                heightMap[y][x] = 'a';
            } else if heightMap[y][x] == 'E' {
                end = (x, y);
                heightMap[y][x] = 'z';
            }
        }
    }


    let mut minPath = 520;

    for y in 0..heightMap.len() {
        for x in 0..heightMap[0].len() {
            if heightMap[y][x] != 'a' {
                continue;
            }
            let pathLen = findSteps((x, y), end, &heightMap);
            if pathLen < minPath {
                minPath = pathLen;
            }
        }
    }

    println!("Smallest path {}", minPath);

    Ok(0)
}