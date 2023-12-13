use core::panic;
use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let records = parse_records(input);

    let total_possible_arrangements = records
        .iter()
        .map(|record| {
            let mut cache = Default::default();
            count_possible_arragements(
                &record.springs_conditions,
                &record.contiguous_counts,
                0,
                &vec![],
                &mut cache,
            )
        })
        .sum::<usize>();

    println!("Part 1: {total_possible_arrangements}");

    let total_possible_arrangements = records
        .iter()
        .map(|record| {
            let unfolded_springs_conditions = [
                record.springs_conditions.as_slice(),
                [None].as_slice(),
                record.springs_conditions.as_slice(),
                [None].as_slice(),
                record.springs_conditions.as_slice(),
                [None].as_slice(),
                record.springs_conditions.as_slice(),
                [None].as_slice(),
                record.springs_conditions.as_slice(),
            ]
            .into_iter()
            .flatten()
            .copied()
            .collect_vec();
            let unfolded_contiguous_counts = itertools::repeat_n(&record.contiguous_counts, 5)
                .flatten()
                .copied()
                .collect_vec();
            let mut cache = Default::default();
            count_possible_arragements(
                &unfolded_springs_conditions,
                &unfolded_contiguous_counts,
                0,
                &vec![],
                &mut cache,
            )
        })
        .sum::<usize>();

    println!("Part 2: {total_possible_arrangements}");
}

fn count_possible_arragements<'a, 'b, 'c>(
    springs_conditions: &'a [Option<Condition>],
    contiguous_counts: &'a [usize],
    current_contiguous_count: usize,
    branch: &'b [Condition],
    cache: &'c mut HashMap<(&'a [Option<Condition>], &'a [usize], usize), usize>,
) -> usize {
    let cache_key = (
        springs_conditions,
        contiguous_counts,
        current_contiguous_count,
    );
    if let Some(&cached_result) = cache.get(&cache_key) {
        return cached_result;
    }

    let pending_contiguous_count = contiguous_counts.first().copied().unwrap_or_default();
    let result = if current_contiguous_count > pending_contiguous_count {
        0
    } else {
        if let Some(&spring_condition) = springs_conditions.first() {
            let count_if_operational = |cache| {
                if current_contiguous_count == 0 {
                    let new_branch = branch
                        .iter()
                        .copied()
                        .chain([Condition::Operational])
                        .collect_vec();
                    count_possible_arragements(
                        &springs_conditions[1..],
                        &contiguous_counts[..],
                        current_contiguous_count,
                        &new_branch,
                        cache,
                    )
                } else if current_contiguous_count == pending_contiguous_count {
                    let new_branch = branch
                        .iter()
                        .copied()
                        .chain([Condition::Operational])
                        .collect_vec();
                    count_possible_arragements(
                        &springs_conditions[1..],
                        &contiguous_counts[1..],
                        0,
                        &new_branch,
                        cache,
                    )
                } else {
                    0
                }
            };

            let count_if_damaged = |cache| {
                if current_contiguous_count < pending_contiguous_count {
                    let new_branch = branch
                        .iter()
                        .copied()
                        .chain([Condition::Damaged])
                        .collect_vec();
                    count_possible_arragements(
                        &springs_conditions[1..],
                        contiguous_counts,
                        current_contiguous_count + 1,
                        &new_branch,
                        cache,
                    )
                } else {
                    0
                }
            };

            match spring_condition {
                Some(Condition::Operational) => count_if_operational(cache),
                Some(Condition::Damaged) => count_if_damaged(cache),
                None => {
                    let operational_count = count_if_operational(cache);
                    let damaged_count = count_if_damaged(cache);
                    operational_count + damaged_count
                }
            }
        } else if contiguous_counts.is_empty() {
            1
        } else if current_contiguous_count != pending_contiguous_count
            || contiguous_counts.len() > 1
        {
            0
        } else {
            1
        }
    };

    cache.insert(cache_key, result);

    result
}

fn parse_records(input: &str) -> Vec<Record> {
    input
        .lines()
        .map(|line| {
            let (springs_conditions_str, contiguous_counts_str) = line
                .split_once(' ')
                .expect("line to be in to space-separated parts");

            let springs_conditions = springs_conditions_str
                .as_bytes()
                .iter()
                .map(|&spring_condition_byte| match spring_condition_byte {
                    b'.' => Some(Condition::Operational),
                    b'#' => Some(Condition::Damaged),
                    b'?' => None,
                    b => panic!("unrecognized spring condition {b}"),
                })
                .collect_vec();

            let contiguous_counts = contiguous_counts_str
                .split(',')
                .map(|count_str| count_str.parse().expect("contiguous count to be valid"))
                .collect_vec();

            Record {
                springs_conditions,
                contiguous_counts,
            }
        })
        .collect_vec()
}

#[derive(Debug, Clone)]
struct Record {
    springs_conditions: Vec<Option<Condition>>,
    contiguous_counts: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Condition {
    Operational,
    Damaged,
}
