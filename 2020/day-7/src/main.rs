use std::collections::HashMap;
use std::env;
use std::fs;

#[macro_use]
extern crate lazy_static;
use regex::Regex;

type BagMap<'a> = HashMap<&'a str, Vec<(usize, &'a str)>>;

fn parse_inner_bags(inner_bags_str: &str) -> Vec<(usize, &str)> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"^ ?(\d) (.*) bags?$").unwrap();
    }

    match inner_bags_str {
        "no other bags" => vec![],
        _ => inner_bags_str
            .split(',')
            .map(|inner_bag_str| {
                let captures = REGEX.captures(inner_bag_str).unwrap();
                let number_inner_bag_str = captures.get(1).unwrap().as_str();
                let number_inner_bag = number_inner_bag_str.parse().unwrap();
                let inner_bag_name = captures.get(2).unwrap().as_str();
                (number_inner_bag, inner_bag_name)
            })
            .collect(),
    }
}

fn parse_bag_line(line: &str) -> (&str, Vec<(usize, &str)>) {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(
            r"^(.*) bags contain (((, )?(\d) (.*) bags?)*|no other bags)\.$"
        )
        .unwrap();
    }

    let captures = REGEX.captures(line).unwrap();
    let outer_bag_name = captures.get(1).unwrap().as_str();
    let inner_bags_str = captures.get(2).unwrap().as_str();
    let inner_bags = parse_inner_bags(inner_bags_str);
    (outer_bag_name, inner_bags)
}

fn recurse<'a>(bag_name: &'a str, bag_map: &'a BagMap) -> usize {
    if let Some(next_bags) = bag_map.get(bag_name) {
        next_bags
            .iter()
            .map(|&(number, next_bag_name)| {
                number + number * recurse(next_bag_name, bag_map)
            })
            .sum()
    } else {
        0
    }
}

fn solve(content: &str) {
    let bag_map: HashMap<_, _> =
        content.lines().map(|line| parse_bag_line(line)).collect();

    let start_bag_name = "shiny gold";
    let count = recurse(start_bag_name, &bag_map);
    println!("{}", count)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let default_filename: &str = "./res/input.txt";
    let filename: &str =
        args.get(1).map(|s| s.as_ref()).unwrap_or(default_filename);

    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    solve(&content);
}

#[cfg(test)]
mod tests {}
