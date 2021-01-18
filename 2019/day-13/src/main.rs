mod computer;

#[macro_use]
extern crate num_derive;

use computer::{run_program, run_program_until_outputs_with, Int, Program};
use itertools::Itertools;
use std::env;
use std::fs;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Object {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl From<Int> for Object {
    fn from(i: Int) -> Self {
        match i {
            0 => Object::Empty,
            1 => Object::Wall,
            2 => Object::Block,
            3 => Object::Paddle,
            4 => Object::Ball,
            _ => panic!(),
        }
    }
}

fn solve_part_1(content: &str) {
    let mut program: Program = content.parse().unwrap();
    run_program(&mut program);

    let objects = program
        .outputs
        .into_iter()
        .chunks(3)
        .into_iter()
        .map(|chunk| chunk.collect_vec())
        .collect_vec();

    let max_y = objects.iter().map(|o| o[1]).max().unwrap() as usize;
    let max_x = objects.iter().map(|o| o[0]).max().unwrap() as usize;
    let mut grid = vec![vec![Object::Empty; max_x + 1]; max_y + 1];

    for object in &objects {
        grid[object[1] as usize][object[0] as usize] = object[2].into();
    }

    let block_count: usize = grid
        .into_iter()
        .map(|line| {
            line.into_iter()
                .filter(|object| *object == Object::Block)
                .count()
        })
        .sum();

    println!("Part 1: {}", block_count);
}

fn solve_part_2(content: &str) {
    let mut program: Program = content.parse().unwrap();
    program.memory[0] = 2;

    let mut score: Int = 0;
    let mut ball_col: Int = 0;
    let mut paddle_col: Int = 0;
    loop {
        let outputs = run_program_until_outputs_with(&mut program, 3, |_| {
            Some((ball_col - paddle_col).signum())
        });

        if let Some((col, row, id)) = outputs.into_iter().collect_tuple() {
            if col == -1 && row == 0 {
                score = id;
            } else if id == 3 {
                paddle_col = col;
            } else if id == 4 {
                ball_col = col;
            }
        } else {
            break;
        }
    }

    println!("Part 2: {}", score)
}

fn main() {
    let args = env::args().collect_vec();
    let filename = args.get(1).map(|s| s.as_ref()).unwrap_or("./res/input.txt");

    let content = fs::read_to_string(filename).unwrap();

    solve_part_1(&content);
    solve_part_2(&content);
}
