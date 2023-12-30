use itertools::{Either, Itertools};
use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");
    let normal_trenches = parse_trenches(input);
    let lagoon_area = compute_lagoon_area(&normal_trenches);
    println!("Part 1: {}", lagoon_area);

    let color_trenches = normal_trenches
        .into_iter()
        .map(Trench::into_color)
        .collect_vec();
    let lagoon_area = compute_lagoon_area(&color_trenches);
    println!("Part 2: {}", lagoon_area);
}

fn compute_lagoon_area(trenches: &[Trench]) -> isize {
    let mut perimeter = 0;
    let mut twice_area = 0;
    let mut position = (0isize, 0isize);

    for trench in trenches {
        let direction_delta = trench.direction.delta();
        let next_position = (
            position.0 + trench.length * direction_delta.0,
            position.1 + trench.length * direction_delta.1,
        );

        perimeter += trench.length;
        twice_area += position.0 * next_position.1 - position.1 * next_position.0;

        position = next_position;
    }

    (twice_area.abs() - perimeter + 2) / 2 + perimeter
}

fn parse_trenches(input: &str) -> Vec<Trench> {
    input
        .lines()
        .map(|line| {
            let mut split = line.trim().split(' ');
            Trench {
                direction: split
                    .next()
                    .expect("trench direction not in line")
                    .parse()
                    .expect("trench direction is invalid"),
                length: split
                    .next()
                    .expect("trench length not in line")
                    .parse()
                    .expect("trench length is invalid"),
                color: split
                    .next()
                    .expect("trench color not in line")
                    .strip_prefix("(#")
                    .expect("color not enclosed in parentheses")
                    .strip_suffix(")")
                    .expect("color not enclosed in parentheses")
                    .to_owned(),
            }
        })
        .collect_vec()
}

#[derive(Debug, Clone)]
struct Trench {
    direction: Direction,
    length: isize,
    color: String,
}

impl Trench {
    fn into_color(self) -> Trench {
        let length = isize::from_str_radix(&self.color[..5], 16).unwrap();
        let direction = Direction::from_hex(&self.color[5..]).unwrap();
        Self {
            direction,
            length,
            ..self
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn delta(&self) -> (isize, isize) {
        match self {
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Up => (-1, 0),
        }
    }

    fn from_hex(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let c = s.parse::<char>().map_err(|e| Either::Left(e))?;
        match c {
            '0' => Ok(Direction::Right),
            '1' => Ok(Direction::Down),
            '2' => Ok(Direction::Left),
            '3' => Ok(Direction::Up),
            _ => Err(Either::Right("unrecognized hex char for direction")),
        }
    }
}

impl FromStr for Direction {
    type Err = Either<<char as FromStr>::Err, &'static str>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = s.parse::<char>().map_err(|e| Either::Left(e))?;
        match c {
            'R' => Ok(Direction::Right),
            'D' => Ok(Direction::Down),
            'L' => Ok(Direction::Left),
            'U' => Ok(Direction::Up),
            _ => Err(Either::Right("unrecognized char for direction")),
        }
    }
}
