use std::env;
use std::fs;

use itertools::Itertools;

fn apply(subject_number: u64, loop_size: u64) -> u64 {
    let mut value = 1;
    for _ in 0..loop_size {
        value *= subject_number;
        value %= 20201227;
    }
    value
}

fn get_values_loop_sizes() -> Vec<u64> {
    let mut values = vec![0; 20201227];
    let mut value = 1;
    let subject_number = 7;
    for loop_size in 1..=20201227 {
        value *= subject_number;
        value %= 20201227;
        values[value] = loop_size;
    }
    values
}

fn solve_part_1(content: &str) {
    let public_keys: Vec<u64> = content.lines().map(str::parse).map(Result::unwrap).collect();
    let values = get_values_loop_sizes();
    let loop_sizes = public_keys.iter().map(|&public_key| values[public_key as usize]).collect_vec();
    let encryption_key = apply(public_keys[0], loop_sizes[1]);
    println!("Part 1: {}", encryption_key);
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
