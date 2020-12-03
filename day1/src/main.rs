use std::fs;

fn main() {
    let num_vec: Vec<i32> = fs::read_to_string("input/input.txt")
        .expect("Failed to read file!")
        .lines()
        .map(|e| e.parse().expect("Failed to parse number to i32"))
        .collect();

    let pair_opt: Option<(i32, i32)> = find_2020_sum_2(&num_vec);

    if let Some((a, b)) = pair_opt {
        println!("part 1: {}", a * b);
    }

    let triple_opt: Option<(i32, i32, i32)> = find_2020_sum_3(&num_vec);

    if let Some((a, b, c)) = triple_opt {
        println!("part 2: {}", a * b * c);
    }
}

fn find_2020_sum_2(num_vec: &Vec<i32>) -> Option<(i32, i32)> {
    for i in 0..num_vec.len() {
        for j in i + 1..num_vec.len() {
            let a = num_vec[i];
            let b = num_vec[j];

            if a + b == 2020 {
                return Some((a, b));
            }
        }
    }

    None
}

fn find_2020_sum_3(num_vec: &Vec<i32>) -> Option<(i32, i32, i32)> {
    for i in 0..num_vec.len() {
        for j in i + 1..num_vec.len() {
            for k in j + 1..num_vec.len() {
                let a = num_vec[i];
                let b = num_vec[j];
                let c = num_vec[k];

                if a + b + c == 2020 {
                    return Some((a, b, c));
                }
            }
        }
    }

    None
}
