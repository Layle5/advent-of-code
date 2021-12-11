use std::collections::HashMap;

enum Analysis {
    Complete,
    Incomplete(Vec<u8>),
    Corrupted(u8),
}

fn get_pair_byte(byte: u8) -> u8 {
    match byte {
        b'(' => b')',
        b'[' => b']',
        b'{' => b'}',
        b'<' => b'>',
        b')' => b'(',
        b']' => b'[',
        b'}' => b'{',
        b'>' => b'<',
        _ => panic!("unrecognized byte {}", byte),
    }
}

fn analyze(input: &str) -> Analysis {
    let mut openings = vec![];

    for &byte in input.as_bytes() {
        match byte {
            b'(' | b'[' | b'{' | b'<' => openings.push(byte),
            _ => {
                let opening_byte = get_pair_byte(byte);
                let last_opening_byte = openings.last().copied();
                if last_opening_byte != Some(opening_byte) {
                    return Analysis::Corrupted(byte);
                } else {
                    openings.pop();
                }
            }
        }
    }

    if openings.is_empty() {
        Analysis::Complete
    } else {
        Analysis::Incomplete(openings)
    }
}

fn analyze_input(input: &str) -> Vec<Analysis> {
    input.trim().lines().map(analyze).collect()
}

fn solve_part_1(analyses: &[Analysis]) -> u64 {
    let points_per_byte: HashMap<u8, u64> =
        HashMap::from([(b')', 3), (b']', 57), (b'}', 1197), (b'>', 25137)]);

    analyses
        .iter()
        .filter_map(|analysis| match analysis {
            Analysis::Corrupted(byte) => Some(byte),
            _ => None,
        })
        .map(|byte| points_per_byte.get(byte))
        .map(Option::unwrap)
        .copied()
        .sum()
}

fn solve_part_2(analyses: &[Analysis]) -> u64 {
    let points_per_byte: HashMap<u8, u64> =
        HashMap::from([(b')', 1), (b']', 2), (b'}', 3), (b'>', 4)]);

    let mut scores: Vec<u64> = analyses
        .iter()
        .filter_map(|analysis| match analysis {
            Analysis::Incomplete(openings) => Some(openings),
            _ => None,
        })
        .map(|openings| {
            openings
                .iter()
                .rev()
                .copied()
                .map(get_pair_byte)
                .map(|byte| points_per_byte.get(&byte))
                .map(Option::unwrap)
                .copied()
                .fold(0, |temp_score, byte_score| temp_score * 5 + byte_score)
        })
        .collect();

    scores.sort_unstable();

    scores[scores.len() / 2]
}

fn main() {
    let input = include_str!("./input.txt");
    let analyses = analyze_input(input);
    println!("Part 1: {}", solve_part_1(&analyses));
    println!("Part 2: {}", solve_part_2(&analyses));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        let input = include_str!("./example.txt");
        assert_eq!(solve_part_1(&analyze_input(input)), 26397);
    }

    #[test]
    fn example_part_2() {
        let input = include_str!("./example.txt");
        assert_eq!(solve_part_2(&analyze_input(input)), 288957);
    }
}
