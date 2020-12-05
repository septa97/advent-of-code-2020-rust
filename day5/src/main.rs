use std::fs;

fn main() {
    let mut seat_ids: Vec<u32> = fs::read_to_string("input/input.txt")
        .expect("Failed to read file!")
        .lines()
        .map(get_seat_id)
        .collect();

    println!("part 1: {}", seat_ids.iter().max().unwrap());

    seat_ids.sort();

    if let Some(your_id) = get_your_id(&seat_ids) {
        println!("part 2: {}", your_id);
    }
}

// had to do this because the wording is kinda vague for me
// it's stated that "some of the seats at the very front and back... don't exist"
fn get_your_id(seat_ids_sorted: &Vec<u32>) -> Option<u32> {
    for i in 0..seat_ids_sorted.len() - 1 {
        let left = seat_ids_sorted[i];
        let right = seat_ids_sorted[i + 1];

        if left + 2 == right {
            return Some(left + 1);
        }
    }

    None
}

fn get_seat_id(boarding_pass: &str) -> u32 {
    let mut i_low = 0u32;
    let mut i_high = 127u32;
    let mut j_low = 0u32;
    let mut j_high = 7u32;

    for c in boarding_pass.chars() {
        if c == 'F' || c == 'B' {
            let i_mid = (i_low + i_high) / 2;

            if c == 'F' {
                i_high = i_mid;
            } else {
                // we're guaranteed that it's 'B'
                i_low = i_mid + 1;
            }
        } else {
            // we're guaranteed that it's either 'L' or 'R'
            let j_mid = (j_low + j_high) / 2;

            if c == 'L' {
                j_high = j_mid;
            } else {
                // we're guaranteed that it's 'R'
                j_low = j_mid + 1;
            }
        }
    }

    // you could use either `i_low` or `i_high` (since they will eventually be the same)
    // also, you could use either `j_low` or `j_high` (since they will eventually be the same too)
    i_low * 8 + j_low
}
