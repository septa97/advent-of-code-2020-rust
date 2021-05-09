use std::fs;

const PREAMBLE_LENGTH: usize = 25;

fn main() {
    let num_vec: Vec<u64> = fs::read_to_string("input/input.txt")
        .expect("Failed to read file!")
        .lines()
        .map(|e| e.parse().expect("Failed to parse number to u64"))
        .collect();

    if let Some(invalid) = find_first_invalid_opt(&num_vec) {
        println!("part 1: {}", invalid);

        let prefix_sum_vec = get_prefix_sum_vec(&num_vec);

        if let Some(encryption_weakness) =
            find_encryption_weakness_opt(&prefix_sum_vec, &num_vec, invalid)
        {
            println!("part 2: {}", encryption_weakness);
        }
    }
}

fn find_encryption_weakness_opt(
    prefix_sum_vec: &Vec<u64>,
    num_vec: &Vec<u64>,
    invalid: u64,
) -> Option<u64> {
    // we're sure that there's a min and max, therefore the contiguous range has atleast 2 elements
    for i in 1..prefix_sum_vec.len() {
        for j in 0..i {
            // check if valid contiguous
            if (j == 0 && prefix_sum_vec[i] == invalid)
                || (j > 0 && prefix_sum_vec[i] - prefix_sum_vec[j - 1] == invalid)
            {
                let contiguous_vec = Vec::from(&num_vec[j..i + 1]);
                let min = contiguous_vec.iter().min().unwrap();
                let max = contiguous_vec.iter().max().unwrap();

                return Some(min + max);
            }
        }
    }

    None
}

fn get_prefix_sum_vec(num_vec: &Vec<u64>) -> Vec<u64> {
    let mut prefix_sum_vec = Vec::new();
    prefix_sum_vec.push(num_vec[0]);

    for i in 1..num_vec.len() {
        prefix_sum_vec.push(num_vec[i] + prefix_sum_vec[i - 1]);
    }

    prefix_sum_vec
}

fn find_first_invalid_opt(num_vec: &Vec<u64>) -> Option<u64> {
    let mut curr_idx: usize = 0;
    let mut sum_vec = get_sum_vec(num_vec, curr_idx);

    for i in PREAMBLE_LENGTH..num_vec.len() {
        let curr = num_vec[i];

        if !sum_inside(&sum_vec, curr) {
            return Some(curr);
        }

        curr_idx += 1;
        sum_vec = get_sum_vec(num_vec, curr_idx);
    }

    None
}

// TODO: maybe we can do a zipped implementation for this?
fn get_sum_vec(num_vec: &Vec<u64>, start_idx: usize) -> Vec<u64> {
    let mut sum_vec = Vec::new();

    for i in start_idx..PREAMBLE_LENGTH + start_idx {
        for j in i + 1..PREAMBLE_LENGTH + start_idx {
            sum_vec.push(num_vec[i] + num_vec[j]);
        }
    }

    sum_vec
}

fn sum_inside(num_vec: &Vec<u64>, num: u64) -> bool {
    num_vec.iter().any(|&e| e == num)
}
