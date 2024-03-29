use crate::utils;
use anyhow::Result;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::str::FromStr;

const MAX_I: i64 = 26;

#[derive(Clone, Debug)]
struct Node {
    value: i64,
    connections: Vec<String>,
}

pub struct ShortestPathCache<'a> {
    nodes: &'a HashMap<String, Node>,
    cache: RefCell<HashMap<(String, String), i64>>,
    scores: RefCell<HashMap<Vec<&'a str>, (Option<i64>, i64)>>,
}

impl<'a> ShortestPathCache<'a> {
    fn new(nodes: &'a HashMap<String, Node>) -> Self {
        ShortestPathCache {
            nodes,
            cache: RefCell::new(HashMap::new()),
            scores: RefCell::new(HashMap::new()),
        }
    }

    fn shortest_path(&self, start: &str, end: &str) -> i64 {
        let key = if start < end {
            (start.to_string(), end.to_string())
        } else {
            (end.to_string(), start.to_string())
        };

        // Check if value is in cache
        let val = {
            let c = self.cache.borrow();
            let k = c.get(&key);
            match k {
                Some(x) => *x,
                None => 0,
            }
        };

        if val > 0 {
            return val;
        }

        let length = self.calculate_shortest_path(start, end);
        self.cache.borrow_mut().insert(key, length);
        return length;
    }

    fn calculate_shortest_path(&self, start: &str, end: &str) -> i64 {
        let mut visited = HashMap::new();
        for node in self.nodes.keys() {
            visited.insert(node.as_str(), false);
        }

        let mut queue = VecDeque::new();
        queue.push_back((start, 0));

        while let Some((node, dist)) = queue.pop_front() {
            if node == end {
                return dist;
            }
            if visited[&node] {
                continue;
            }
            visited.insert(&node, true);
            for adjacent in &self.nodes[node].connections {
                queue.push_back((adjacent.as_str(), dist + 1));
            }
        }

        0 // Default to dummy value
    }

    fn total_path(&self, path: &Vec<&'a str>) -> (Option<i64>, i64) {
        let val = {
            match self.scores.borrow().get(path) {
                Some(x) => *x,
                None => (None, 0),
            }
        };
        if val.1 > 0 {
            return val;
        }

        let mut total_length = 0;
        let mut total_value = 0;
        for window in path.windows(2) {
            let length = self.shortest_path(&window[0], &window[1]);
            let cur_node = &self.nodes[window[1]];
            total_length += length + 1;
            total_value += (MAX_I - total_length) * cur_node.value;
        }

        let key = path.clone();
        self.scores
            .borrow_mut()
            .insert(key, (Some(total_length), total_value));

        (Some(total_length), total_value)
    }
}

fn dfs<'a>(
    nodes: &HashMap<String, Node>,
    cache: &mut ShortestPathCache<'a>,
    path1: &mut Vec<&'a str>,
    path2: &mut Vec<&'a str>,
    remaining: &mut Vec<&'a str>,
    best_value: &mut i64,
    best_paths: &mut (Vec<&'a str>, Vec<&'a str>),
) {
    let (length1, value1) = { cache.total_path(path1) };
    let (length2, value2) = { cache.total_path(path2) };

    let mut total_len = 0;

    // If either path is too long, stop exploring this branch
    if let (Some(length1), Some(length2)) = (length1, length2) {
        if length1 > MAX_I || length2 > MAX_I {
            return;
        }
        total_len = length1 + length2;
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

    if (total_len > 10) && (combined_value < 1000) {
        return;
    }

    if (total_len > 20) && (combined_value < 1500) {
        return;
    }

    if (total_len > 30) && (combined_value < 2000) {
        return;
    }

    // Iterate over the remaining nodes to visit
    for i in 0..remaining.len() {
        // Try adding the node to the shorter path
        let node = remaining.remove(i);
        path1.push(node);
        dfs(
            nodes, cache, path1, path2, remaining, best_value, best_paths,
        );
        path1.pop();

        // Try adding the node to the second path
        path2.push(node);
        dfs(
            nodes, cache, path1, path2, remaining, best_value, best_paths,
        );
        path2.pop();

        remaining.insert(i, node);
    }
}

fn find_best_paths<'a>(
    mut cache: &mut ShortestPathCache<'a>,
    nodes: &'a HashMap<String, Node>,
    mut to_visit: Vec<&'a str>,
) -> ((Vec<&'a str>, i64), (Vec<&'a str>, i64)) {
    let mut best_paths = (Vec::new(), Vec::new());
    let mut best_value = 0;
    let mut path1 = vec!["AA"];
    let mut path2 = vec!["AA"];

    dfs(
        nodes,
        &mut cache,
        &mut path1,
        &mut path2,
        &mut to_visit,
        &mut best_value,
        &mut best_paths,
    );

    let value1 = cache.total_path(&best_paths.0).1;
    let value2 = cache.total_path(&best_paths.1).1;

    ((best_paths.0, value1), (best_paths.1, value2))
}

pub fn calc_best_twin_path(filename: &str) -> Result<i32> {
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
            let name = input[i].replace(",", "");
            let v = connections.push(name);
        }
        valves.insert(
            name,
            Node {
                value: rate,
                connections,
            },
        );
    }

    let mut valid_valves = vec![
        "NK", "GY", "EW", "MI", "WG", "RY", "UE", "JJ", "OA", "MC", "JE", "OQ", "QO", "GO", "JH",
    ];

    let mut cache = ShortestPathCache::new(&valves);

    for i in 1..valid_valves.len() {
        let to_visit = valid_valves[0..i].to_vec();
        let ((best_path1, best_value1), (best_path2, best_value2)) =
            find_best_paths(&mut cache, &valves, to_visit);
        println!(
            "Best Value: {} Best path1: {:?} Best path2: {:?}",
            best_value1 + best_value2,
            best_path1,
            best_path2
        );
    }

    return Ok(0);
}
