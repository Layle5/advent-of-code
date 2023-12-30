use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let patterns = parse_patterns(input);
    let summarized = summarize_patterns(&patterns, 0);
    println!("Part 1: {}", summarized);
    let summarized = summarize_patterns(&patterns, 1);
    println!("Part 2: {}", summarized);
}

fn summarize_patterns(patterns: &[Pattern], smudge_tolerance: usize) -> usize {
    patterns
        .iter()
        .map(
            |pattern| match summarize_pattern(pattern, smudge_tolerance) {
                Mirror::Row(row_index) => (row_index + 1) * 100,
                Mirror::Col(col_index) => col_index + 1,
            },
        )
        .sum::<usize>()
}

fn summarize_pattern(pattern: &Pattern, smudge_tolerance: usize) -> Mirror {
    let number_rows = pattern.rows.len();
    let number_cols = pattern.rows[0].len();

    for col_index in 0..number_cols - 1 {
        if is_mirrored(
            number_rows,
            number_cols,
            col_index,
            smudge_tolerance,
            |along_index, across_index| pattern.rows[along_index][across_index],
        ) {
            return Mirror::Col(col_index);
        }
    }

    for row_index in 0..number_rows - 1 {
        if is_mirrored(
            number_cols,
            number_rows,
            row_index,
            smudge_tolerance,
            |along_index, across_index| pattern.rows[across_index][along_index],
        ) {
            return Mirror::Row(row_index);
        }
    }

    panic!("mirror not found")
}

fn is_mirrored(
    number_along: usize,
    number_across: usize,
    across_index: usize,
    smudge_tolerance: usize,
    get_tile: impl Fn(usize, usize) -> char,
) -> bool {
    let number_differences = (0..number_along)
        .map(|along_index| {
            (0..=across_index)
                .rev()
                .zip(across_index + 1..number_across)
                .filter(|&(first_across_index, second_across_index)| {
                    get_tile(along_index, first_across_index)
                        != get_tile(along_index, second_across_index)
                })
                .count()
        })
        .sum::<usize>();

    smudge_tolerance == number_differences
}

enum Mirror {
    Row(usize),
    Col(usize),
}

fn parse_patterns(input: &str) -> Vec<Pattern> {
    let mut parser = nom::multi::separated_list0(
        nom::multi::many1(nom::character::complete::line_ending::<&str, nom::error::Error<_>>),
        nom::multi::separated_list1(
            nom::character::complete::line_ending,
            nom::multi::many1(nom::character::complete::one_of("#.")),
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
    rows: Vec<Vec<char>>,
}
