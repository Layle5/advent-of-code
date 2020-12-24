use core::panic;
use std::{collections::HashMap, fs};
use std::{collections::HashSet, env};

type Position = (isize, isize);

struct Directions;

impl Directions {
    const E: Position = (1, 0);
    const SE: Position = (1, -1);
    const SW: Position = (0, -1);
    const W: Position = (-1, 0);
    const NW: Position = (-1, 1);
    const NE: Position = (0, 1);

    fn all() -> [Position; 6] {
        [
            Directions::E,
            Directions::SE,
            Directions::SW,
            Directions::W,
            Directions::NW,
            Directions::NE,
        ]
    }
}

type Path = Vec<Position>;
type PositionList = Vec<Position>;
type PositionSet = HashSet<Position>;
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

fn fold_paths(paths: Paths) -> PositionList {
    paths
        .into_iter()
        .map(|path| {
            path.into_iter().fold((0, 0), |a, p| (a.0 + p.0, a.1 + p.1))
        })
        .collect()
}

fn flip_tiles(positions: PositionList) -> PositionSet {
    positions
        .into_iter()
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

fn get_neighbors(tile: Position) -> PositionSet {
    Directions::all()
        .iter()
        .map(|direction| (tile.0 + direction.0, tile.1 + direction.1))
        .collect()
}

fn iterate_day(tiles: PositionSet) -> PositionSet {
    let mut possible_tiles: HashMap<Position, bool> = tiles
        .iter()
        .map(|&tile| get_neighbors(tile))
        .flat_map(|neighbors| neighbors.into_iter())
        .map(|neighbor| (neighbor, false))
        .collect();

    for &tile in &tiles {
        possible_tiles.insert(tile, true);
    }

    possible_tiles
        .into_iter()
        .filter(|&(tile, is_flipped)| {
            let neighbors_count = get_neighbors(tile)
                .into_iter()
                .filter(|neighbor| tiles.contains(neighbor))
                .count();

            match is_flipped {
                true => [1, 2].contains(&neighbors_count),
                false => neighbors_count == 2,
            }
        })
        .map(|(tile, _)| tile)
        .collect()
}
fn iterate_days(tiles: PositionSet) -> PositionSet {
    (0..100).fold(tiles, |prev_tiles, _| iterate_day(prev_tiles))
}

fn solve_part_2(content: &str) {
    let paths = parse_paths(content);
    let positions = fold_paths(paths);
    let start_tiles = flip_tiles(positions);
    let final_tiles = iterate_days(start_tiles);
    println!("Part 1: {}", final_tiles.len());
}

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
