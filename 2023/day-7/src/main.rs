use std::{collections::HashMap, fmt::Display, fmt::Write};

use itertools::Itertools;
use nom;

fn main() {
    let input = include_str!("input.txt");
    let hands = parse_hands(input);

    let total_winnings = compute_winnings(&hands, false);
    println!("Part 1: {}", total_winnings);

    let total_winnings = compute_winnings(&hands, true);
    println!("Part 2: {}", total_winnings);
}

fn compute_winnings(hands: &[Hand], consider_jokers: bool) -> u64 {
    let mut typed_hands = hands
        .iter()
        .map(|hand| (get_hand_type(hand, consider_jokers), hand))
        .collect_vec();

    typed_hands.sort_by(|lhs, rhs| {
        lhs.0.cmp(&rhs.0).then_with(|| {
            let get_card_value = |c: &Card| {
                if consider_jokers && c.value == Card::JACK_VALUE {
                    Card::JOKER_VALUE
                } else {
                    c.value
                }
            };

            lhs.1
                .cards
                .iter()
                .map(get_card_value)
                .cmp(rhs.1.cards.iter().map(get_card_value))
        })
    });

    let total_winnings = typed_hands
        .iter()
        .enumerate()
        .map(|(index, (_, hand))| (index + 1) as u64 * hand.bid)
        .sum::<u64>();

    total_winnings
}

fn parse_hands(input: &str) -> Vec<Hand> {
    let (_, hands) =
        nom::multi::separated_list1(nom::character::complete::line_ending, parse_hand)(input)
            .unwrap();
    hands
}

type NomError<'a> = nom::error::Error<&'a str>;
type NomResult<'a, T> = nom::IResult<&'a str, T, NomError<'a>>;

fn parse_hand(input: &str) -> NomResult<Hand> {
    let (input, (cards, _, bid)) = nom::sequence::tuple((
        nom::multi::many1(parse_card),
        nom::character::complete::space1,
        nom::character::complete::u64,
    ))(input)?;

    Ok((input, Hand::new(cards, bid)))
}

fn parse_card(input: &str) -> NomResult<Card> {
    let (input, parsed) = nom::character::complete::one_of("AKQJT98765432")(input)?;
    let value = match parsed {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        char => (char as u8 - b'0') as u64,
    };

    Ok((input, Card { value }))
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    bid: u64,
}

fn get_hand_type(hand: &Hand, consider_jokers: bool) -> HandType {
    let Hand { cards, .. } = hand;

    let mut joker_counts = 0;
    let mut card_counts = HashMap::<Card, usize>::new();
    for &card in cards {
        if consider_jokers && card.value == Card::JACK_VALUE {
            joker_counts += 1;
        } else {
            *card_counts.entry(card).or_default() += 1;
        }
    }

    let card_counts = card_counts
        .into_iter()
        .map(|(_, count)| count)
        .sorted_by(|lhs, rhs| lhs.cmp(rhs).reverse())
        .collect_vec();

    let five_for_jokers = 5usize.saturating_sub(joker_counts);
    let four_for_jokers = 4usize.saturating_sub(joker_counts);
    let three_for_jokers = 3usize.saturating_sub(joker_counts);
    let two_for_jokers = 2usize.saturating_sub(joker_counts);
    match card_counts[..] {
        [] if joker_counts == cards.len() => HandType::FiveOfAKind,
        [j] if j == five_for_jokers => HandType::FiveOfAKind,
        [j, ..] if j == four_for_jokers => HandType::FourOfAKind,
        [j, 2] if j == three_for_jokers => HandType::FullHouse,
        [j, ..] if j == three_for_jokers => HandType::ThreeOfAKind,
        [2, 2, ..] => HandType::TwoPairs,
        [j, ..] if j == two_for_jokers => HandType::OnePair,
        [1, ..] => HandType::HighCard,
        _ => panic!("unrecognized hand type {:?}", card_counts),
    }
}

impl Hand {
    fn new(cards: Vec<Card>, bid: u64) -> Self {
        Self { cards, bid }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for card in &self.cards {
            write!(f, "{}", card)?;
        }
        write!(f, " {}", self.bid)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Card {
    value: u64,
}

impl Card {
    const JACK_VALUE: u64 = 11;
    const JOKER_VALUE: u64 = 1;
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self.value {
            14 => 'A',
            13 => 'K',
            12 => 'Q',
            11 => 'J',
            10 => 'T',
            v => (v as u8 + b'0') as char,
        })
    }
}
