mod computer;

use computer::{run_program_until_output, Int, Program};
use itertools::Itertools;
use std::fs;
use std::{env, ops::Range};

#[macro_use]
extern crate num_derive;

fn compute_signal(start_program: &Program, phases: Vec<Int>) -> Int {
    let mut programs = vec![start_program.clone(); phases.len()];
    for index in 0..phases.len() {
        programs[index].inputs.push_back(phases[index])
    }

    let mut input = 0;
    for index in (0..phases.len()).cycle() {
        let program = &mut programs[index];
        program.inputs.push_back(input);
        run_program_until_output(program);
        match program.outputs.pop_back() {
            Some(output) => input = output,
            None => break,
        }
    }

    input
}

fn solve(content: &str, range: Range<Int>) -> Int {
    let start_program = content.parse().unwrap();

    range
        .permutations(5)
        .map(|phases| compute_signal(&start_program, phases))
        .max()
        .unwrap()
}

fn solve_part_1(content: &str) {
    let highest_signal = solve(content, 0..5);
    println!("Part 1: {}", highest_signal)
}

fn solve_part_2(content: &str) {
    let highest_signal = solve(content, 5..10);
    println!("Part 2: {}", highest_signal)
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
