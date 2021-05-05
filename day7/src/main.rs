use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main() {
    let bag_re = Regex::new(r"(\w+ \w+) bags").unwrap();
    let bag_count_re = Regex::new(r"([1-9] )?(\w+) (\w+) bags?").unwrap();
    let no_bag_re = Regex::new(r"no other bags").unwrap();

    let bag_map: HashMap<String, Vec<(String, usize)>> = fs::read_to_string("input/input.txt")
        .expect("Failed to read file!")
        .lines()
        .filter(|&line| no_bag_re.find(line).is_none())
        .map(|line| {
            let mut values = Vec::new();

            // .unwrap() cause we're sure that there's a match (or else, there's a bug in the code)
            let key_match_str = bag_re.find(line).unwrap().as_str();
            let key = key_match_str[..key_match_str.len() - 5].to_string();

            let mut cap_iter = bag_count_re.captures_iter(line);
            cap_iter.next(); // to remove the first match, which is the `key` (not sure if there's a cleaner way)

            for cap in cap_iter {
                let color = format!("{} {}", &cap[2], &cap[3]);
                let count: usize = cap[1].trim().parse().expect("Failed to parse to usize!"); // not reference for string on stack (&cap[1]) ???
                values.push((color, count));
            }

            (key, values)
        })
        .collect();

    println!("part 1: {}", part1(&bag_map));
    println!("part 2: {}", part2(&bag_map));
}

fn part1(bag_map: &HashMap<String, Vec<(String, usize)>>) -> usize {
    return bag_map
        .keys()
        .filter(|&key| key != "shiny gold" && count_shiny_gold(bag_map, key))
        .count();
}

fn count_shiny_gold(bag_map: &HashMap<String, Vec<(String, usize)>>, curr: &String) -> bool {
    if curr == "shiny gold" {
        return true;
    }

    if let Some(edges) = bag_map.get(curr) {
        return edges.into_iter().fold(false, |acc, (edge, _)| {
            acc || count_shiny_gold(bag_map, edge)
        });
    } else {
        return false;
    }
}

fn part2(bag_map: &HashMap<String, Vec<(String, usize)>>) -> usize {
    return count_inside_shiny_gold(bag_map, &String::from("shiny gold"));
}

fn count_inside_shiny_gold(
    bag_map: &HashMap<String, Vec<(String, usize)>>,
    curr: &String,
) -> usize {
    if let Some(value) = bag_map.get(curr) {
        return value.into_iter().fold(0, |acc, (color, count)| {
            acc + count + count * count_inside_shiny_gold(bag_map, color)
        });
    } else {
        return 0;
    }
}
