use itertools::Itertools;
use std::borrow::ToOwned;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");
    let (instructions, nodes) = parse_input(input);

    let default_number_steps = find_number_steps(
        Label::default_start(),
        &instructions,
        &nodes,
        &HashSet::from([Label::default_end()]),
    );
    println!("Part 1: {}", default_number_steps);

    let ends = nodes.keys().filter(|l| l.is_end()).cloned().collect();
    let combined_number_steps = nodes
        .keys()
        .filter(|l| l.is_start())
        .map(|start| find_number_steps(start.clone(), &instructions, &nodes, &ends))
        .reduce(num::integer::lcm)
        .unwrap();
    println!("Part 2: {}", combined_number_steps);
}

fn find_number_steps(
    start: Label,
    instructions: &[Instruction],
    nodes: &HashMap<Label, Node>,
    ends: &HashSet<Label>,
) -> usize {
    let mut remaining_instructions = instructions;
    let mut current_visit = Visit {
        distance: 0,
        label: start,
    };

    while !ends.contains(&current_visit.label) {
        if remaining_instructions.is_empty() {
            remaining_instructions = instructions;
        }

        let instruction = remaining_instructions.first().unwrap();
        remaining_instructions = &remaining_instructions[1..];

        if let Some(node) = nodes.get(&current_visit.label) {
            current_visit.distance += 1;
            current_visit.label = match instruction {
                Instruction::Left => node.left.clone(),
                Instruction::Right => node.right.clone(),
            };
        }
    }

    current_visit.distance
}

type NomError<'a> = nom::error::Error<&'a str>;

fn parse_input(input: &str) -> (Vec<Instruction>, HashMap<Label, Node>) {
    let (input, instructions_str) =
        nom::multi::many1(nom::character::complete::one_of::<&str, &str, NomError>(
            "LR",
        ))(input)
        .unwrap();
    let (input, _) =
        nom::multi::many1(nom::character::complete::line_ending::<&str, NomError>)(input).unwrap();
    let (_, nodes_tuple) = nom::multi::separated_list1(
        nom::character::complete::line_ending,
        nom::sequence::tuple((
            parse_label,
            nom::bytes::complete::tag(" = ("),
            parse_label,
            nom::bytes::complete::tag(", "),
            parse_label,
            nom::bytes::complete::tag(")"),
        )),
    )(input)
    .unwrap();

    let instructions = instructions_str
        .into_iter()
        .map(|c| match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("unrecognized instruction {}", c),
        })
        .collect_vec();

    let nodes = nodes_tuple
        .into_iter()
        .map(|tuple| {
            let (label, _, left, _, right, _) = tuple;
            (label.clone(), Node { label, left, right })
        })
        .collect();

    (instructions, nodes)
}

fn parse_label(input: &str) -> nom::IResult<&str, Label, NomError> {
    nom::character::complete::alphanumeric1(input).map(|r| {
        (
            r.0,
            Label {
                value: r.1.to_owned(),
            },
        )
    })
}

#[derive(Debug, Clone, Copy, Hash, Ord, PartialOrd, Eq, PartialEq)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Visit {
    distance: usize,
    label: Label,
}

impl PartialOrd for Visit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Visit {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance
            .cmp(&other.distance)
            .then(self.label.cmp(&other.label))
            .reverse()
    }
}

#[derive(Debug, Clone)]
struct Node {
    label: Label,
    left: Label,
    right: Label,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
struct Label {
    value: String,
}

impl Label {
    fn default_start() -> Label {
        Label {
            value: "AAA".to_owned(),
        }
    }

    fn is_start(&self) -> bool {
        self.value.chars().last() == Some('A')
    }

    fn default_end() -> Label {
        Label {
            value: "ZZZ".to_owned(),
        }
    }

    fn is_end(&self) -> bool {
        self.value.chars().last() == Some('Z')
    }
}
