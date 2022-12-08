use crate::utils;
use std::str::FromStr;


fn is_visible(trees: &Vec<Vec<usize>>, tree: (usize, usize)) -> bool {
    if tree.0 == 0 || tree.1 == 0 || tree.0 == trees[0].len() - 1 || tree.1 == trees.len() - 1 {
        return true;
    }

    let tree_height = trees[tree.1][tree.0];

    let mut visible_up = true;
    let mut visible_down = true;
    let mut visible_left = true;
    let mut visible_right = true;

    for y in 0..tree.1 {
        if trees[y][tree.0] >= tree_height {
            visible_up = false;
            break;
        }
    }

    for y in (tree.1+1)..trees.len() {
        if trees[y][tree.0] >= tree_height {
            visible_down = false;
            break;
        }
    }

    for x in 0..tree.0 {
        if trees[tree.1][x] >= tree_height {
            visible_left = false;
            break;
        }
    }

    for x in (tree.0+1)..trees[0].len() {
        if trees[tree.1][x] >= tree_height {
            visible_right = false;
            break;
        }
    }

    println!("{:?} {} {}", tree, tree_height, visible_up || visible_down || visible_left || visible_right);

    return visible_up || visible_down || visible_left || visible_right;
}

fn scenic_score(trees: &Vec<Vec<usize>>, tree: (usize, usize)) -> usize {
    if tree.0 == 0 || tree.1 == 0 || tree.0 == trees[0].len() - 1 || tree.1 == trees.len() - 1 {
        return 0;
    }

    let tree_height = trees[tree.1][tree.0];

    let mut score_up = 0;
    let mut score_down = 0;
    let mut score_left = 0;
    let mut score_right = 0;

    // Look up
    for y in 0..tree.1 {
        if trees[y][tree.0] >= tree_height {
            score_up = 1;
        } else {
            score_up += 1;
        }
    }

    // Look down
    for y in (tree.1+1)..trees.len() {
        score_down += 1;
        if trees[y][tree.0] >= tree_height {
            break;
        }
    }

    // Look left
    for x in 0..tree.0 {
        if trees[tree.1][x] >= tree_height {
            score_left = 1;
        } else {
            score_left += 1;
        }
    }

    // Look right
    for x in (tree.0+1)..trees[0].len() {
        score_right += 1;
        if trees[tree.1][x] >= tree_height {
            break;
        }
    }

    let score = score_up * score_down * score_left * score_right;
    println!("{:?} {} {}", tree, tree_height, score);
    println!("{} {} {} {}", score_up, score_down, score_left, score_right);

    return score;
}


pub fn star1(filename: &str) -> Result<usize, utils::AocError> {
    let mut trees: Vec<Vec<usize>> = Vec::new();
    let lines = utils::read_lines(filename).expect("failed to read lines from file");

    for raw_line in lines {
        let line = raw_line.expect("oops bad line");
        let srt_trees: Vec<usize> = line.chars().map(|x|  usize::from_str(x.to_string().as_str()).expect("bad int")).collect();
        trees.push(srt_trees);
    }

    let mut total_visible = 0;
    for x in 0..trees[0].len() {
        for y in 0..trees.len() {
            if is_visible(&trees, (x, y)) {
                total_visible += 1;
            }
        }
    }

    Ok(total_visible)
}

pub fn star2(filename: &str) -> Result<usize, utils::AocError> {
    let mut trees: Vec<Vec<usize>> = Vec::new();
    let lines = utils::read_lines(filename).expect("failed to read lines from file");

    for raw_line in lines {
        let line = raw_line.expect("oops bad line");
        let srt_trees: Vec<usize> = line.chars().map(|x|  usize::from_str(x.to_string().as_str()).expect("bad int")).collect();
        trees.push(srt_trees);
    }

    let mut max_score = 0;
    for x in 0..trees[0].len() {
        for y in 0..trees.len() {
            let cur_score = scenic_score(&trees, (x, y));
            if cur_score > max_score {
                max_score = cur_score;
            }
        }
    }

    Ok(max_score)
}