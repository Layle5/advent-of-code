use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("input.txt");
    let games = parse_games(input);
    let sum_of_scores = compute_sum_of_points(&games);
    println!("Part 1: {}", sum_of_scores);
    let number_copies = compute_number_of_card_instances(&games);
    println!("Part 2: {}", number_copies);
}

fn parse_games(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let (id_str, numbers_str) = line
                .trim()
                .split_once(':')
                .expect("card and numbers to be seperated with colon ':'");

            let id = (id_str)
                .strip_prefix("Card")
                .expect("card id to be perfixed with 'Card'")
                .trim()
                .parse()
                .expect("card id to be a valid number");

            let mut numbers = (numbers_str).split('|').map(|split| {
                (split)
                    .trim()
                    .split(' ')
                    .filter(|numbers_str| !numbers_str.is_empty())
                    .map(|number_str| {
                        number_str
                            .trim()
                            .parse()
                            .expect("card number to ba a valid number")
                    })
                    .collect()
            });

            Game {
                id,
                winning_numbers: numbers.next().expect("card to have winning numbers"),
                pulled_numbers: numbers.next().expect("card to have pulled numbers"),
            }
        })
        .collect()
}

#[derive(Debug)]
struct Game {
    #[allow(dead_code)]
    id: usize,
    winning_numbers: HashSet<u64>,
    pulled_numbers: HashSet<u64>,
}

fn compute_sum_of_points(games: &[Game]) -> u64 {
    games.iter().fold(0, |total_points, game| {
        let guessed_numbers_count = game
            .winning_numbers
            .intersection(&game.pulled_numbers)
            .count();

        let card_points = if guessed_numbers_count == 0 {
            0
        } else {
            2u64.pow(guessed_numbers_count as u32 - 1)
        };

        total_points + card_points
    })
}

fn compute_number_of_card_instances(games: &[Game]) -> usize {
    games
        .iter()
        .fold(
            (0, VecDeque::new()),
            |(total_instances, mut number_copies), game| {
                let current_number_instances = 1 + number_copies.pop_front().unwrap_or(0);

                let guessed_numbers_count = game
                    .winning_numbers
                    .intersection(&game.pulled_numbers)
                    .count();

                number_copies.resize(guessed_numbers_count.max(number_copies.len()), 0);
                for number_copy in number_copies.iter_mut().take(guessed_numbers_count) {
                    *number_copy += current_number_instances;
                }

                (total_instances + current_number_instances, number_copies)
            },
        )
        .0
}
