use crate::utils;
use anyhow::{Result, anyhow};
use std::str::FromStr;

fn parse_point(point: &str) -> (usize, usize) {
    let i = point.find(",").expect("invalid point");
    let x = usize::from_str(&point[..i]).expect("bad point int");
    let y = usize::from_str(&point[i+1..]).expect("bad point int");
    return (x, y);
}

pub fn star1(filename: &str) -> Result<i32> {
    let lines = utils::read_lines(filename).expect("failed to read lines from file");
    let mut total = 0;

    let mut grid: Vec<Vec<char>> = Vec::new();
    for y in 0..200 {
        let mut line = Vec::new();
        for x in 0..1024 {
            line.push('.');
        }
        grid.push(line);
    }

    let mut max_height = 0;

    for line in lines {
        let mut line = line.expect("failed to red line");
        line.retain(|c| !c.is_whitespace());
        
        let raw_points: Vec<&str> = line.split("->").collect();
        let mut l_point = parse_point(raw_points[0]);
        if l_point.1 > max_height {
            max_height = l_point.1;
        }
        let mut r_point = (0, 0);
        for i in 1..raw_points.len() {
            r_point = parse_point(raw_points[i]);
            if r_point.1 > max_height {
                max_height = r_point.1;
            }

            let mut x = l_point.0;
            let mut y = l_point.1;

            loop {
                if x == r_point.0 && y == r_point.1 {
                    break;
                }

                grid[y][x] = '#';
                if x < r_point.0 {
                    x = x + 1;
                } else if x > r_point.0 {
                    x = x - 1;
                }
                if y < r_point.1 {
                    y = y + 1;
                } else if y > r_point.1 {
                    y = y - 1;
                }
            }
            l_point = r_point;
        }
        grid[r_point.1][r_point.0] = '#';
    }

    for x in 0..1024 {
        grid[max_height + 2][x] = '#';
    }

    loop {
        let mut pos = (500, 0);
        loop {
            if grid[pos.1 + 1][pos.0] == '.' {
                pos.1 = pos.1 + 1;
                continue;
            }

            if grid[pos.1 + 1][pos.0 - 1] == '.' {
                pos.1 = pos.1 + 1;
                pos.0 = pos.0 - 1;
                continue;
            }

            if grid[pos.1 + 1][pos.0 + 1] == '.' {
                pos.1 = pos.1 + 1;
                pos.0 = pos.0 + 1;
                continue;
            }

            grid[pos.1][pos.0] = 'o';
            break;
        }

        total = total + 1;
        if grid[0][500] == 'o' {
            break;
        }
    }

    for line in &grid {
        let s: String = line[450..555].into_iter().collect();
        println!("{}", s);
    }

    Ok(total)
}