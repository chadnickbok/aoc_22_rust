use crate::utils;
use anyhow::Result;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::str::FromStr;

const MAX_I: i64 = 30;

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
