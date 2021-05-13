use std::fs;

fn main() {
    let mut board: Vec<Vec<char>> = fs::read_to_string("input/input.txt")
        .expect("Failed to read file!")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    // print_board(&board);

    // nasty do-while loop here :DDDDDD
    while {
        let new_board = step(&board);

        let has_changed = has_changed(&board, &new_board);
        board = new_board;

        has_changed
    } {
        // print_board(&board);
    }

    let occupied_seats: usize = board
        .iter()
        .map(|line| line.iter().filter(|&&c| c == '#').count())
        .sum();

    println!("part 1: {}", occupied_seats);
}

// for print debugging
fn print_board(board: &Vec<Vec<char>>) {
    let rows = board.len();
    let cols = board[0].len();

    for i in 0..rows {
        for j in 0..cols {
            print!("{}", board[i][j]);
        }
        println!("");
    }
    println!("");
}

fn step(board: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_board = board.clone();
    let rows = board.len();
    let cols = board[0].len();

    for i in 0..rows {
        for j in 0..cols {
            let adj_occupied_count = count_occupied(board, i, j, rows, cols);

            if board[i][j] == 'L' {
                if adj_occupied_count == 0 {
                    new_board[i][j] = '#';
                }
            } else if board[i][j] == '#' {
                if adj_occupied_count >= 4 {
                    new_board[i][j] = 'L';
                }
            }
        }
    }

    return new_board;
}

fn count_occupied(board: &Vec<Vec<char>>, i: usize, j: usize, rows: usize, cols: usize) -> usize {
    let mut total: usize = 0;

    if i > 0 && j > 0 && board[i - 1][j - 1] == '#' {
        total += 1;
    }
    if j > 0 && board[i][j - 1] == '#' {
        total += 1;
    }
    if i > 0 && board[i - 1][j] == '#' {
        total += 1;
    }
    if i > 0 && j + 1 < cols && board[i - 1][j + 1] == '#' {
        total += 1;
    }
    if j + 1 < cols && board[i][j + 1] == '#' {
        total += 1;
    }
    if i + 1 < rows && j > 0 && board[i + 1][j - 1] == '#' {
        total += 1;
    }
    if i + 1 < rows && board[i + 1][j] == '#' {
        total += 1;
    }
    if i + 1 < rows && j + 1 < cols && board[i + 1][j + 1] == '#' {
        total += 1;
    }

    return total;
}

fn has_changed(old_board: &Vec<Vec<char>>, new_board: &Vec<Vec<char>>) -> bool {
    return !old_board
        .iter()
        .zip(new_board.iter())
        .all(|(a, b)| a.iter().zip(b.iter()).all(|(&x, &y)| x == y));
}
