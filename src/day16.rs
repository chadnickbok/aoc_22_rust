use crate::utils;
use anyhow::Result;
use std::collections::HashMap;
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

// Calculate and return the total 'value' accumulated by opening valves along
// the path represented by the action vector.
fn value_of_path(actions: &Vec<Action>, valves: &HashMap<String, Node>) -> i64 {
    let mut value = 0;
    let mut i = MAX_I;
    for action in actions {
        i = i - 1;
        match action {
            Action::Open(valve) => match valves.get(valve) {
                Some(v) => value = value + v.value * i,
                _ => (),
            },
            _ => (),
        }
    }

    value
}

// Check whether a specific node represented by the string has been visited
// in the path represented by the action vector.
fn is_visited(path: &Vec<Action>, n: &String) -> bool {
    for v in path {
        match v {
            Action::Open(s) => {
                if s == n {
                    return true;
                }
            }
            _ => {}
        }
    }
    return false;
}

fn path_search(
    count: i64,
    path: Vec<Action>,
    nodes: &HashMap<String, Node>,
    cur_node_name: &String,
) -> i64 {
    let c = count + 1;
    if c == MAX_I {
        let v = value_of_path(&path, nodes);
        return v;
    }

    let node = nodes.get(cur_node_name).expect("wrong node");

    // First, check if the current valve is open
    let mut max_flow = 0;
    if node.value > 0 && !is_visited(&path, &cur_node_name) {
        let mut p = path.clone();
        p.push(Action::Open(cur_node_name.clone()));
        max_flow = path_search(c, p, nodes, cur_node_name);
    }

    if count > 25 {
        let mut all_move = false;
        for i in path.len() - 10..path.len() {
            match &path[i] {
                Action::Open(_) => all_move = false,
                _ => all_move = true,
            }
        }

        if all_move {
            let mut p = path.clone();
            p.push(Action::None);
            let cur_flow = value_of_path(&p, nodes);
            if cur_flow > max_flow {
                return cur_flow;
            } else {
                return max_flow;
            }
        }
    }

    let mut prev_n = String::new();
    if path.len() > 1 {
        let prev = &path[path.len() - 1];
        match prev {
            Action::Move((a, _)) => prev_n = a.clone(),
            _ => (),
        }
    }

    for next in &node.connections {
        if next == &prev_n {
            // Can't move back to the previous valve
            continue;
        }
        let mut p = path.clone();
        p.push(Action::Move((cur_node_name.clone(), next.clone())));
        let cur_flow = path_search(c, p, nodes, &next);
        if cur_flow > max_flow {
            max_flow = cur_flow;
        }
    }

    if count == 2 {
        println!("{:?} {}", &path, max_flow);
    }

    max_flow
}

pub fn find_best_path(filename: &str) -> Result<i32> {
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

    let p = Vec::new();
    let start = "AA".to_string();
    let best_flow = path_search(0, p, &valves, &start);

    println!("Best Flow: {}", best_flow);

    println!("{:?}", valves);

    return Ok(0);
}
