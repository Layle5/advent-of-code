use core::panic;
use std::{collections::{HashSet, VecDeque}, env};
use std::fs;

type Position = (isize, isize);

struct Directions;

impl Directions {
    const E: Position = (1, 0);
    const SE: Position = (1, -1);
    const SW: Position = (0, -1);
    const W: Position = (-1, 0);
    const NW: Position = (-1, 1);
    const NE: Position = (0, 1);
}

type Path = Vec<Position>;
type Positions = VecDeque<Position>;
type Paths = Vec<Path>;

fn parse_paths(content: &str) -> Paths {
    content
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let mut index = 0;
            let mut path = Path::new();
            while index < bytes.len() {
                let direction = match bytes[index] {
                    b'e' => Directions::E,
                    b'w' => Directions::W,
                    b's' => {
                        index += 1;
                        match bytes[index] {
                            b'e' => Directions::SE,
                            b'w' => Directions::SW,
                            _ => panic!(),
                        }
                    }
                    b'n' => {
                        index += 1;
                        match bytes[index] {
                            b'e' => Directions::NE,
                            b'w' => Directions::NW,
                            _ => panic!(),
                        }
                    }
                    _ => panic!(),
                };
                path.push(direction);
                index += 1;
            }
            path
        })
        .collect()
}

fn fold_paths(paths: Paths) -> Positions {
    paths.into_iter().map(|path| {
        path.into_iter().fold((0,0), |a, p| {
            (a.0 + p.0, a.1 + p.1)
        })
    }).collect()
}

fn flip_tiles(positions: Positions) -> HashSet<Position> {
    positions.into_iter()
    .fold(HashSet::new(), |mut ft, position| {
        if ft.contains(&position) {
            ft.remove(&position);
        } else {
            ft.insert(position);
        }
        ft
    })
}

fn solve_part_1(content: &str) {
    let paths = parse_paths(content);
    let positions = fold_paths(paths);
    let flipped_tiles = flip_tiles(positions);
    println!("Part 1: {}", flipped_tiles.len());
}

fn solve_part_2(_content: &str) {}

fn get_content(index: usize, default_filename: &str) -> String {
    let args: Vec<String> = env::args().collect();
    let filename: &str = args
        .get(index)
        .map(|s| s.as_ref())
        .unwrap_or(default_filename);

    fs::read_to_string(filename).expect("Something went wrong reading the file")
}

fn main() {
    let content = get_content(1, "./res/input.txt");
    solve_part_1(&content);
    solve_part_2(&content);
}

#[cfg(test)]
mod tests {}
