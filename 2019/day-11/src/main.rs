mod computer;

#[macro_use]
extern crate num_derive;

use computer::{run_program_until_outputs, Int, Program};
use itertools::Itertools;
use std::fs;
use std::{collections::HashSet, env};

type Position = (isize, isize);
type Positions = HashSet<Position>;

fn solve(content: &str, start_on_white: bool) -> (Positions, Positions) {
    let mut program: Program = content.parse().unwrap();

    let mut position: Position = (0, 0);
    let mut direction: Position = (-1, 0);
    let mut white_set: Positions = HashSet::new();
    let mut painted_set: Positions = HashSet::new();

    if start_on_white {
        white_set.insert(position);
    }

    loop {
        program.input(white_set.contains(&position) as Int);

        let outputs = run_program_until_outputs(&mut program, 2)
            .into_iter()
            .collect_tuple();

        if let Some((color, turn)) = outputs {
            painted_set.insert(position);
            match color {
                0 => white_set.remove(&position),
                _ => white_set.insert(position),
            };

            direction = match turn {
                0 => (-direction.1, direction.0),
                _ => (direction.1, -direction.0),
            };

            position = (position.0 + direction.0, position.1 + direction.1);
        } else {
            break;
        }
    }

    (white_set, painted_set)
}

fn solve_part_1(content: &str) {
    let (_, painted_set) = solve(content, false);
    println!("Part 1: {:?}", painted_set.len())
}

fn solve_part_2(content: &str) {
    let (white_iset, _) = solve(content, true);

    let min_row = white_iset.iter().map(|p| p.0).min().unwrap();
    let min_col = white_iset.iter().map(|p| p.1).min().unwrap();

    let white_set = white_iset
        .into_iter()
        .map(|p| (p.0 - min_row, p.1 - min_col))
        .map(|p| (p.0 as usize, p.1 as usize))
        .collect_vec();

    let max_row = white_set.iter().map(|p| p.0).max().unwrap();
    let max_col = white_set.iter().map(|p| p.1).max().unwrap();

    let mut grid = vec![vec![' '; max_col + 1]; max_row + 1];
    for (row, col) in white_set {
        grid[row][col] = '\u{2588}';
    }

    let image = grid
        .into_iter()
        .map(|line| line.into_iter().collect::<String>())
        .join("\n");

    println!("Part 2:\n{}", image)
}

fn main() {
    let args = env::args().collect_vec();
    let filename = args.get(1).map(|s| s.as_ref()).unwrap_or("./res/input.txt");

    let content = fs::read_to_string(filename).unwrap();

    solve_part_1(&content);
    solve_part_2(&content);
}
