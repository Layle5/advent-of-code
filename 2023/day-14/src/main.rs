use std::collections::{HashSet};
use itertools::Itertools;

const TILE_EMPTY: char = '.';
const TILE_CUBE_ROCK: char = '#';
const TILE_ROUND_ROCK: char = 'O';

fn main() {
    let input = include_str!("input.txt");
    let grid = input.lines().map(|line| line.trim().chars().collect_vec()).collect_vec();

    let mut grid_clone = grid.clone();
    roll_rocks_north(&mut grid_clone);
    let total_load = get_north_load(&grid_clone);
    println!("Part 1: {}", total_load);

    let final_load = find_load_for_cycle(grid);
    println!("Part 2: {}", final_load);
}

fn find_load_for_cycle(mut grid: Vec<Vec<char>>) -> usize {
    let mut start_repeats_cycle = None;
    let mut previous_results: Vec<CycleResult> = vec![];

    for cycle in 0..1000 {
        run_spin_cycle(&mut grid);

        // Skip a certain number of cycles to make sure we are within the repeating pattern of loads.
        if cycle < 100 {
            continue;
        }

        let north_load = get_north_load(&grid);
        let rounded_rocks = get_rounded_rocks(&grid);

        if let Some(found_result) = previous_results.iter().find(|result| result.rounded_rocks == rounded_rocks) {
            match start_repeats_cycle {
                None => {
                    start_repeats_cycle = Some(found_result.cycle);
                }
                Some(start_repeats_cycle) => {
                    if start_repeats_cycle == found_result.cycle {
                        let equivalent_cycle = compute_equivalent_cycle_to_target(cycle, start_repeats_cycle, found_result);
                        let equivalent_result = previous_results.iter().find(|result| result.cycle == equivalent_cycle).unwrap();
                        return equivalent_result.north_load;
                    }
                }
            }
        } else {
            previous_results.push(CycleResult { cycle: cycle + 1, rounded_rocks, north_load });
        }
    }

    panic!("could not find repeats with 1000 attempts")
}

fn compute_equivalent_cycle_to_target(cycle: usize, start_repeats_cycle: usize, found_result: &CycleResult) -> usize {
    let target_cycle = 1000000000;
    let repeat_size = ((cycle + 1) - start_repeats_cycle) / 2;
    let equivalent_cycle = (target_cycle - found_result.cycle) % repeat_size + found_result.cycle;
    equivalent_cycle
}

struct CycleResult {
    cycle: usize,
    rounded_rocks: HashSet<(usize, usize)>,
    north_load: usize,
}

fn run_spin_cycle(grid: &mut Vec<Vec<char>>) {
    for _ in 0..4 {
        roll_rocks_north(grid);
        *grid = rotate(grid);
    }
}

fn rotate(old_grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let old_rows = old_grid.len();
    let old_cols = old_grid[0].len();
    let mut new_grid = vec![vec!['?'; old_rows]; old_cols];

    for row in 0..old_rows {
        for col in 0..old_cols {
            new_grid[col][old_rows - row - 1] = old_grid[row][col];
        }
    }

    new_grid
}

fn roll_rocks_north(grid: &mut Vec<Vec<char>>) {
    let number_rows = grid.len();
    let number_cols = grid[0].len();

    for col in 0..number_cols {
        let mut roll_row = 0;
        for row in 0..number_rows {
            let tile = grid[row][col];
            match tile {
                TILE_CUBE_ROCK => {
                    roll_row = row + 1;
                }
                TILE_ROUND_ROCK => {
                    grid[row][col] = TILE_EMPTY;
                    grid[roll_row][col] = TILE_ROUND_ROCK;
                    roll_row += 1;
                }
                _ => {}
            }
        }
    }
}

fn get_north_load(grid: &[Vec<char>]) -> usize {
    let mut total_load = 0;
    let number_rows = grid.len();
    let number_cols = grid[0].len();

    for col in 0..number_cols {
        for row in 0..number_rows {
            if grid[row][col] == TILE_ROUND_ROCK {
                total_load += number_rows - row;
            }
        }
    }

    total_load
}

fn get_rounded_rocks(grid: &[Vec<char>]) -> HashSet<(usize, usize)> {
    let mut set = HashSet::default();

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == TILE_ROUND_ROCK {
                set.insert((row, col));
            }
        }
    }

    set
}