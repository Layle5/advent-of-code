#![feature(str_split_once)]

use std::fs;
use std::{collections::HashMap, env};

#[derive(Clone, Debug)]
enum Rule {
    Letter(u8),
    And(Vec<usize>),
    Or(Vec<usize>, Vec<usize>),
}

fn parse_numbers<'a>(i: impl IntoIterator<Item = &'a str>) -> Vec<usize> {
    i.into_iter()
        .map(|n| n.parse())
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect()
}

fn parse_rules(lines: &[&str]) -> HashMap<usize, Rule> {
    let mut rules = HashMap::new();
    for line in lines {
        let (index_str, line_rest) = line.split_once(':').unwrap();
        let index: usize = index_str.parse().unwrap();
        let rule = if line_rest.contains('"') {
            Rule::Letter(line_rest.as_bytes()[2])
        } else if line_rest.contains('|') {
            let q: Vec<_> = line_rest.split(' ').collect();
            let mut indexes: Vec<Vec<usize>> = q
                .split(|line| *line == "|")
                .map(|p| parse_numbers(p.iter().copied()))
                .collect();
            Rule::Or(indexes.pop().unwrap(), indexes.pop().unwrap())
        } else {
            let indexes: Vec<_> = parse_numbers(line_rest.split(' '));
            Rule::And(indexes)
        };

        rules.insert(index, rule);
    }
    rules
}

fn parse_messages(lines: &[&str]) -> Vec<Vec<u8>> {
    lines.iter().map(|line| line.bytes().collect()).collect()
}

fn advance_multiple(
    rules: &HashMap<usize, Rule>,
    message: &[u8],
    rule_indexes: &[usize],
) -> Vec<usize> {
    let mut prev_message_indexes: Vec<usize> = vec![0];
    for rule_index in rule_indexes {
        let mut next_message_indexes = Vec::new();
        for message_index in prev_message_indexes {
            let results =
                advance(rules, &message[message_index..], *rule_index);
            for result in results {
                next_message_indexes.push(result + message_index);
            }
        }
        prev_message_indexes = next_message_indexes;
    }

    prev_message_indexes
}

fn advance(
    rules: &HashMap<usize, Rule>,
    message: &[u8],
    rule_index: usize,
) -> Vec<usize> {
    let rule = rules.get(&rule_index).unwrap();
    match rule {
        Rule::Letter(c) => {
            if message.first().filter(|l| c == *l).is_none() {
                vec![]
            } else {
                vec![1]
            }
        }
        Rule::And(inner_rule_indexes) => {
            advance_multiple(rules, message, inner_rule_indexes)
        }
        Rule::Or(left, right) => {
            let mut left_results = advance_multiple(rules, message, left);
            let mut right_results = advance_multiple(rules, message, right);
            left_results.append(&mut right_results);
            left_results
        }
    }
}

fn is_valid(rules: &HashMap<usize, Rule>, message: &[u8]) -> bool {
    advance(rules, message, 0).contains(&message.len())
}

fn solve(part: &str, content: &str) {
    let lines: Vec<_> = content.lines().collect();
    let paragraphs: Vec<_> = lines.split(|line| line.is_empty()).collect();
    let rules = parse_rules(paragraphs[0]);
    let messages = parse_messages(paragraphs[1]);

    let valid_count = messages
        .into_iter()
        .filter(|message| is_valid(&rules, message))
        .count();

    println!("{}: {}", part, valid_count);
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
    let content_part_1 = &get_content(1, "./res/input-1.txt");
    solve("Part 1", content_part_1);

    let content_part_2 = &get_content(2, "./res/input-2.txt");
    solve("Part 2", content_part_2);
}

#[cfg(test)]
mod tests {}
