use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main() {
    let bag_re = Regex::new(r"\w+ \w+ bags?").unwrap();

    let bag_map: HashMap<String, Vec<String>> = fs::read_to_string("input/input.txt")
        .expect("Failed to read file!")
        .lines()
        .map(|line| {
            let mut matches = Vec::new();

            for mat in bag_re.find_iter(line) {
                // TODO: I think using match groups is better? research on this more
                let match_str_vec: Vec<&str> = mat.as_str().split(' ').collect();
                let desc = format!("{} {}", match_str_vec[0], match_str_vec[1]);
                matches.push(desc);
            }

            let key = matches[0].clone(); // TODO: research on the difference of .clone() and .to_owned()
            let value: Vec<String> = matches[1..].to_vec();

            (key, value)
        })
        .collect();

    let mut count = 0;
    for key in bag_map.keys() {
        if key != "shiny gold" && find_recursively(&bag_map, key) {
            count += 1;
        }
    }

    println!("part 1: {}", count);
}

fn find_recursively(bag_map: &HashMap<String, Vec<String>>, curr: &String) -> bool {
    if curr == "shiny gold" {
        return true;
    } else if curr == "no other" {
        return false;
    }

    if let Some(edges) = bag_map.get(curr) {
        return edges
            .into_iter()
            .fold(false, |acc, edge| acc || find_recursively(bag_map, edge));
    } else {
        return false;
    }
}
