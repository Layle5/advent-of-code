mod computer;

#[macro_use]
extern crate num_derive;

use computer::{run, Int, Program, ProgramState};
use itertools::Itertools;
use std::env;
use std::fs;

fn get_damage(content: &str, instructions: &[&str]) -> Int {
    let mut program: Program = content.parse().unwrap();

    for byte in instructions.iter().join("").bytes() {
        program.input(byte as Int);
    }

    while run(&mut program) == ProgramState::Output {
        continue;
    }
    
    *program.outputs.back().unwrap()
}

fn solve_part_1(content: &str) {
    let out = get_damage(
        content,
        &[
            "NOT A J\n",
            "NOT B T\n",
            "AND D T\n",
            "OR T J\n",
            "NOT C T\n",
            "AND D T\n",
            "OR T J\n",
            "WALK\n",
        ],
    );

    println!("Part 1: {}", out);
}

fn solve_part_2(content: &str) {
    let out = get_damage(content, &[
            "NOT A J\n",
            "NOT B T\n",
            "AND D T\n",
            "OR T J\n",
            "NOT C T\n",
            "AND D T\n",
            "AND H T\n",
            "OR T J\n",
            "RUN\n",
    ]);

    println!("Part 2: {}", out);
}

fn main() {
    let args = env::args().collect_vec();
    let filename = args.get(1).map(|s| s.as_ref()).unwrap_or("./res/input.txt");

    let content = fs::read_to_string(filename).unwrap();

    solve_part_1(&content);
    solve_part_2(&content);
}
