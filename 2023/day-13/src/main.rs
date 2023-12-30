use itertools::{assert_equal, Itertools};
use std::arch::x86_64::__m128;

fn main() {
    let input = include_str!("example.txt");
    let patterns = parse_patterns(input);
    dbg!(patterns);
}

fn parse_patterns(input: &str) -> Vec<Pattern> {
    let mut parser = nom::multi::separated_list0(
        nom::multi::many1(nom::character::complete::line_ending::<&str, nom::error::Error<_>>),
        nom::multi::separated_list1(
            nom::character::complete::line_ending,
            nom::combinator::map(
                nom::multi::many1(nom::character::complete::one_of("#.")),
                String::from_iter,
            ),
        ),
    );

    let (_, patterns_rows) = parser(input).expect("patterns to be parsed correctly");

    patterns_rows
        .into_iter()
        .map(|pattern_rows| Pattern { rows: pattern_rows })
        .collect_vec()
}

#[derive(Debug, Clone)]
struct Pattern {
    rows: Vec<String>,
}
