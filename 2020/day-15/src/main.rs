use std::fs;
use std::{collections::HashMap, env};

fn find_last_turn(numbers: &[u64], target: u64) -> Option<usize> {
    for index in (0..numbers.len() - 1).rev() {
        if numbers[index] == target {
            return Some(index + 1);
        }
    }
    None
}

fn solve_part_1(content: &str) {
    let mut numbers: Vec<u64> = content
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    while numbers.len() < 2020 {
        let &last_spoked_number = numbers.last().unwrap();
        let res = find_last_turn(&numbers, last_spoked_number);
        let difference = res.map_or(0, |ppt| numbers.len() - ppt);
        numbers.push(difference as u64);
    }

    println!("Part 1: {:#?}", numbers);
}

fn solve_part_2(content: &str) {
    let start_numbers: Vec<u64> = content
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let last_start_number = *start_numbers.last().unwrap();

    let mut numbers: HashMap<u64, (u64, Option<u64>)> = start_numbers
        .into_iter()
        .enumerate()
        .map(|(i, n)| (n, (i as u64 + 1, None)))
        .collect();

    let mut last_speech =
        (last_start_number, *numbers.get(&last_start_number).unwrap());

    for current_turn in numbers.len() + 1..30000001 {
        let (_, last_value) = last_speech;
        let (last_pt, last_ppt) = last_value;

        let spoken_number = last_ppt.map_or(0, |lppt| last_pt - lppt);
        let spoken_pt = current_turn as u64;
        let spoken_ppt = numbers.get(&spoken_number).map(|(pt, _)| *pt);
        let spoken_value = (spoken_pt, spoken_ppt);

        numbers.insert(spoken_number, spoken_value);
        last_speech = (spoken_number, spoken_value);
    }

    println!("Part 2: {:#?}", last_speech.0);
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
