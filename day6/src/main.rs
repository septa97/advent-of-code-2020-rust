use std::fs;

const CHAR_SET_SIZE: usize = 26; // 'a' to 'z'

// I'm not sure if tuple struct is the cleanest way to do this
struct Group(Vec<Vec<bool>>);

fn main() {
    let groups: Vec<Group> = fs::read_to_string("input/input.txt")
        .expect("Failed to read file!")
        .split("\n\n")
        .map(|group_str| {
            let group_str = group_str.trim();

            let answers: Vec<Vec<bool>> = group_str
                .split('\n')
                .map(|line| {
                    let mut v = vec![false; CHAR_SET_SIZE];

                    for b in line.as_bytes() {
                        // we're sure that the characters are 'a' to 'z' only
                        // we don't have to worry about multi-byte characters
                        let idx = b - b'a';

                        v[idx as usize] = true;
                    }

                    v
                })
                .collect();

            Group(answers)
        })
        .collect();

    let part1: usize = groups
        .iter()
        .map(|group| {
            let Group(answers) = group;
            let accumulated = answers.iter().fold(vec![false; CHAR_SET_SIZE], |acc, x| {
                acc.iter().zip(x.iter()).map(|(&a, &b)| a || b).collect()
            });

            accumulated.iter().filter(|&&e| e).count()
        })
        .sum();

    let part2: usize = groups
        .iter()
        .map(|group| {
            let Group(answers) = group;
            let accumulated = answers.iter().fold(vec![true; CHAR_SET_SIZE], |acc, x| {
                acc.iter().zip(x.iter()).map(|(&a, &b)| a && b).collect()
            });

            accumulated.iter().filter(|&&e| e).count()
        })
        .sum();

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}
