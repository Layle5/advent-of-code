mod computer;

#[macro_use]
extern crate num_derive;

use computer::{run, Int, Program};
use itertools::Itertools;
use std::env;
use std::fs;

type Position = (Int, Int);

fn is_in_beam(program: &Program, pos: Position) -> bool {
    let mut p = program.clone();
    p.input(pos.0);
    p.input(pos.1);
    run(&mut p);
    p.output().unwrap() != 0
}

fn solve_part_1(content: &str) {
    let program: Program = content.parse().unwrap();

    let size = 50;

    let outputs = (0..size)
        .cartesian_product(0..size)
        .map(|pos| is_in_beam(&program, pos))
        .collect_vec();

    let count = outputs.into_iter().filter(|&b| b).count();

    println!("Part 1: {}", count);
}

fn solve_part_2(content: &str) {
    let program: Program = content.parse().unwrap();
    let is_in_beam = |pos| is_in_beam(&program, pos);

    let size = 100;
    let mut upper_right = (4, 6);

    loop {
        let is_upper_right_in_beam = is_in_beam(upper_right);
        if is_upper_right_in_beam {
            let lower_left_op = {
                if upper_right.1 + 1 >= size {
                    Some((upper_right.0 + size - 1, upper_right.1 + 1 - size))
                } else {
                    None
                }
            };

            let mut found = false;
            if let Some(lower_left) = lower_left_op {
                let is_lower_left_in_beam = is_in_beam(lower_left);
                if is_lower_left_in_beam {
                    found = true;
                }
            }
            if found {
                break;
            } else {
                upper_right.1 += 1;
            }
        } else {
            upper_right.0 += 1;
            upper_right.1 -= 1;
            if !is_in_beam(upper_right) {
                upper_right.1 += 1;
            }
        }
    }

    let upper_left = (upper_right.0, upper_right.1 - size + 1);
    println!("Part 2: {}", upper_left.0 * 10000 + upper_left.1);
}

fn main() {
    let args = env::args().collect_vec();
    let filename = args.get(1).map(|s| s.as_ref()).unwrap_or("./res/input.txt");

    let content = fs::read_to_string(filename).unwrap();

    solve_part_1(&content);
    solve_part_2(&content);
}
