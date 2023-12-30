use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let input = include_str!("input.txt");
    let grid = parse_grid(input);
    let start_position = find_start(&grid);
    let tile_loop = find_loop(&grid, start_position);
    let farthest = tile_loop.len() / 2;
    println!("Part 1: {}", farthest);

    let twice_area_count = compute_twice_area(&tile_loop);
    let border_count = tile_loop.len() - 1;
    let inside_count = (twice_area_count - border_count + 2) / 2;
    println!("Part 2: {}", inside_count);
}

type Position = (usize, usize);

fn compute_twice_area(tile_loop: &[Position]) -> usize {
    tile_loop
        .iter()
        .tuple_windows()
        .map(|(lhs, rhs)| (lhs.0 * rhs.1) as isize - (lhs.1 * rhs.0) as isize)
        .sum::<isize>()
        .abs_diff(0)
}

fn find_loop(grid: &[Vec<Tile>], start: Position) -> Vec<Position> {
    let mut position = start;
    let mut pipe_loop = vec![];

    loop {
        pipe_loop.push(position);

        for (direction, neighbor) in neighbors(position, grid) {
            let tile = grid[position.0][position.1];
            let neighbor_tile = grid[neighbor.0][neighbor.1];
            if are_connecting(tile, direction, neighbor_tile) {
                if neighbor == start {
                    pipe_loop.push(start);
                    return pipe_loop;
                }

                if !pipe_loop.contains(&neighbor) {
                    position = neighbor;
                    break;
                }
            }
        }
    }
}

fn neighbors(
    position: Position,
    grid: &[Vec<Tile>],
) -> impl Iterator<Item = (Direction, Position)> + '_ {
    [
        (
            Direction::North,
            position.0.checked_sub(1).map(|row| (row, position.1)),
        ),
        (
            Direction::East,
            position.1.checked_add(1).map(|col| (position.0, col)),
        ),
        (
            Direction::South,
            position.0.checked_add(1).map(|row| (row, position.1)),
        ),
        (
            Direction::West,
            position.1.checked_sub(1).map(|col| (position.0, col)),
        ),
    ]
    .into_iter()
    .flat_map(|(d, op)| op.map(|p| (d, p)))
    .filter(|p| grid.get(p.1 .0).and_then(|row| row.get(p.1 .1)).is_some())
}

fn are_connecting(from: Tile, direction: Direction, to: Tile) -> bool {
    connects(from, direction) && connects(to, direction.reverse())
}

fn connects(pipe: Tile, direction: Direction) -> bool {
    match (pipe, direction) {
        (Tile::PipeVertical, Direction::North) | (Tile::PipeVertical, Direction::South) => true,
        (Tile::PipeHorizontal, Direction::East) | (Tile::PipeHorizontal, Direction::West) => true,
        (Tile::PipeNorthEast, Direction::North) | (Tile::PipeNorthEast, Direction::East) => true,
        (Tile::PipeNorthWest, Direction::North) | (Tile::PipeNorthWest, Direction::West) => true,
        (Tile::PipeSouthWest, Direction::South) | (Tile::PipeSouthWest, Direction::West) => true,
        (Tile::PipeSouthEast, Direction::South) | (Tile::PipeSouthEast, Direction::East) => true,
        (Tile::Start, _) => true,
        _ => false,
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn reverse(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

fn find_start(grid: &[Vec<Tile>]) -> Position {
    grid.iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .find(|(col_index, col)| **col == Tile::Start)
                .map(|(col_index, _)| (row_index, col_index))
        })
        .next()
        .unwrap()
}

fn parse_grid(input: &str) -> Vec<Vec<Tile>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| match c {
                    '|' => Tile::PipeVertical,
                    '-' => Tile::PipeHorizontal,
                    'L' => Tile::PipeNorthEast,
                    'J' => Tile::PipeNorthWest,
                    '7' => Tile::PipeSouthWest,
                    'F' => Tile::PipeSouthEast,
                    '.' => Tile::Ground,
                    'S' => Tile::Start,
                    _ => panic!("unrecognized tile"),
                })
                .collect_vec()
        })
        .collect_vec()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    PipeVertical,
    PipeHorizontal,
    PipeNorthEast,
    PipeNorthWest,
    PipeSouthWest,
    PipeSouthEast,
    Ground,
    Start,
}
