use crate::utils;
use anyhow::Result;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::str::FromStr;

const MAX_I: i64 = 26;

#[derive(Clone, Debug)]
struct Node {
    value: i64,
    connections: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
enum Action {
    Open(String),
    Move((String, String)),
    None,
}

fn shortest_path(nodes: &HashMap<String, Node>, start: &str, end: &str) -> Option<i64> {
    let mut visited = HashMap::new();
    for node in nodes.keys() {
        visited.insert(node.clone(), false);
    }

    let mut queue = VecDeque::new();
    queue.push_back((start.to_string(), 0));

    while let Some((node, dist)) = queue.pop_front() {
        if node == end {
            return Some(dist);
        }
        if visited[&node] {
            continue;
        }
        visited.insert(node.clone(), true);
        for adjacent in &nodes[&node].connections {
            queue.push_back((adjacent.clone(), dist + 1));
        }
    }

    None
}

// Length between nodes, plus 1 each time to turn on the valve
// returns the length of the path, and the total expected value of the path
fn total_path(nodes: &HashMap<String, Node>, path: &Vec<&str>) -> (Option<i64>, i64) {
    let mut total_length = 0;
    let mut total_value = 0;
    for window in path.windows(2) {
        match shortest_path(nodes, &window[0], &window[1]) {
            Some(length) => {
                let cur_node = &nodes[window[1]];
                total_length += length + 1;
                total_value += (MAX_I - total_length) * cur_node.value;
            }
            None => return (None, 0),
        }
    }
    (Some(total_length), total_value)
}

fn dfs<'a>(
    nodes: &HashMap<String, Node>,
    path: &mut Vec<&'a str>,
    remaining: &mut Vec<&'a str>,
    best_value: &mut i64,
    best_path: &mut Vec<&'a str>,
) {
    // Calculate the total length and value of the current path
    let (length, value) = total_path(nodes, path);

    // If the path is too long, stop exploring this branch
    if let Some(length) = length {
        if length > MAX_I {
            return;
        }
    } else {
        return;
    }

    // If the value of the current path is the best so far, save it
    if value > *best_value {
        *best_value = value;
        let new_best_path = path.clone();
        *best_path = new_best_path;
        println!(
            "New best path found: {:?}, value: {}",
            best_path, best_value
        );
    }

    // Iterate over the remaining nodes to visit
    for i in 0..remaining.len() {
        // Move a node from `remaining` to `path`
        let node = remaining.remove(i);
        path.push(node.clone());

        // Recurse
        dfs(nodes, path, remaining, best_value, best_path);

        // Move the node back from `path` to `remaining`
        path.pop();
        remaining.insert(i, node);
    }
}

fn find_best_path<'a>(
    nodes: &HashMap<String, Node>,
    mut to_visit: Vec<&'a str>,
) -> (Vec<&'a str>, i64) {
    let mut best_path = Vec::new();
    let mut best_value = 0;
    let mut path = vec!["AA"];

    dfs(
        nodes,
        &mut path,
        &mut to_visit,
        &mut best_value,
        &mut best_path,
    );

    (best_path, best_value)
}

fn dfs_2<'a>(
    nodes: &HashMap<String, Node>,
    path1: &mut Vec<&'a str>,
    path2: &mut Vec<&'a str>,
    remaining: &mut Vec<&'a str>,
    best_value: &mut i64,
    best_paths: &mut (Vec<&'a str>, Vec<&'a str>),
) {
    // Calculate the total length and value of the current paths
    let (length1, value1) = total_path(nodes, path1);
    let (length2, value2) = total_path(nodes, path2);

    // If either path is too long, stop exploring this branch
    if let (Some(length1), Some(length2)) = (length1, length2) {
        if length1 > MAX_I || length2 > MAX_I {
            return;
        }
    } else {
        return;
    }

    // If the combined value of the current paths is the best so far, save it
    let combined_value = value1 + value2;
    if combined_value > *best_value {
        *best_value = combined_value;
        *best_paths = (path1.clone(), path2.clone());
        println!(
            "New best paths found: {:?} and {:?}, combined value: {}",
            path1, path2, combined_value
        );
    }

    // Iterate over the remaining nodes to visit
    for i in 0..remaining.len() {
        // Try adding the node to the shorter path
        if (path1.len() < path2.len()) {
            let node = remaining.remove(i);
            path1.push(node.clone());
            dfs_2(nodes, path1, path2, remaining, best_value, best_paths);
            path1.pop();

            // Try adding the node to the second path
            path2.push(node.clone());
            dfs_2(nodes, path1, path2, remaining, best_value, best_paths);
            path2.pop();

            remaining.insert(i, node);
        } else {
            let node = remaining.remove(i);
            // Try adding the node to the second path
            path2.push(node.clone());
            dfs_2(nodes, path1, path2, remaining, best_value, best_paths);
            path2.pop();

            path1.push(node.clone());
            dfs_2(nodes, path1, path2, remaining, best_value, best_paths);
            path1.pop();

            remaining.insert(i, node);
        }
    }
}

fn find_best_paths_2<'a>(
    nodes: &HashMap<String, Node>,
    mut to_visit: Vec<&'a str>,
) -> ((Vec<&'a str>, i64), (Vec<&'a str>, i64)) {
    let mut best_paths = (Vec::new(), Vec::new());
    let mut best_value = 0;
    let mut path1 = vec!["AA"];
    let mut path2 = vec!["AA"];

    dfs_2(
        nodes,
        &mut path1,
        &mut path2,
        &mut to_visit,
        &mut best_value,
        &mut best_paths,
    );

    let value1 = total_path(nodes, &best_paths.0).1;
    let value2 = total_path(nodes, &best_paths.1).1;

    ((best_paths.0, value1), (best_paths.1, value2))
}

fn bfs<'a>(
    nodes: &HashMap<String, Node>,
    to_visit: Vec<&'a str>,
    best_value: &mut i64,
    best_paths: &mut (Vec<&'a str>, Vec<&'a str>),
) {
    let max_depth = to_visit.len() + 1;

    for depth in 1..=max_depth {
        println!("Searching with depth limit {}", depth);
        let mut path1 = vec!["AA"];
        let mut path2 = vec!["AA"];
        let mut remaining = to_visit.clone();
        generate_paths(
            nodes,
            &mut path1,
            &mut path2,
            &mut remaining,
            best_value,
            best_paths,
            depth,
        );
    }

    let value1 = total_path(nodes, &best_paths.0).1;
    let value2 = total_path(nodes, &best_paths.1).1;

    println!(
        "Best paths found: {:?} and {:?}, combined value: {}",
        best_paths.0,
        best_paths.1,
        value1 + value2
    );
}

fn generate_paths<'a>(
    nodes: &HashMap<String, Node>,
    path1: &mut Vec<&'a str>,
    path2: &mut Vec<&'a str>,
    remaining: &mut Vec<&'a str>,
    best_value: &mut i64,
    best_paths: &mut (Vec<&'a str>, Vec<&'a str>),
    depth: usize,
) {
    if path1.len() == depth && path2.len() <= depth {
        update_best(nodes, path1, path2, best_value, best_paths);
    }

    // Iterate over the remaining nodes to visit
    for i in 0..remaining.len() {
        // Try adding the node to the first path
        let node = remaining.remove(i);
        path1.push(node);
        if path1.len() <= depth {
            generate_paths(
                nodes, path1, path2, remaining, best_value, best_paths, depth,
            );
        }
        path1.pop();

        // Try adding the node to the second path
        path2.push(node);
        if path2.len() <= depth {
            generate_paths(
                nodes, path1, path2, remaining, best_value, best_paths, depth,
            );
        }
        path2.pop();

        remaining.insert(i, node);
    }
}

fn update_best<'a>(
    nodes: &HashMap<String, Node>,
    path1: &mut Vec<&'a str>,
    path2: &mut Vec<&'a str>,
    best_value: &mut i64,
    best_paths: &mut (Vec<&'a str>, Vec<&'a str>),
) {
    let (length1, value1) = total_path(nodes, path1);
    let (length2, value2) = total_path(nodes, path2);

    if let (Some(length1), Some(length2)) = (length1, length2) {
        if length1 > MAX_I || length2 > MAX_I {
            return;
        }
    } else {
        return;
    }

    let combined_value = value1 + value2;
    if combined_value > *best_value {
        *best_value = combined_value;
        *best_paths = (path1.clone(), path2.clone());
        println!(
            "New best paths found: {:?} and {:?}, combined value: {}",
            path1, path2, combined_value
        );
    }
}

pub fn calc_star1(filename: &str) -> Result<i32> {
    let mut valves = HashMap::new();
    let lines = utils::read_lines(filename).expect("failed to read lines from file");
    for line in lines {
        let line = line.expect("bad line");
        let input: Vec<&str> = line.split_whitespace().collect();

        // "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB";

        let name = input[1].to_string();
        let rate_str = input[4].replace("rate=", "").replace(";", "");
        println!("rate: {}", rate_str);
        let rate = i64::from_str(rate_str.as_str()).expect("bad int");
        let mut connections = Vec::new();

        for i in 9..input.len() {
            connections.push(input[i].replace(",", ""));
        }
        valves.insert(
            name.clone(),
            Node {
                value: rate,
                connections,
            },
        );
    }

    let mut to_visit = vec![
        "OA", "QO", "UE", "JJ", "MI", "WG", "GY", "OQ", "NK", "RY", "JE", "JH", "EW", "MC", "GO",
    ];

    let (best_path, best_value) = find_best_path(&valves, to_visit);
    println!("Best Value: {} Best path: {:?}", best_value, best_path);

    return Ok(0);
}

pub fn calc_star2(filename: &str) -> Result<i32> {
    let mut valves = HashMap::new();
    let lines = utils::read_lines(filename).expect("failed to read lines from file");
    for line in lines {
        let line = line.expect("bad line");
        let input: Vec<&str> = line.split_whitespace().collect();

        // "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB";

        let name = input[1].to_string();
        let rate_str = input[4].replace("rate=", "").replace(";", "");
        println!("rate: {}", rate_str);
        let rate = i64::from_str(rate_str.as_str()).expect("bad int");
        let mut connections = Vec::new();

        for i in 9..input.len() {
            connections.push(input[i].replace(",", ""));
        }
        valves.insert(
            name.clone(),
            Node {
                value: rate,
                connections,
            },
        );
    }

    //let mut to_visit = vec![
    //    "OA", "QO", "UE", "JJ", "MI", "WG", "GY", "OQ", "NK", "RY", "JE", "JH", "EW", "MC", "GO",
    //];
    let mut to_visit = vec![
        "NK", "GY", "EW", "MI", "WG", "RY", "UE", "JJ", "OA", "MC", "JE", "OQ", "QO", "GO", "JH",
    ];

    //let mut best_value: i64 = 0;
    //let mut best_paths = (Vec::new(), Vec::new());
    //bfs(&valves, to_visit, &mut best_value, &mut best_paths);

    let ((best_path1, best_value1), (best_path2, best_value2)) =
        find_best_paths_2(&valves, to_visit);
    println!(
        "Best Value: {} Best path1: {:?} Best path2: {:?}",
        best_value1 + best_value2,
        best_path1,
        best_path2
    );

    return Ok(0);
}
