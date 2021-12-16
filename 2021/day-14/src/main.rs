use std::{collections::HashMap, str::FromStr};

type Element = u8;
type ElementPair = (Element, Element);

#[derive(Clone, Debug)]
struct Polymer {
    count_per_pair: HashMap<ElementPair, u64>,
    first_element: Element,
    last_element: Element,
}

impl FromStr for Polymer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        if let Some(&first_element) = bytes.first() {
            if let Some(&last_element) = bytes.last() {
                let mut count_per_pair = HashMap::new();
                for pair in bytes.windows(2) {
                    let key = (pair[0], pair[1]);
                    *count_per_pair.entry(key).or_default() += 1;
                }

                Ok(Self {
                    first_element,
                    last_element,
                    count_per_pair,
                })
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

impl Polymer {
    fn from_other(other: &Polymer) -> Self {
        Self {
            count_per_pair: Default::default(),
            ..*other
        }
    }
}

type Rules = HashMap<(Element, Element), Element>;

fn parse(s: &str) -> (Polymer, Rules) {
    let (polymer_str, rules_str) = s.split_once("\n\n").unwrap();
    let polymer = polymer_str.parse().unwrap();
    let rules = rules_str
        .trim()
        .lines()
        .map(|line| {
            let (inputs_str, output_str) = line.split_once(" -> ").unwrap();
            let inputs_bytes = inputs_str.as_bytes();
            ((inputs_bytes[0], inputs_bytes[1]), output_str.as_bytes()[0])
        })
        .collect();

    (polymer, rules)
}

fn step(polymer: Polymer, rules: &Rules) -> Polymer {
    let mut next_polymer = Polymer::from_other(&polymer);

    for (pair, &count) in polymer.count_per_pair.iter() {
        if let Some(&next_middle_element) = rules.get(pair) {
            let &(left_element, right_element) = pair;
            let next_left_pair = (left_element, next_middle_element);
            let next_right_pair = (next_middle_element, right_element);
            *next_polymer
                .count_per_pair
                .entry(next_left_pair)
                .or_default() += count;
            *next_polymer
                .count_per_pair
                .entry(next_right_pair)
                .or_default() += count;
        }
    }

    next_polymer
}

fn step_multiple(initial_polymer: Polymer, rules: &Rules, number_steps: usize) -> Polymer {
    (0..number_steps).fold(initial_polymer, |p, _| step(p, rules))
}

fn get_element_histogram(polymer: &Polymer) -> HashMap<Element, u64> {
    let mut element_histogram: HashMap<Element, u64> = HashMap::new();
    for (pair, count) in &polymer.count_per_pair {
        *element_histogram.entry(pair.0).or_default() += count;
        *element_histogram.entry(pair.1).or_default() += count;
    }

    element_histogram
}

fn solve(initial_polymer: Polymer, rules: &Rules, number_steps: usize) -> u64 {
    let final_polymer = step_multiple(initial_polymer, rules, number_steps);

    let element_histogram = get_element_histogram(&final_polymer);

    let get_real_element_count = |(&element, &count)| {
        let mut real_count = count;
        if element == final_polymer.first_element {
            real_count += 1;
        }
        if element == final_polymer.last_element {
            real_count += 1;
        }
        real_count / 2
    };

    let (min_count, max_count) = element_histogram
        .iter()
        .map(get_real_element_count)
        .fold((u64::MAX, u64::MIN), |counts, element_count| {
            (counts.0.min(element_count), counts.1.max(element_count))
        });

    max_count - min_count
}

fn main() {
    let input = include_str!("./input.txt");
    let (polymer, rules) = parse(input);
    println!("Part 1: {}", solve(polymer.clone(), &rules, 10));
    println!("Part 2: {}", solve(polymer, &rules, 40));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        let input = include_str!("./example.txt");
        let (polymer, rules) = parse(input);
        assert_eq!(solve(polymer, &rules, 10), 1588);
    }

    #[test]
    fn example_part_2() {
        let input = include_str!("./example.txt");
        let (polymer, rules) = parse(input);
        assert_eq!(solve(polymer, &rules, 40), 2188189693529);
    }
}
