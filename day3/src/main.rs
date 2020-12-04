use std::fs;

struct Point {
    i: usize,
    j: usize,
}

fn main() {
    let board: Vec<Vec<char>> = fs::read_to_string("input/input.txt")
        .expect("Failed to read file!")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let height = board.len();
    let width = board[0].len();

    let r1d1 = count_trees(&board, width, height, 1, 1);
    let r3d1 = count_trees(&board, width, height, 3, 1);
    let r5d1 = count_trees(&board, width, height, 5, 1);
    let r7d1 = count_trees(&board, width, height, 7, 1);
    let r1d2 = count_trees(&board, width, height, 1, 2);

    let product = r1d1 * r3d1 * r5d1 * r7d1 * r1d2;

    println!("part 1: {}", r3d1);
    println!("part 2: {}", product);
}

fn count_trees(
    board: &Vec<Vec<char>>,
    width: usize,
    height: usize,
    right_step: usize,
    down_step: usize,
) -> u64 {
    let mut curr_point = Point { i: 0, j: 0 };
    let mut trees_count = 0u64;

    while !at_end_row(&curr_point, height) {
        curr_point = perform_step(curr_point, width, right_step, down_step);

        if curr_point.i < height && board[curr_point.i][curr_point.j] == '#' {
            trees_count += 1;
        }
    }

    trees_count
}

fn at_end_row(point: &Point, height: usize) -> bool {
    point.i >= height - 1
}

fn perform_step(curr_point: Point, width: usize, right_step: usize, down_step: usize) -> Point {
    let new_i = curr_point.i + down_step;

    if curr_point.j + right_step < width {
        let new_j = curr_point.j + right_step;
        Point { i: new_i, j: new_j }
    } else {
        let new_j = curr_point.j + right_step - width;
        Point { i: new_i, j: new_j }
    }
}
