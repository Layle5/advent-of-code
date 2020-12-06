#![feature(once_cell)]

use std::env;
use std::fs;
use std::lazy::Lazy;

fn find_numbers(numbers: &[u64], remaining: u64, expected: u64) -> u64 {
    match (numbers.first(), remaining, expected) {
        (_, 0, 0) => 1,
        (_, 0, _) => 0,
        (None, _, _) => 0,
        (Some(&head), _, _) => {
            let tail = &numbers[1..];
            let found_product = Lazy::new(|| {
                find_numbers(tail, remaining - 1, expected - head)
            });

            if expected < head || *found_product == 0 {
                find_numbers(tail, remaining, expected)
            } else {
                head * *found_product
            }
        }
    }
}

fn solve(content: &str, remaining: u64) -> u64 {
    let numbers: Vec<u64> = content
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();

    find_numbers(&numbers, remaining, 2020)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let default_filename: &str = "./res/input.txt";
    let filename: &str =
        args.get(1).map(|s| s.as_ref()).unwrap_or(default_filename);

    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("Part 1: {}", solve(&content, 2));
    println!("Part 2: {}", solve(&content, 3));
}

#[cfg(test)]
mod tests {
    use super::find_numbers;

    #[test]
    fn find_numbers_test() {
        let numbers = &[2, 3, 5, 7, 13];
        assert_eq!(find_numbers(numbers, 0, 0), 1);
        assert_eq!(find_numbers(numbers, 0, 1), 0);
        assert_eq!(find_numbers(numbers, 1, 5), 5);
        assert_eq!(find_numbers(numbers, 2, 5), 6);
        assert_eq!(find_numbers(numbers, 3, 23), 273);
    }
}
