#![feature(str_split_once)]

use std::env;
use std::fs;

type Argument = i64;

#[derive(Debug)]
enum Instruction {
    Nop(Argument),
    Acc(Argument),
    Jmp(Argument),
}

type Instructions = Vec<Instruction>;

#[derive(Debug, Clone)]
struct Program {
    next_instruction_index: usize,
    accumulation: Argument,
    executed_instructions: Vec<bool>,
}

impl Program {
    fn new(number_instructions: usize) -> Program {
        Program {
            next_instruction_index: 0,
            accumulation: 0,
            executed_instructions: vec![false; number_instructions],
        }
    }
}

fn add_to_usize(u: usize, a: Argument) -> usize {
    if a < 0 {
        u - ((-a) as usize)
    } else {
        u + (a as usize)
    }
}

fn parse_instructions(content: &str) -> Instructions {
    content
        .lines()
        .map(|line: &str| {
            let (name, argument_str) = line.split_once(' ').unwrap();
            let argument = argument_str.parse().unwrap();
            match name {
                "nop" => Instruction::Nop(argument),
                "acc" => Instruction::Acc(argument),
                "jmp" => Instruction::Jmp(argument),
                _ => panic!("Unrecognized instruction"),
            }
        })
        .collect()
}

fn execute_nop(program: &mut Program) {
    program.next_instruction_index += 1;
}

fn execute_acc(argument: &Argument, program: &mut Program) {
    program.accumulation += argument;
    program.next_instruction_index += 1;
}

fn execute_jmp(argument: &Argument, program: &mut Program) {
    program.next_instruction_index =
        add_to_usize(program.next_instruction_index, *argument);
}

fn try_replace(
    instructions: &Instructions,
    program: &Program,
) -> Option<Argument> {
    let instruction_index = program.next_instruction_index;
    let instruction = instructions.get(instruction_index).unwrap();
    let mut new_program = program.clone();

    match instruction {
        Instruction::Nop(argument) => execute_jmp(argument, &mut new_program),
        Instruction::Jmp(_) => execute_nop(&mut new_program),
        _ => return None,
    };

    accumulate_before_loop(new_program, instructions, false)
}

fn accumulate_before_loop(
    mut program: Program,
    instructions: &Instructions,
    can_replace: bool,
) -> Option<Argument> {
    loop {
        let instruction = match instructions.get(program.next_instruction_index)
        {
            None => return Some(program.accumulation),
            Some(instruction) => instruction,
        };

        if can_replace {
            if let Some(replace_result) = try_replace(instructions, &program) {
                return Some(replace_result);
            }
        }

        let executed_instruction: &mut bool = program
            .executed_instructions
            .get_mut(program.next_instruction_index)
            .unwrap();

        if *executed_instruction {
            return None;
        }
        *executed_instruction = true;

        match instruction {
            Instruction::Nop(_) => execute_nop(&mut program),
            Instruction::Acc(argument) => execute_acc(argument, &mut program),
            Instruction::Jmp(argument) => execute_jmp(argument, &mut program),
        }
    }
}

fn solve(content: &str) {
    let instructions = parse_instructions(content);
    let program = Program::new(instructions.len());
    let accumulation =
        accumulate_before_loop(program, &instructions, true).unwrap();
    println!("{}", accumulation)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let default_filename: &str = "./res/input.txt";
    let filename: &str =
        args.get(1).map(|s| s.as_ref()).unwrap_or(default_filename);

    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    solve(&content);
}

#[cfg(test)]
mod tests {}
