use std::{collections::HashSet, fs};
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

fn can_game_continue(decks: &[Deck]) -> bool {
    decks.iter().all(|deck| !deck.is_empty())
}

fn pop_played_cards(decks: &mut Decks) -> Vec<u64> {
    decks
        .iter_mut()
        .map(|deck| deck.pop_front().unwrap())
        .collect_vec()
}

fn find_winning_index(played_cards: &[u64]) -> usize {
    played_cards
        .iter()
        .enumerate()
        .map(|(index, card)| (index, card))
        .max_by_key(|(_, card)| *card)
        .unwrap()
        .0
}

fn give_cards_to_winner(
    decks: &mut Decks,
    played_cards: Vec<u64>,
    winning_index: usize,
) {
    let mut earned_cards = played_cards
        .iter()
        .enumerate()
        .filter(|(index, _)| *index != winning_index)
        .map(|(_, played_card)| *played_card)
        .collect();

    let winning_deck = &mut decks[winning_index];
    let winning_card = played_cards[winning_index];

    winning_deck.push_back(winning_card);
    winning_deck.append(&mut earned_cards);
}

fn compute_deck_score(deck: &Deck) -> Score {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(index, card)| *card * (index as u64 + 1))
        .sum()
}

fn combat(mut decks: Decks) -> Score {
    let mut winning_index = 0;

    while can_game_continue(&decks) {
        let played_cards = pop_played_cards(&mut decks);
        winning_index = find_winning_index(&played_cards);
        give_cards_to_winner(&mut decks, played_cards, winning_index);
    }

    let winning_deck = &decks[winning_index];
    compute_deck_score(winning_deck)
}

fn solve_part_1(content: &str) {
    let decks = parse_decks(content);
    let score = combat(decks);
    println!("Part 1: {}", score);
}

fn can_play_recursive_game(decks: &[Deck], played_cards: &[u64]) -> bool {
    played_cards
        .iter()
        .enumerate()
        .all(|(index, card)| *card <= (decks[index].len()) as Card)
}

fn get_recursive_decks(decks: &[Deck], played_cards: &[u64]) -> Decks {
    decks
        .iter()
        .enumerate()
        .map(|(index, deck)| {
            deck.iter()
                .copied()
                .take(played_cards[index] as usize)
                .collect()
        })
        .collect()
}

fn recursive_combat(mut decks: Decks) -> (usize, Score) {
    let mut previous_configurations: HashSet<Decks> = HashSet::new();
    let mut winning_index = 0;

    while decks.iter().all(|deck| !deck.is_empty()) {
        if previous_configurations.contains(&decks) {
            winning_index = 0;
            break;
        }

        previous_configurations.insert(decks.clone());

        let played_cards = pop_played_cards(&mut decks);

        winning_index = if can_play_recursive_game(&decks, &played_cards) {
            let recursive_decks = get_recursive_decks(&decks, &played_cards);
            recursive_combat(recursive_decks).0
        } else {
            find_winning_index(&played_cards)
        };

        give_cards_to_winner(&mut decks, played_cards, winning_index);
    }

    let winning_deck = &decks[winning_index];
    let winning_score = compute_deck_score(winning_deck);
    (winning_index, winning_score)
}

fn solve_part_2(content: &str) {
    let decks = parse_decks(content);
    let (_, score) = recursive_combat(decks);
    println!("Part 2: {}", score);
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
