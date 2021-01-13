use std::{env, str::FromStr};
use std::{fs, num::ParseIntError};

use itertools::Itertools;

type Int = i64;
type Memory = Vec<Int>;
type Address = usize;

#[derive(Clone)]
struct Program {
    instruction_pointer: Address,
    memory: Memory,
}

impl FromStr for Program {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let memory_result: Result<Vec<_>, _> =
            s.trim().split(',').map(&str::parse::<Int>).collect();

        Ok(Program {
            instruction_pointer: 0,
            memory: memory_result?,
        })
    }
}

fn apply_bin_op(op: fn(Int, Int) -> Int, program: &mut Program) {
    let address = program.instruction_pointer;
    let left_address = program.memory[address + 1] as Address;
    let right_address = program.memory[address + 2] as Address;
    let result_address = program.memory[address + 3] as Address;

    let left_value = program.memory[left_address];
    let right_value = program.memory[right_address];
    let result_value = &mut program.memory[result_address];

    *result_value = op(left_value, right_value);
    program.instruction_pointer += 4;
}

fn run_program(mut program: Program) -> Program {
    loop {
        match program.memory[program.instruction_pointer] {
            1 => apply_bin_op(|l, r| l + r, &mut program),
            2 => apply_bin_op(|l, r| l * r, &mut program),
            99 => break,
            _ => unreachable!(),
        };
    }
    program
}

fn solve_part_1(content: &str) {
    let mut program: Program = content.parse().unwrap();

    program.memory[1] = 12;
    program.memory[2] = 2;
    program = run_program(program);

    println!("Part 1: {}", program.memory[0])
}

fn solve_part_2(content: &str) {
    let program: Program = content.parse().unwrap();

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut program_clone = program.clone();

            program_clone.memory[1] = noun;
            program_clone.memory[2] = verb;
            let result = run_program(program_clone).memory[0];
            
            if result == 19690720 {
                println!("Part 2: {}", 100 * noun + verb)
            }
        }
    }
}

fn main() {
    let args = env::args().collect_vec();
    let filename = args.get(1).map(|s| s.as_ref()).unwrap_or("./res/input.txt");

    let content = fs::read_to_string(filename).unwrap();

    solve_part_1(&content);
    solve_part_2(&content);
}

#[cfg(test)]
mod tests {}
