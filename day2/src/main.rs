use std::fs;
use std::collections::HashMap;

struct Row {
    min: u32,
    max: u32,
    c: char,
    s: String,
}

fn main() {
    let rows: Vec<Row> = fs::read_to_string("input/input.txt")
        .expect("Failed to read file!")
        .lines()
        .map(|line | {
            // TODO: uhhhhh, too much .split here, not sure if there's a better way
            // maybe deconstructors? (if possible)
            let main_split: Vec<&str> = line.split(':').collect();
            let left = main_split[0];
            let right = main_split[1];
            let s  = right.trim().to_string();

            let left_split: Vec<&str> = left.split(' ').collect();
            let c = left_split[1].as_bytes()[0] as char; // we're sure that it's ASCII only and that it's only a single character
            let min_max_split: Vec<&str> = left_split[0].split('-').collect();
            let min: u32 = min_max_split[0].parse().expect("Failed to parse number to u32");
            let max: u32 = min_max_split[1].parse().expect("Failed to parse number to u32");

            Row {min, max, c, s }
        }).collect();

    let part1_count = rows.iter().map(|row | {
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
    }).filter(|&e| e).count();

    let part2_count = rows.iter().map(|row| {
        let bytes = row.s.as_bytes();

        let left_idx = (row.min-1) as usize;
        let right_idx = (row.max-1) as usize;
        let left = bytes[left_idx] as char == row.c;
        let right = bytes[right_idx] as char == row.c;

        left ^ right
    }).filter(|&e| e).count();
    
    println!("part 1: {}", part1_count);
    println!("part 2: {}", part2_count);
}
