use std::fs;

#[derive(Clone)]
struct Ins {
    opcode: String,
    arg: i32,
}

fn main() {
    let file_string = fs::read_to_string("input/input.txt").expect("Failed to read file!");
    let ins_vec: Vec<Ins> = file_string
        .lines()
        .map(|line| {
            let splits: Vec<&str> = line.split(' ').collect();
            let opcode = splits[0].to_string();
            let arg = splits[1].parse().expect("Failed to parse number to i32!");

            Ins {opcode, arg}
        })
        .collect();

    println!("part 1: {}", get_acc(&ins_vec)); // we're sure that there's an infinite loop on the first part

    if let Some(acc) = get_acc_fixed_opt(&ins_vec) {
        println!("part 2: {}", acc);
    }
}

fn get_acc_fixed_opt(ins_vec: &Vec<Ins>) -> Option<i32> {
    for (idx, ins) in ins_vec.iter().enumerate() {
        if ins.opcode == "jmp" {
            // change to `nop`
            let mut new_instructions = ins_vec.clone();
            new_instructions[idx] = Ins {
                opcode: String::from("nop"),
                arg: ins.arg
            };

            if !is_infinite_loop(&new_instructions) {
                return Some(get_acc(&new_instructions));
            }
        } else if ins.opcode == "nop" {
            // change to `jmp`
            let mut new_instructions = ins_vec.clone();
            new_instructions[idx] = Ins {
                opcode: String::from("jmp"),
                arg: ins.arg
            };

            if !is_infinite_loop(&new_instructions) {
                return Some(get_acc(&new_instructions));
            }
        }
    }

    None
}

fn get_acc(ins_vec: &Vec<Ins>) -> i32 {
    let mut acc = 0;
    let mut pc = 0; // program counter
    let mut curr_ins = &ins_vec[pc];
    let mut ins_exec_vec: Vec<bool> = vec![false; ins_vec.len()];

    while pc < ins_vec.len() && !ins_exec_vec[pc] {
        ins_exec_vec[pc] = true;
        if curr_ins.opcode == "acc" {
            acc += curr_ins.arg;
            pc += 1;
        } else if curr_ins.opcode == "jmp" {
            if curr_ins.arg.is_negative() {
                pc -= curr_ins.arg.abs() as usize;
            } else {
                pc += curr_ins.arg as usize;
            }
        } else {
            // we can safely assume that this is "nop"
            pc += 1;
        }

        if pc < ins_vec.len() {
            curr_ins = &ins_vec[pc];
        }
    }

    acc
}

fn is_infinite_loop(ins_vec: &Vec<Ins>) -> bool {
    let mut pc = 0; // program counter
    let mut curr_ins = &ins_vec[pc];
    let mut ins_exec_vec: Vec<bool> = vec![false; ins_vec.len()];

    while pc < ins_vec.len() && !ins_exec_vec[pc] {
        ins_exec_vec[pc] = true;
        if curr_ins.opcode == "acc" {
            pc += 1;
        } else if curr_ins.opcode == "jmp" {
            if curr_ins.arg.is_negative() {
                pc -= curr_ins.arg.abs() as usize;
            } else {
                pc += curr_ins.arg as usize;
            }
        } else {
            // we can safely assume that this is "nop"
            pc += 1;
        }

        if pc < ins_vec.len() {
            curr_ins = &ins_vec[pc];
        }
    }

    pc < ins_vec.len()
}