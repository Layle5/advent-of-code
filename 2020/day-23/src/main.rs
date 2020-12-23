use std::{cmp::min, fs};
use std::{collections::VecDeque, env};

use itertools::Itertools;

type Cup = u64;
type Cups = VecDeque<Cup>;

fn parse_cups(content: &str) -> Cups {
    content
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect()
}

fn pick_up_cups(cups: &mut Cups, current_cup_index: &mut usize) -> Cups {
    let number_end_cups = min(3, cups.len() - *current_cup_index - 1);
    let mut picked_up_cups: Cups = cups
        .iter()
        .copied()
        .skip(*current_cup_index + 1)
        .take(number_end_cups)
        .collect();
    for _ in 0..number_end_cups {
        cups.remove(*current_cup_index + 1);
    }
    for _ in picked_up_cups.len()..3 {
        let cup = cups.pop_front().unwrap();
        picked_up_cups.push_back(cup);
        *current_cup_index -= 1;
    }
    picked_up_cups
}

fn find_destination_cup(cups: &Cups, current_cup_index: usize) -> usize {
    let min_cup = *cups.iter().min().unwrap();
    let max_cup = *cups.iter().max().unwrap();
    let mut destination_cup = cups[current_cup_index] - 1;
    loop {
        if destination_cup < min_cup {
            destination_cup = max_cup;
        }

        if let Some(destination_cup_index) =
            cups.iter().position(|cup| *cup == destination_cup)
        {
            return destination_cup_index;
        }

        destination_cup -= 1;
    }
}

fn insert_cups(
    cups: &mut Cups,
    picked_up_cups: Cups,
    destination_cup_index: usize,
    current_cup_index: &mut usize,
) {
    let mut insert_index = destination_cup_index + 1;
    for picked_up_cup in picked_up_cups {
        cups.insert(insert_index, picked_up_cup);
        if insert_index <= *current_cup_index {
            *current_cup_index += 1;
        }
        insert_index += 1;
    }
}

fn play_moves(cups: &mut Cups, number_moves: usize) {
    let mut current_cup_index = 0;
    for _ in 1..=number_moves {
        let picked_up_cups = pick_up_cups(cups, &mut current_cup_index);
        let destination_cup_index =
            find_destination_cup(cups, current_cup_index);
        insert_cups(
            cups,
            picked_up_cups,
            destination_cup_index,
            &mut current_cup_index,
        );
        current_cup_index = (current_cup_index + 1) % cups.len();
    }
}

fn get_labels_after_1(mut cups: Cups) -> String {
    while *cups.front().unwrap() != 1 {
        cups.rotate_left(1);
    }
    cups.iter().skip(1).map(Cup::to_string).join("")
}

fn solve_part_1(content: &str) {
    let mut cups = parse_cups(content);
    play_moves(&mut cups, 100);
    let labels = get_labels_after_1(cups);
    println!("Part 1: {}", labels);
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
