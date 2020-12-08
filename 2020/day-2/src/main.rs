#![feature(str_split_once)]

use std::env;
use std::fs;

#[macro_use]
extern crate lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Line<'a> {
    min: usize,
    max: usize,
    letter: char,
    password: &'a str,
}

fn line_from_str<'a>(s: &'a str) -> Line<'a> {
    lazy_static! {
        static ref REGEX: Regex =
            Regex::new(r"^(\d*)-(\d*) (.): (.*)$").unwrap();
    }

    let captures = REGEX.captures(s).unwrap();

    let min = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let max = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
    let letter = captures.get(3).unwrap().as_str().chars().next().unwrap();
    let password = captures.get(4).unwrap().as_str();

    Line {
        min,
        max,
        letter,
        password,
    }
}

fn xor(b1: bool, b2: bool) -> bool {
    match (b1, b2) {
        (false, false) => false,
        (true, false) => true,
        (false, true) => true,
        (true, true) => false,
    }
}

fn is_line_valid_part_1(line: &Line) -> bool {
    let letter_count =
        line.password.chars().filter(|&c| c == line.letter).count();
    line.min <= letter_count && letter_count <= line.max
}

fn is_line_valid_part_2(line: &Line) -> bool {
    let letter_1 = line.password.chars().nth(line.min - 1).unwrap();
    let letter_2 = line.password.chars().nth(line.max - 1).unwrap();
    xor(letter_1 == line.letter, letter_2 == line.letter)
}

fn solve(content: &str, is_line_valid: fn(&Line) -> bool) -> usize {
    content
        .lines()
        .map(|line_str| line_from_str(line_str))
        .filter(|line| is_line_valid(line))
        .count()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let default_filename: &str = "./res/input.txt";
    let filename: &str =
        args.get(1).map(|s| s.as_ref()).unwrap_or(default_filename);

    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("Part 1: {}", solve(&content, is_line_valid_part_1));
    println!("Part 2: {}", solve(&content, is_line_valid_part_2));
}

#[cfg(test)]
mod tests {}
