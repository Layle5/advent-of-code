use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

type Signal = Vec<u8>;

#[derive(Debug)]
struct Entry {
    patterns: Vec<Signal>,
    outputs: Vec<Signal>,
}

fn parse_signals(s: &str) -> Vec<Signal> {
    s.trim()
        .split(' ')
        .map(|signal_str| {
            let mut signal = signal_str.as_bytes().to_owned();
            signal.sort_unstable();
            signal
        })
        .collect()
}

impl FromStr for Entry {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (patterns_str, outputs_str) = input.split_once('|').ok_or("could not split line")?;
        let patterns = parse_signals(patterns_str);
        let outputs = parse_signals(outputs_str);
        Ok(Self { patterns, outputs })
    }
}

fn parse(input: &str) -> Vec<Entry> {
    input
        .lines()
        .map(Entry::from_str)
        .map(Result::unwrap)
        .collect()
}

fn solve_part_1(entries: &[Entry]) -> usize {
    entries
        .iter()
        .flat_map(|entry| &entry.outputs)
        .map(|signal| signal.len())
        .filter(|&signal_len| {
            signal_len == 2 || signal_len == 4 || signal_len == 3 || signal_len == 7
        })
        .count()
}

fn is_signal_contained_in(big_signal: &[u8], small_signal: &[u8]) -> bool {
    small_signal.iter().all(|b| big_signal.contains(b))
}

fn pop_pattern<'a, F>(pattern_set: &mut HashSet<&'a Signal>, predicate: F) -> &'a Signal
where
    F: Fn(&Signal) -> bool,
{
    let pattern = *pattern_set.iter().find(|p| predicate(p)).unwrap();
    pattern_set.remove(pattern);
    pattern
}

fn identify_entry_patterns(entry: &Entry) -> HashMap<&Signal, u8> {
    let mut pattern_set = entry.patterns.iter().collect::<HashSet<_>>();

    let pattern_1 = pop_pattern(&mut pattern_set, |p| p.len() == 2);
    let pattern_4 = pop_pattern(&mut pattern_set, |p| p.len() == 4);
    let pattern_7 = pop_pattern(&mut pattern_set, |p| p.len() == 3);
    let pattern_8 = pop_pattern(&mut pattern_set, |p| p.len() == 7);
    let pattern_3 = pop_pattern(&mut pattern_set, |p| {
        p.len() == 5 && is_signal_contained_in(p, pattern_1)
    });
    let pattern_9 = pop_pattern(&mut pattern_set, |p| {
        p.len() == 6 && is_signal_contained_in(p, pattern_3)
    });
    let pattern_6 = pop_pattern(&mut pattern_set, |p| {
        p.len() == 6 && !is_signal_contained_in(p, pattern_1)
    });
    let pattern_0 = pop_pattern(&mut pattern_set, |p| p.len() == 6);
    let pattern_5 = pop_pattern(&mut pattern_set, |p| {
        p.len() == 5 && is_signal_contained_in(pattern_6, p)
    });
    let pattern_2 = pop_pattern(&mut pattern_set, |_| true);

    [
        (pattern_0, 0),
        (pattern_1, 1),
        (pattern_2, 2),
        (pattern_3, 3),
        (pattern_4, 4),
        (pattern_5, 5),
        (pattern_6, 6),
        (pattern_7, 7),
        (pattern_8, 8),
        (pattern_9, 9),
    ]
    .into_iter()
    .collect::<HashMap<&Signal, u8>>()
}

fn compute_entry_output(entry: &Entry) -> u64 {
    entry
        .outputs
        .iter()
        .map(|output| {
            let found_patterns = identify_entry_patterns(entry);
            *found_patterns.get(output).unwrap()
        })
        .fold(0, |accumulate, output_number| {
            accumulate * 10 + (output_number as u64)
        })
}

fn solve_part_2(entries: &[Entry]) -> u64 {
    entries.iter().map(compute_entry_output).sum::<u64>()
}

fn main() {
    let input = include_str!("./input.txt");
    let entries = parse(input);
    println!("Part 1: {}", solve_part_1(&entries));
    println!("Part 2: {}", solve_part_2(&entries));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn large_example_part_1() {
        let input = include_str!("./example.txt");
        assert_eq!(solve_part_1(&parse(input)), 26);
    }

    #[test]
    fn small_example_part_2() {
        let input = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        assert_eq!(solve_part_2(&parse(input)), 5353);
    }

    #[test]
    fn large_example_part_2() {
        let input = include_str!("./example.txt");
        assert_eq!(solve_part_2(&parse(input)), 61229);
    }
}
