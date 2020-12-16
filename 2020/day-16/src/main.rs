use std::fs;
use std::{collections::HashMap, env};

#[macro_use]
extern crate lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Range {
    from: usize,
    to: usize,
}

impl Range {
    fn new(from: usize, to: usize) -> Range {
        Range { from, to }
    }
}

#[derive(Debug)]
struct Rule<'a> {
    name: &'a str,
    ranges: Vec<Range>,
}

#[derive(Debug)]
struct Ticket {
    values: Vec<usize>,
}

#[derive(Debug)]
struct Input<'a> {
    rules: Vec<Rule<'a>>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

fn parse_rule(line: &str) -> Rule {
    lazy_static! {
        static ref REGEX: Regex =
            Regex::new(r"^(.*): (\d*)-(\d*) or (\d*)-(\d*)$").unwrap();
    }

    let captures = REGEX.captures(line).unwrap();
    let name = captures.get(1).unwrap().as_str();
    let range1_from = captures.get(2).unwrap().as_str().parse().unwrap();
    let range1_to = captures.get(3).unwrap().as_str().parse().unwrap();
    let range2_from = captures.get(4).unwrap().as_str().parse().unwrap();
    let range2_to = captures.get(5).unwrap().as_str().parse().unwrap();

    Rule {
        name,
        ranges: vec![
            Range::new(range1_from, range1_to),
            Range::new(range2_from, range2_to),
        ],
    }
}

fn parse_ticket(line: &str) -> Ticket {
    let values = line
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    Ticket { values }
}

fn parse(content: &str) -> Input {
    let lines: Vec<&str> = content.lines().collect();
    let paragraphs: Vec<&[&str]> = lines.split(|s| s.is_empty()).collect();

    let rules: Vec<Rule> = paragraphs[0]
        .iter()
        .copied()
        .map(crate::parse_rule)
        .collect();

    let my_ticket = parse_ticket(paragraphs[1][1]);

    let nearby_tickets = paragraphs[2]
        .iter()
        .skip(1)
        .copied()
        .map(crate::parse_ticket)
        .collect();

    Input {
        rules,
        my_ticket,
        nearby_tickets,
    }
}

fn is_valid(rule: &Rule, value: usize) -> bool {
    rule.ranges.iter().any(|r| r.from <= value && value <= r.to)
}

fn is_valid_for_any(rules: &[Rule], value: usize) -> bool {
    rules.iter().any(|r| is_valid(r, value))
}

fn solve_part_1(content: &str) {
    let input = parse(content);
    let r: usize = input
        .nearby_tickets
        .iter()
        .flat_map(|t| t.values.iter())
        .filter(|v| !is_valid_for_any(&input.rules, **v))
        .sum();

    println!("Part 1: {:?}", r)
}

fn find_possible_rules<'a, 'b>(
    rules: &'a [Rule<'b>],
    values: &[usize],
) -> Vec<&'a Rule<'b>> {
    rules
        .iter()
        .filter(|&r| values.iter().all(|&v| is_valid(r, v)))
        .collect()
}

fn solve_part_2(content: &str) {
    let input = parse(content);
    let remaining_tickets: Vec<&Ticket> = input
        .nearby_tickets
        .iter()
        .filter(|&t| {
            t.values.iter().all(|&v| is_valid_for_any(&input.rules, v))
        })
        .collect();

    let mut columns_map: HashMap<usize, Vec<&Rule>> =
        (0..remaining_tickets[0].values.len())
            .map(|column_index| {
                let values: Vec<usize> = remaining_tickets
                    .iter()
                    .map(|t| t.values[column_index])
                    .collect();

                let column_rules = find_possible_rules(&input.rules, &values);

                (column_index, column_rules)
            })
            .collect();

    let mut rule_names_map: HashMap<&str, usize> = HashMap::new();
    while let Some((column_index, column_rules)) =
        columns_map.iter_mut().find(|(_, v)| v.len() == 1)
    {
        let rule_name = column_rules[0].name;
        rule_names_map.insert(rule_name, *column_index);

        for column_rules in columns_map.values_mut() {
            if let Some(rule_index) =
                column_rules.iter().position(|x| x.name == rule_name)
            {
                column_rules.remove(rule_index);
            }
        }
    }

    let departure_product: usize = rule_names_map
        .into_iter()
        .filter(|(rule_name, _)| rule_name.starts_with("departure"))
        .map(|(_, values_index)| values_index)
        .map(|values_index| input.my_ticket.values[values_index])
        .product();

    println!("Part 2: {}", departure_product)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let default_filename: &str = "./res/input.txt";
    let filename: &str =
        args.get(1).map(|s| s.as_ref()).unwrap_or(default_filename);

    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    solve_part_1(&content);
    solve_part_2(&content);
}

#[cfg(test)]
mod tests {}
