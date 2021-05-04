use std::fs;

fn main() {
    let file_string = fs::read_to_string("input/input.txt").expect("Failed to read file!");
    let instructions: Vec<&str> = file_string
        .lines()
        .collect();

    println!("part 1: {}", get_acc(&instructions)); // we're sure that there's an infinite loop on the first part

    if let Some(acc) = get_acc_fixed_opt(&instructions) {
        println!("part 2: {}", acc);
    }
}

fn get_acc_fixed_opt(instructions: &Vec<&str>) -> Option<i32> {
    for (idx, &ins) in instructions.iter().enumerate() {
        let main_split: Vec<&str> = ins.split(' ').collect();
        let opcode = main_split[0];
        
        if opcode == "jmp" {
            // change to `nop`
            let mut new_ins = "nop".to_owned();
            new_ins.push_str(" ");
            new_ins.push_str(main_split[1]);

            let mut new_instructions = instructions.clone(); // not sure if this is valid?
            new_instructions[idx] = new_ins.as_str();

            if !is_infinite_loop(&new_instructions) {
                return Some(get_acc(&new_instructions));
            }
        } else if opcode == "nop" {
            // change to `jmp`
            let mut new_ins = "jmp".to_owned();
            new_ins.push_str(" ");
            new_ins.push_str(main_split[1]);

            let mut new_instructions = instructions.clone(); // not sure if this is valid?
            new_instructions[idx] = new_ins.as_str();

            if !is_infinite_loop(&new_instructions) {
                return Some(get_acc(&new_instructions));
            }
        }
    }

    None
}

fn get_acc(instructions: &Vec<&str>) -> i32 {
    let mut acc = 0;
    let mut pc = 0; // program counter
    let mut curr_ins = instructions[pc];
    let mut ins_exec_vec: Vec<bool> = vec![false; instructions.len()];

    while pc < instructions.len() && !ins_exec_vec[pc] {
        let main_split: Vec<&str> = curr_ins.split(' ').collect();
        let opcode = main_split[0];
        let arg: i32 = main_split[1]
            .parse()
            .expect("Failed to parse number to i32!");

        ins_exec_vec[pc] = true;
        if opcode == "acc" {
            acc += arg;
            pc += 1;
        } else if opcode == "jmp" {
            if arg.is_negative() {
                pc -= arg.abs() as usize;
            } else {
                pc += arg as usize;
            }
        } else {
            // we can safely assume that this is "nop"
            pc += 1;
        }

        if pc < instructions.len() {
            curr_ins = instructions[pc];
        }
    }

    acc
}

fn is_infinite_loop(instructions: &Vec<&str>) -> bool {
    let mut pc = 0; // program counter
    let mut curr_ins = instructions[pc];
    let mut ins_exec_vec: Vec<bool> = vec![false; instructions.len()];

    while pc < instructions.len() && !ins_exec_vec[pc] {
        let main_split: Vec<&str> = curr_ins.split(' ').collect();
        let opcode = main_split[0];
        let arg: i32 = main_split[1]
            .parse()
            .expect("Failed to parse number to i32!");

        ins_exec_vec[pc] = true;
        if opcode == "acc" {
            pc += 1;
        } else if opcode == "jmp" {
            if arg.is_negative() {
                pc -= arg.abs() as usize;
            } else {
                pc += arg as usize;
            }
        } else {
            // we can safely assume that this is "nop"
            pc += 1;
        }

        if pc < instructions.len() {
            curr_ins = instructions[pc];
        }
    }

    pc < instructions.len()
}