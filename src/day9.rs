use crate::utils;
use std::str::FromStr;



fn move_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    // Move Right
    if (head.1 == tail.1) && (head.0 > (tail.0 + 1)) {
        return (tail.0 + 1, tail.1);
    }

    // Move Left
    if (head.1 == tail.1) && (head.0 < (tail.0 - 1)) {
        return (tail.0 - 1, tail.1);
    }

    // Move Up
    if (head.1 > (tail.1 + 1)) && head.0 == tail.0 {
        return (tail.0, tail.1 + 1);
    }

    // Move Down
    if (head.1 < (tail.1 - 1)) && head.0 == tail.0 {
        return (tail.0, tail.1 - 1);
    }

    // Move Up and Right
    if (head.1 > tail.1) && (head.0 > tail.0) && ((head.1 > (tail.1 + 1)) || (head.0 > (tail.0 + 1))) {
        return (tail.0 + 1, tail.1 + 1);
    }

    // Move Up and Left
    if (head.1 > tail.1) && (head.0 < tail.0) && ((head.1 > (tail.1 + 1)) || (head.0 < (tail.0 - 1))) {
        return (tail.0 - 1, tail.1 + 1);
    }

    // Move Down and Right
    if (head.1 < tail.1) && (head.0 > tail.0) && ((head.1 < (tail.1 - 1)) || (head.0 > (tail.0 + 1))) {
        return (tail.0 + 1, tail.1 - 1);
    }

    // Move Down and Left
    if (head.1 < tail.1) && (head.0 < tail.0) && ((head.1 < (tail.1 - 1)) || (head.0 < (tail.0 - 1))) {
        return (tail.0 - 1, tail.1 - 1);
    }

    // No need to move
    (tail.0, tail.1)
}


pub fn star1(filename: &str) -> Result<usize, utils::AocError> {
    let mut total = 0;
    let mut visited_grid:Vec<Vec<bool>> = Vec::new();
    
    // Setup an empty grid
    for j in 0..1024 {
        visited_grid.push(Vec::new());
        for i in 0..1024 {
            visited_grid[j].push(false);
        }
    }

    let mut tail = (512, 512);

    let mut head = (512, 512);

    let lines = utils::read_lines(filename).expect("failed to read lines from file");
    for line in lines {
        let line = line.expect("bad line");
        let cmd: Vec<&str> = line.split_whitespace().collect();
        let count = usize::from_str(cmd[1]).expect("bad count");

        let mut x_step = 0;
        let mut y_step = 0;
        match cmd[0] {
            "R" => x_step = 1,
            "U" => y_step = 1,
            "L" => x_step = -1,
            "D" => y_step = -1,
            _ => println!("Bad move!"),
        }

        for _ in 0..count {
            head.0 += x_step;
            head.1 += y_step;

            println!("{} {}", head.0, head.1);

            let (new_x, new_y) = move_tail(head,tail);
            tail.0 = new_x;
            tail.1 = new_y;

            visited_grid[tail.1 as usize][tail.0 as usize] = true;
        }
    }

    for j in 0..1024 {
        for i in 0..1024 {
            if visited_grid[j][i] {
                total += 1;
            }
        }
    }

    Ok(total)
}


pub fn star2(filename: &str) -> Result<usize, utils::AocError> {
    let mut total = 0;
    let mut visited_grid:Vec<Vec<bool>> = Vec::new();
    
    // Setup an empty grid
    for j in 0..1024 {
        visited_grid.push(Vec::new());
        for i in 0..1024 {
            visited_grid[j].push(false);
        }
    }

    let mut segments: Vec<(i32, i32)> = Vec::new();
    for _ in 0..10 {
        segments.push((512, 512));
    }

    let lines = utils::read_lines(filename).expect("failed to read lines from file");
    for line in lines {
        let line = line.expect("bad line");
        let cmd: Vec<&str> = line.split_whitespace().collect();
        let count = usize::from_str(cmd[1]).expect("bad count");

        let mut x_step = 0;
        let mut y_step = 0;
        match cmd[0] {
            "R" => x_step = 1,
            "U" => y_step = 1,
            "L" => x_step = -1,
            "D" => y_step = -1,
            _ => println!("Bad move!"),
        }

        for _ in 0..count {
            segments[0].0 += x_step;
            segments[0].1 += y_step;

            println!("{:?}", segments[0]);

            for i in 0..9 {
                let (new_x, new_y) = move_tail(segments[i], segments[i+1]);
                segments[i+1] = (new_x, new_y);
            }

            visited_grid[segments[9].1 as usize][segments[9].0 as usize] = true;
        }
    }

    for j in 0..1024 {
        for i in 0..1024 {
            if visited_grid[j][i] {
                total += 1;
            }
        }
    }

    Ok(total)
}