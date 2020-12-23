use std::env;
use std::fs;

type Cup = usize;
type Cups = Vec<Cup>;

fn parse_cups(content: &str, minimum_number: usize) -> (Cup, Cups) {
    let mut cups_vec: Cups = content
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();
    while cups_vec.len() < minimum_number {
        cups_vec.push(cups_vec.len() + 1);
    }
    let first_cup = *cups_vec.first().unwrap();
    let last_cup = *cups_vec.last().unwrap();
    let mut cups = vec![0; cups_vec.len() + 1];
    for next_index in 1..cups_vec.len() {
        let next_cup = cups_vec[next_index];
        let previous_cup = cups_vec[next_index - 1];
        cups[previous_cup] = next_cup;
    }
    cups[last_cup] = first_cup;
    (first_cup, cups)
}

fn play_moves(cups: &mut Cups, start_cup: Cup, number_moves: usize) {
    let mut current_cup = start_cup;
    for _ in 1..=number_moves {
        let picked_cup_1 = cups[current_cup];
        let picked_cup_2 = cups[picked_cup_1];
        let picked_cup_3 = cups[picked_cup_2];
        let picked_cups = [picked_cup_1, picked_cup_2, picked_cup_3];

        cups[current_cup] = cups[picked_cup_3];

        let get_previous = |i| {
            if i > 1 {
                i - 1
            } else {
                cups.len() - 1
            }
        };
        let mut destination_cup = get_previous(current_cup);
        while picked_cups.contains(&destination_cup) {
            destination_cup = get_previous(destination_cup);
        }

        cups[picked_cup_3] = cups[destination_cup];
        cups[destination_cup] = picked_cup_1;

        current_cup = cups[current_cup];
    }
}

fn get_labels_after_1(cups: Cups) -> String {
    let mut labels = String::new();
    let mut current_cup = 1;
    while cups[current_cup] != 1 {
        labels += &cups[current_cup].to_string();
        current_cup = cups[current_cup]
    }
    labels
}

fn solve_part_1(content: &str) {
    let (start_cup, mut cups) = parse_cups(content, 0);
    play_moves(&mut cups, start_cup, 100);
    let labels = get_labels_after_1(cups);
    println!("Part 1: {}", labels);
}

fn get_product_after_1(cups: Cups) -> Cup {
    cups[1] * cups[cups[1]]
}

fn solve_part_2(content: &str) {
    let (start_cup, mut cups) = parse_cups(content, 1000000);
    play_moves(&mut cups, start_cup, 10000000);
    let product = get_product_after_1(cups);
    println!("Part 2: {}", product);
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
