use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::fs;

fn main() {
    let bag_re = Regex::new(r"\w+ \w+ bags?").unwrap();

    let bag_map: HashMap<String, Vec<String>> = fs::read_to_string("input/input.txt")
        .expect("Failed to read file!")
        .lines()
        .map(|line| {
            let mut matches = Vec::new();

            for mat in bag_re.find_iter(line) {
                matches.push(mat.as_str());
            }

            let key = matches[0].to_string();
            let value = matches[1..].iter().map(|&e| e.to_string()).collect();

            (key, value)
        })
        .collect();

    let mut count = 0;
    for key in bag_map.keys() {
        if key != "shiny gold bags" && bfs(&bag_map, key) {
            count += 1;
        }
    }

    println!("part 1: {}", count);
}

// TODO: do I really need BFS/DFS here?
fn bfs(bag_map: &HashMap<String, Vec<String>>, root: &String) -> bool {
    let mut deque: VecDeque<String> = VecDeque::new();
    let mut discovered: HashMap<String, bool> = HashMap::new(); // I could use a HashSet but it would result in an O(log n) find (my assumption, atleast)
    discovered.insert(root.clone(), true);
    deque.push_back(root.clone());

    while !deque.is_empty() {
        if let Some(curr) = deque.pop_front() {
            if curr == "shiny gold bags" || curr == "shiny gold bag" {
                // TODO: I could memoize this
                return true;
            }

            if let Some(edges) = bag_map.get(&curr) {
                for edge in edges {
                    if let None = discovered.get(edge) {
                        discovered.insert(edge.clone(), true);
                        deque.push_back(edge.clone());
                    }
                }
            }
        }
    }

    false
}
