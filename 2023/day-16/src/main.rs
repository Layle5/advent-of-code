use itertools::Itertools;
use std::collections::HashMap;
use std::ops::Add;

fn main() {
    let input = include_str!("input.txt");
    let grid = input
        .lines()
        .map(|line| line.trim().chars().collect_vec())
        .collect_vec();

    let energized_tiles_count = run_beams(&grid, Beam::new(Position::ZERO, Direction::RIGHT));
    println!("Part 1: {}", energized_tiles_count);

    let number_rows = grid.len() as isize;
    let number_cols = grid[0].len() as isize;
    let last_row = number_rows - 1;
    let last_col = number_cols - 1;
    let initial_beams = (0..number_cols)
        .map(|col| Beam {
            direction: Direction::UP,
            position: Position::new(last_row, col),
        })
        .chain((0..number_cols).map(|col| Beam {
            direction: Direction::DOWN,
            position: Position::new(0, col),
        }))
        .chain((0..number_rows).map(|row| Beam {
            direction: Direction::LEFT,
            position: Position::new(row, last_col),
        }))
        .chain((0..number_rows).map(|row| Beam {
            direction: Direction::RIGHT,
            position: Position::new(row, 0),
        }))
        .collect_vec();

    let (_, best_energized_tiles_count) = initial_beams
        .into_iter()
        .map(|initial_beam| (initial_beam.clone(), run_beams(&grid, initial_beam)))
        .max_by_key(|&(_, energized_tiles_count)| energized_tiles_count)
        .expect("best beam not found");

    println!("Part 2: {}", best_energized_tiles_count);
}

fn run_beams(grid: &[Vec<char>], initial_beam: Beam) -> usize {
    let mut beams = vec![initial_beam];
    let mut visited_positions = HashMap::<Position, Vec<Direction>>::new();

    while let Some(beam) = beams.pop() {
        if let Some(current_tile) = grid
            .get(beam.position.row as usize)
            .and_then(|row| row.get(beam.position.col as usize))
        {
            let visited_directions = visited_positions.entry(beam.position).or_default();
            if !visited_directions.contains(&beam.direction) {
                visited_directions.push(beam.direction);

                match current_tile {
                    '.' => {
                        beams.push(Beam {
                            position: beam.position + beam.direction,
                            direction: beam.direction,
                        });
                    }
                    '/' => {
                        let new_direction = match beam.direction {
                            Direction::UP => Direction::RIGHT,
                            Direction::DOWN => Direction::LEFT,
                            Direction::LEFT => Direction::DOWN,
                            Direction::RIGHT => Direction::UP,
                        };
                        beams.push(Beam {
                            position: beam.position + new_direction,
                            direction: new_direction,
                        });
                    }
                    '\\' => {
                        let new_direction = match beam.direction {
                            Direction::UP => Direction::LEFT,
                            Direction::DOWN => Direction::RIGHT,
                            Direction::LEFT => Direction::UP,
                            Direction::RIGHT => Direction::DOWN,
                        };
                        beams.push(Beam {
                            position: beam.position + new_direction,
                            direction: new_direction,
                        });
                    }
                    '|' => match beam.direction {
                        Direction::UP | Direction::DOWN => {
                            beams.push(Beam {
                                position: beam.position + beam.direction,
                                direction: beam.direction,
                            });
                        }
                        Direction::LEFT | Direction::RIGHT => {
                            beams.push(Beam {
                                position: beam.position + Direction::UP,
                                direction: Direction::UP,
                            });
                            beams.push(Beam {
                                position: beam.position + Direction::DOWN,
                                direction: Direction::DOWN,
                            });
                        }
                    },
                    '-' => match beam.direction {
                        Direction::UP | Direction::DOWN => {
                            beams.push(Beam {
                                position: beam.position + Direction::LEFT,
                                direction: Direction::LEFT,
                            });
                            beams.push(Beam {
                                position: beam.position + Direction::RIGHT,
                                direction: Direction::RIGHT,
                            });
                        }
                        Direction::LEFT | Direction::RIGHT => {
                            beams.push(Beam {
                                position: beam.position + beam.direction,
                                direction: beam.direction,
                            });
                        }
                    },
                    tile => panic!("unrecognized tile {}", tile),
                }
            }
        }
    }

    visited_positions.len()
}

#[derive(Debug, Clone)]
struct Beam {
    position: Position,
    direction: Direction,
}

impl Beam {
    fn new(position: Position, direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Ord, Eq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn to_position(&self) -> Position {
        match self {
            Direction::UP => Position::UP,
            Direction::DOWN => Position::DOWN,
            Direction::LEFT => Position::LEFT,
            Direction::RIGHT => Position::RIGHT,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Position {
    row: isize,
    col: isize,
}

impl Position {
    const ZERO: Position = Position::new(0, 0);
    const UP: Position = Position::new(-1, 0);
    const DOWN: Position = Position::new(1, 0);
    const LEFT: Position = Position::new(0, -1);
    const RIGHT: Position = Position::new(0, 1);

    const fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }
}

impl Add<Direction> for Position {
    type Output = Position;

    fn add(self, rhs: Direction) -> Self::Output {
        let rhs = rhs.to_position();
        Position {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}
