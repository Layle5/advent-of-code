use std::fs;
use std::{collections::VecDeque, env};

use itertools::Itertools;

type Card = u64;
type Deck = VecDeque<Card>;
type Decks = Vec<Deck>;
type Score = u64;

fn parse_deck(paragraph: &[&str]) -> Deck {
    paragraph
        .iter()
        .skip(1)
        .copied()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn parse_decks(content: &str) -> Decks {
    let lines = content.lines().collect_vec();
    let paragraphs = lines.split(|line| line.is_empty());
    paragraphs
        .map(|paragraph| parse_deck(paragraph))
        .collect_vec()
}

fn get_winning_deck_score(mut decks: Decks) -> Score {
    while decks.iter().all(|deck| !deck.is_empty()) {
        let played_cards =
            decks.iter_mut().map(|deck| deck.pop_front()).collect_vec();

        let winning_card_pair_op = played_cards
            .iter()
            .enumerate()
            .filter_map(|(index, card_op)| card_op.map(|card| (index, card)))
            .max_by_key(|(_, card)| *card);

        if winning_card_pair_op.is_none() {
            panic!()
        }

        let (winning_index, _) = winning_card_pair_op.unwrap();
        let mut earned_cards = played_cards
            .iter()
            .enumerate()
            .filter(|(index, _)| *index != winning_index)
            .filter_map(|(_, played_card)| *played_card)
            .collect();
        let winning_deck = &mut decks[winning_index];
        let winning_card = played_cards[winning_index].unwrap();

        winning_deck.push_back(winning_card);
        winning_deck.append(&mut earned_cards);
    }
    let winning_deck = decks.iter().find(|deck| !deck.is_empty()).unwrap();
    winning_deck.iter().rev().enumerate().map(|(index, card)| *card * (index as u64 + 1)).sum()
}

fn solve_part_1(content: &str) {
    let decks = parse_decks(content);
    let winning_deck_score = get_winning_deck_score(decks);
    println!("Part 1: {}", winning_deck_score);
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
