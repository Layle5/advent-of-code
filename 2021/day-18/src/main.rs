use std::{fmt::Display, ops::Add};

#[derive(Clone, Debug)]
enum Element {
    Regular(u64),
    Snailfish(Box<Number>),
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Regular(value) => write!(f, "{}", value)?,
            Element::Snailfish(number_box) => write!(f, "{}", number_box)?,
        }
        Ok(())
    }
}

impl Element {
    fn propagate_explosion_left(&mut self, explosion_value: u64) {
        match self {
            Element::Regular(value) => *value += explosion_value,
            Element::Snailfish(number_box) => {
                number_box.elements[0].propagate_explosion_left(explosion_value)
            }
        }
    }

    fn propagate_explosion_right(&mut self, explosion_value: u64) {
        match self {
            Element::Regular(value) => *value += explosion_value,
            Element::Snailfish(number_box) => {
                number_box.elements[1].propagate_explosion_right(explosion_value)
            }
        }
    }

    fn try_splitting(&mut self) -> bool {
        match self {
            Element::Regular(value) => {
                if *value >= 10 {
                    let mid = *value / 2;
                    let left_element = Element::Regular(mid);
                    let right_element = Element::Regular(*value - mid);
                    *self = Element::Snailfish(Box::new(Number {
                        elements: [left_element, right_element],
                    }));

                    true
                } else {
                    false
                }
            }
            Element::Snailfish(number) => number.try_splitting(),
        }
    }

    fn magnitude(&self) -> u64 {
        match self {
            Element::Regular(value) => *value,
            Element::Snailfish(number) => number.magnitude(),
        }
    }
}

#[derive(Clone, Debug)]
struct Number {
    elements: [Element; 2],
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.elements[0], self.elements[1])?;
        Ok(())
    }
}

struct Explosion {
    replaced_zero: bool,
    left_value: u64,
    right_value: u64,
}

impl Number {
    fn add_without_reducing(self, rhs: Self) -> Self {
        Number {
            elements: [
                Element::Snailfish(Box::new(self)),
                Element::Snailfish(Box::new(rhs)),
            ],
        }
    }

    fn reduce(&mut self) {
        while self.try_exploding() || self.try_splitting() {}
    }

    fn try_exploding(&mut self) -> bool {
        self.try_exploding_recursive(0).is_some()
    }

    fn try_exploding_recursive(&mut self, depth: usize) -> Option<Explosion> {
        if depth >= 4 {
            if let [Element::Regular(left_value), Element::Regular(right_value)] = self.elements {
                return Some(Explosion {
                    replaced_zero: false,
                    left_value,
                    right_value,
                });
            }
        }

        let left_explosion_option = match &mut self.elements[0] {
            Element::Regular(_) => None,
            Element::Snailfish(left_number) => left_number.try_exploding_recursive(depth + 1),
        };
        if let Some(left_explosion) = left_explosion_option {
            if !left_explosion.replaced_zero {
                self.elements[0] = Element::Regular(0);
            }
            if left_explosion.right_value > 0 {
                self.elements[1].propagate_explosion_left(left_explosion.right_value);
            }
            return Some(Explosion {
                replaced_zero: true,
                right_value: 0,
                ..left_explosion
            });
        }

        let right_explosion_option = match &mut self.elements[1] {
            Element::Regular(_) => None,
            Element::Snailfish(right_number) => right_number.try_exploding_recursive(depth + 1),
        };
        if let Some(right_explosion) = right_explosion_option {
            if !right_explosion.replaced_zero {
                self.elements[1] = Element::Regular(0);
            }
            if right_explosion.left_value > 0 {
                self.elements[0].propagate_explosion_right(right_explosion.left_value);
            }
            return Some(Explosion {
                replaced_zero: true,
                left_value: 0,
                ..right_explosion
            });
        }

        None
    }

    fn try_splitting(&mut self) -> bool {
        self.elements[0].try_splitting() || self.elements[1].try_splitting()
    }

    fn magnitude(&self) -> u64 {
        3 * self.elements[0].magnitude() + 2 * self.elements[1].magnitude()
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self.add_without_reducing(rhs);
        result.reduce();
        result
    }
}

#[derive(Debug)]
struct ParseContext<'a> {
    bytes: &'a [u8],
    index: usize,
}

impl<'a> Iterator for ParseContext<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let item_option = self.bytes.get(self.index).copied();
        if item_option.is_some() {
            self.index += 1;
        }

        item_option
    }
}

impl<'a> ParseContext<'a> {
    fn peek(&self) -> Option<<Self as Iterator>::Item> {
        self.bytes.get(self.index).copied()
    }
}

fn is_ascii_numeric(byte: u8) -> bool {
    (b'0'..=b'9').contains(&byte)
}

fn parse_element(context: &mut ParseContext) -> Element {
    let peeked_byte = context.peek();
    match peeked_byte {
        Some(b'[') => {
            let number = parse_number(context);
            Element::Snailfish(Box::from(number))
        }
        Some(byte) if is_ascii_numeric(byte) => {
            context.next();
            Element::Regular((byte - b'0') as u64)
        }
        _ => panic!("unrecognized first byte for snailfish element"),
    }
}

fn parse_number(context: &mut ParseContext) -> Number {
    if context.next() != Some(b'[') {
        panic!("could not recognize first snailfish opening byte");
    }

    let first_element = parse_element(context);
    if context.next() != Some(b',') {
        panic!("could not recognize comma byte after snailfish element");
    }

    let second_element = parse_element(context);
    if context.next() != Some(b']') {
        panic!("could not recognize closing byte after snailfish element");
    }

    let elements = [first_element, second_element];
    Number { elements }
}

fn parse(input: &str) -> Vec<Number> {
    input
        .lines()
        .map(|line| {
            let bytes = line.trim().as_bytes();
            let index = 0;
            let mut context = ParseContext { bytes, index };
            parse_number(&mut context)
        })
        .collect()
}

fn solve_part_1(numbers: Vec<Number>) -> u64 {
    let mut iter = numbers.into_iter();
    let first = iter.next().unwrap();
    let result: Number = iter.fold(first, |t, a| t + a);
    result.magnitude()
}

fn solve_part_2(numbers: Vec<Number>) -> u64 {
    let mut greatest_magnitude = 0;
    for lhs_index in 0..numbers.len() {
        for rhs_index in 0..numbers.len() {
            if lhs_index == rhs_index {
                continue;
            }
            let lhs_number = &numbers[lhs_index];
            let rhs_number = &numbers[rhs_index];
            let result_1 = lhs_number.clone() + rhs_number.clone();
            let result_2 = rhs_number.clone() + lhs_number.clone();
            greatest_magnitude = greatest_magnitude.max(result_1.magnitude());
            greatest_magnitude = greatest_magnitude.max(result_2.magnitude());
        }
    }

    greatest_magnitude
}

fn main() {
    let input = include_str!("./input.txt");
    let numbers = parse(input);
    println!("Part 1: {}", solve_part_1(numbers.clone()));
    println!("Part 2: {}", solve_part_2(numbers));
}
