use std::{collections::HashMap, fs, hash::Hash};
use std::{collections::HashSet, env};

use itertools::Itertools;

type Tiles<T> = HashSet<T>;

fn parse<T>(content: &str, tile_new: fn(usize, usize) -> T) -> Tiles<T>
where
    T: Eq,
    T: Hash,
{
    content
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, tile_char)| *tile_char == '#')
                .map(move |(x, _)| tile_new(y, x))
        })
        .collect()
}

fn iterate_algorithm_step<T, F>(
    prev_tiles: Tiles<T>,
    get_neighbors: F,
) -> Tiles<T>
where
    T: Copy,
    T: Eq,
    T: Hash,
    F: Fn(&T) -> Vec<T>,
{
    let mut next_tiles = Tiles::new();
    let mut possible_next_tiles: HashMap<T, bool> =
        prev_tiles.iter().map(|t| (*t, true)).collect();
    {
        let inactive_neighbor_tiles = prev_tiles
            .iter()
            .flat_map(|tile| get_neighbors(tile).into_iter());

        for t in inactive_neighbor_tiles {
            possible_next_tiles.entry(t).or_insert(false);
        }
    }

    for (tile, prev_is_active) in possible_next_tiles {
        let c = get_neighbors(&tile)
            .into_iter()
            .filter(|neighbor| prev_tiles.contains(neighbor))
            .count();

        let next_is_active = match prev_is_active {
            true => c == 2 || c == 3,
            false => c == 3,
        };

        if next_is_active {
            next_tiles.insert(tile);
        }
    }

    next_tiles
}

fn iterate_algorithm<T, F>(start_tiles: Tiles<T>, get_neighbors: F) -> Tiles<T>
where
    T: Copy,
    T: Eq,
    T: Hash,
    F: Fn(&T) -> Vec<T>,
{
    let mut prev_tiles = start_tiles;
    for _ in 0..6 {
        prev_tiles = iterate_algorithm_step(prev_tiles, &get_neighbors);
    }
    prev_tiles
}

type GridTile = (isize, isize, isize);
type HyperTile = (isize, isize, isize, isize);

fn get_grid_neighbors(tile: &GridTile) -> Vec<GridTile> {
    let (tz, ty, tx) = tile;
    (-1..=1)
        .cartesian_product(-1..=1)
        .cartesian_product(-1..=1)
        .map(|((dz, dy), dx)| (dz, dy, dx))
        .map(move |(dz, dy, dx)| (tz + dz, ty + dy, tx + dx))
        .filter(|neighbor| *neighbor != *tile)
        .collect()
}

fn solve_part_1(content: &str) {
    let start_tiles = parse(content, |y, x| (0, y as isize, x as isize));
    let final_tiles = iterate_algorithm(start_tiles, get_grid_neighbors);
    println!("Part 1: {}", final_tiles.len());
}

fn get_hyper_grid_neighbors(tile: &HyperTile) -> Vec<HyperTile> {
    let (tw, tz, ty, tx) = tile;
    (-1..=1)
        .cartesian_product(-1..=1)
        .cartesian_product(-1..=1)
        .cartesian_product(-1..=1)
        .map(|(((dw, dz), dy), dx)| (dw, dz, dy, dx))
        .map(move |(dw, dz, dy, dx)| (tw + dw, tz + dz, ty + dy, tx + dx))
        .filter(|neighbor| *neighbor != *tile)
        .collect()
}

fn solve_part_2(content: &str) {
    let start_tiles = parse(content, |y, x| (0, 0, y as isize, x as isize));
    let final_tiles = iterate_algorithm(start_tiles, get_hyper_grid_neighbors);
    println!("Part 2: {}", final_tiles.len());
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let default_filename: &str = "./res/input.txt";
    let filename: &str =
        args.get(1).map(|s| s.as_ref()).unwrap_or(default_filename);

    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    solve_part_1(&content);
    solve_part_2(&content);
}

#[cfg(test)]
mod tests {}
