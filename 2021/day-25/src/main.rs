use std::fmt::Display;
use std::{collections::HashSet, str::FromStr};

use itertools::chain;
use nalgebra::point;
use nalgebra::{vector, Point2, Vector2};

type Point = (usize, usize);
type Direction = Vector2<usize>;
type Cucumber = Point;

#[derive(Debug)]
struct Herd {
    character: char,
    direction: Point,
    cucumbers: HashSet<Cucumber>,
}

impl Herd {
    fn east() -> Self {
        Self {
            character: '>',
            direction: (0, 1),
            cucumbers: Default::default(),
        }
    }

    fn south() -> Self {
        Self {
            character: 'v',
            direction: (1, 0),
            cucumbers: Default::default(),
        }
    }

    fn herds() -> Vec<Self> {
        vec![Self::east(), Self::south()]
    }
}

#[derive(Debug)]
struct Ocean {
    herds: Vec<Herd>,
    width: usize,
    height: usize,
}

impl FromStr for Ocean {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tuples = s.trim().lines().enumerate().flat_map(|(row, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(move |(col, char)| (char, row, col))
        });

        let mut herds = Herd::herds();
        let mut height = 0;
        let mut width = 0;

        for (char, row, col) in tuples {
            height = height.max(row + 1);
            width = width.max(col + 1);
            let herd_option = herds.iter_mut().find(|h| h.character == char);
            if let Some(herd) = herd_option {
                let cucumber = (row, col);
                herd.cucumbers.insert(cucumber);
            }
        }

        Ok(Self {
            herds,
            height,
            width,
        })
    }
}

impl Display for Ocean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let cucumber = (row, col);
                let herd_option = self.herds.iter().find(|h| h.cucumbers.contains(&cucumber));
                let character = match herd_option {
                    Some(herd) => herd.character,
                    None => '.',
                };
                write!(f, "{}", character)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn increment(current: usize, increment: usize, dimension: usize) -> usize {
    let next = current + increment;
    if next >= dimension {
        0
    } else {
        next
    }
}

fn get_next_cucumber(ocean: &Ocean, herd: &Herd, current: &Cucumber) -> Cucumber {
    (
        increment(current.0, herd.direction.0, ocean.height),
        increment(current.1, herd.direction.1, ocean.width),
    )
}

fn step(ocean: &Ocean) -> (Ocean, usize) {
    let mut next_herds = vec![];
    let mut r=0;
    for current_herd_index in 0..ocean.herds.len() {
        let current_herd = &ocean.herds[current_herd_index];
        let next_cucumbers = current_herd
            .cucumbers
            .iter()
            .copied()
            .map(|current_cucumber| {
                let next_cucumber = get_next_cucumber(ocean, current_herd, &current_cucumber);
                let can_move = chain![next_herds.iter(), ocean.herds[current_herd_index..].iter()]
                    .flat_map(|herd| herd.cucumbers.iter())
                    .all(|cucumber| *cucumber != next_cucumber);

                if can_move {
                    r += 1;
                    next_cucumber
                } else {
                    current_cucumber
                }
            })
            .collect();

        let next_herd = Herd {
            cucumbers: next_cucumbers,
            ..*current_herd
        };

        next_herds.push(next_herd);
    }

    let next_ocean = Ocean {
        herds: next_herds,
        ..*ocean
    };

    (next_ocean, r)
}

fn solve(mut ocean: Ocean) -> usize {
    for step_number in 1.. {
        let (next_ocean, count) = step(&ocean);
        ocean = next_ocean;
        if count == 0 {
            return step_number;
        }
    }

    panic!()
}

fn main() {
    let input = include_str!("./input.txt");
    let ocean = Ocean::from_str(input).unwrap();
    println!("This might take a while...");
    println!("Result: {}", solve(ocean));
}
