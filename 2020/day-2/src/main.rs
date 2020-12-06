#![feature(str_split_once)]

use std::env;
use std::fs;

#[derive(Debug)]
struct Line<'a> {
    min: usize,
    max: usize,
    letter: char,
    password: &'a str,
}

fn split<'a>(s: &'a str, separators: &[char]) -> Vec<&'a str> {
    let mut splits = Vec::with_capacity(separators.len() + 1);
    let mut push_split = |split: &'a str| {
        if split.len() > 0 {
            splits.push(split)
        }
    };

    let mut rest = s;
    for &delimiter in separators {
        let (left_split, right_split) = rest.split_once(delimiter).unwrap();
        push_split(left_split);
        rest = right_split
    }
    push_split(rest);

    splits
}

fn line_from_str<'a>(s: &'a str) -> Line<'a> {
    let splits = split(s, &['-', ' ', ':', ' ']);

    let min = splits[0].parse::<usize>().unwrap();
    let max = splits[1].parse::<usize>().unwrap();
    let letter = splits[2].chars().next().unwrap();
    let password = splits[3];

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
mod tests {
    use super::split;
    #[test]
    fn split_test() {
        let empty_strs: &[&str; 0] = &[];
        assert_eq!(split("", &[]), empty_strs);
        assert_eq!(split("1", &[]), &["1"]);
        assert_eq!(split("1-", &['-']), &["1"]);
        assert_eq!(split("1-2", &['-']), &["1", "2"]);
        assert_eq!(split("-2", &['-']), &["2"]);
        assert_eq!(split("1-2-3", &['-']), &["1", "2-3"]);
        assert_eq!(split("1-2:3", &['-', ':']), &["1", "2", "3"]);
        assert_eq!(split("1-2:3-4", &['-', ':', '-']), &["1", "2", "3", "4"]);
    }
}
