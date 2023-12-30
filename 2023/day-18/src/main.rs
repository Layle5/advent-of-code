use itertools::{Either, Itertools};
use std::str::FromStr;

const TILE_TERRAIN: char = '.';
const TILE_TRENCH: char = '#';

fn main() {
    let input = include_str!("example.txt");
    let trenches = parse_trenches(input);
    let (initial_position, (number_rows, number_cols)) = find_grid_dimensions(&trenches);
    let mut grid = vec![vec![TILE_TERRAIN; number_cols]; number_rows];
    dig_grid(&mut grid, &trenches, initial_position);
}

fn dig_grid(grid: &mut Vec<Vec<char>>, trenches: &Vec<Trench>, initial_position: (usize, usize)) {
    let mut digger_position = initial_position;
    grid[digger_position.0][digger_position.1] = TILE_TRENCH;

    for trench in trenches {
        for _ in 0..trench.length {
            let op = trench.direction.move_position(digger_position);
            if op.is_none() {
                dbg!("au secours");
            }

            digger_position = op.expect("digger moved in invalid space");

            let mut tile = grid
                .get_mut(digger_position.0)
                .and_then(|r| r.get_mut(digger_position.1))
                .expect("digger moved outside of grid");

            *tile = TILE_TRENCH;
        }
    }
}

fn find_grid_dimensions(trenches: &[Trench]) -> ((usize, usize), (usize, usize)) {
    let (minimums, _, maximums) = trenches.iter().fold(
        ((0isize, 0isize), (0isize, 0isize), (0isize, 0isize)),
        |ranges, trench| {
            let length = trench.length as isize;
            let new_position = match trench.direction {
                Direction::Up => (ranges.1 .0 - length, ranges.1 .1),
                Direction::Down => (ranges.1 .0 + length, ranges.1 .1),
                Direction::Left => (ranges.1 .0, ranges.1 .1 - length),
                Direction::Right => (ranges.1 .0, ranges.1 .1 + length),
            };
            let new_minimums = (
                ranges.0 .0.min(new_position.0),
                ranges.0 .1.min(new_position.1),
            );
            let new_maximums = (
                ranges.2 .0.max(new_position.0),
                ranges.2 .1.max(new_position.1),
            );
            (new_minimums, new_position, new_maximums)
        },
    );

    (
        (minimums.0.abs() as usize + 1, minimums.1.abs() as usize + 1),
        (
            (minimums.0.abs() + maximums.0) as usize + 3,
            (minimums.1.abs() + maximums.1) as usize + 3,
        ),
    )
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
                    .strip_prefix("(")
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
    length: usize,
    color: String,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn move_position(&self, position: (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Direction::Up => position.0.checked_sub(1).map(|r| (r, position.1)),
            Direction::Down => position.0.checked_add(1).map(|r| (r, position.1)),
            Direction::Left => position.1.checked_sub(1).map(|c| (position.0, c)),
            Direction::Right => position.1.checked_add(1).map(|c| (position.0, c)),
        }
    }
}

impl FromStr for Direction {
    type Err = Either<<char as FromStr>::Err, &'static str>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = s.parse::<char>().map_err(|e| Either::Left(e))?;
        match c {
            'U' => Ok(Direction::Up),
            'D' => Ok(Direction::Down),
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(Either::Right("unrecognized char for direction")),
        }
    }
}
