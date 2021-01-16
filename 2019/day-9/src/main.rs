mod computer;

#[macro_use]
extern crate num_derive;

use computer::{run_program_until_output, Program};
use itertools::Itertools;
use std::env;
use std::fs;

fn solve_part_1(content: &str) {
    let mut program: Program = content.parse().unwrap();
    program.input(1);
    let output = run_program_until_output(&mut program).unwrap();
    println!("Part 1: {}", output);
}

fn solve_part_2(content: &str) {
    let mut program: Program = content.parse().unwrap();
    program.input(2);
    let output = run_program_until_output(&mut program).unwrap();
    println!("Part 2: {}", output);
}

fn main() {
    let args = env::args().collect_vec();
    let filename = args.get(1).map(|s| s.as_ref()).unwrap_or("./res/input.txt");

    let content = fs::read_to_string(filename).unwrap();

    solve_part_1(&content);
    solve_part_2(&content);
}
