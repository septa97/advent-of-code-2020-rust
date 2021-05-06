use regex::Regex;
use std::collections::HashMap;
use std::fs;

struct Row {
    min: u32,
    max: u32,
    c: char,
    s: String,
}

fn main() {
    let line_re = Regex::new(r"(\d{1,2})-(\d{1,2}) ([a-z]): ([a-z]+)").unwrap();

    let rows: Vec<Row> = fs::read_to_string("input/input.txt")
        .expect("Failed to read file!")
        .lines()
        .map(|line| {
            let cap = line_re.captures(line).unwrap();
            let min: u32 = cap[1].parse().expect("Failed to parse number to u32");
            let max: u32 = cap[2].parse().expect("Failed to parse number to u32");
            let c = cap[3].chars().nth(0).unwrap();
            let s = cap[4].to_string();

            Row { min, max, c, s }
        })
        .collect();

    let part1_count = rows
        .iter()
        .map(|row| {
            let mut count_map = HashMap::new();

            for ch in row.s.chars() {
                let count = count_map.entry(ch).or_insert(0u32);
                *count += 1;
            }

            let count = if let Some(count) = count_map.get(&row.c) {
                *count
            } else {
                0
            };

            count >= row.min && count <= row.max
        })
        .filter(|&e| e)
        .count();

    let part2_count = rows
        .iter()
        .map(|row| {
            let bytes = row.s.as_bytes();

            let left_idx = (row.min - 1) as usize;
            let right_idx = (row.max - 1) as usize;
            let left = bytes[left_idx] as char == row.c;
            let right = bytes[right_idx] as char == row.c;

            left ^ right
        })
        .filter(|&e| e)
        .count();

    println!("part 1: {}", part1_count);
    println!("part 2: {}", part2_count);
}
