use crate::utils;
use std::str::FromStr;



fn move_tail(head_x: i32, head_y: i32, tail_x: i32, tail_y: i32) -> (i32, i32) {
    // Move Right
    if (head_y == tail_y) && (head_x > (tail_x + 1)) {
        return (tail_x + 1, tail_y);
    }

    // Move Left
    if (head_y == tail_y) && (head_x < (tail_x - 1)) {
        return (tail_x - 1, tail_y);
    }

    // Move Up
    if (head_y > (tail_y + 1)) && head_x == tail_x {
        return (tail_x, tail_y + 1);
    }

    // Move Down
    if (head_y < (tail_y - 1)) && head_x == tail_x {
        return (tail_x, tail_y - 1);
    }

    // Move Up and Right
    if (head_y > tail_y) && (head_x > tail_x) && ((head_y > (tail_y + 1)) || (head_x > (tail_x + 1))) {
        return (tail_x + 1, tail_y + 1);
    }

    // Move Up and Left
    if (head_y > tail_y) && (head_x < tail_x) && ((head_y > (tail_y + 1)) || (head_x < (tail_x - 1))) {
        return (tail_x - 1, tail_y + 1);
    }

    // Move Down and Right
    if (head_y < tail_y) && (head_x > tail_x) && ((head_y < (tail_y - 1)) || (head_x > (tail_x + 1))) {
        return (tail_x + 1, tail_y - 1);
    }

    // Move Down and Left
    if (head_y < tail_y) && (head_x < tail_x) && ((head_y < (tail_y - 1)) || (head_x < (tail_x - 1))) {
        return (tail_x - 1, tail_y - 1);
    }

    // No need to move
    (tail_x, tail_y)
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

    let mut tail_x = 512;
    let mut tail_y = 512;

    let mut head_x = 512;
    let mut head_y = 512;

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
            head_x += x_step;
            head_y += y_step;

            println!("{} {}", head_x, head_y);

            let (new_x, new_y) = move_tail(head_x, head_y, tail_x, tail_y);
            tail_x = new_x;
            tail_y = new_y;

            visited_grid[tail_y as usize][tail_x as usize] = true;
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