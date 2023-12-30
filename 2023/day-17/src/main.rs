use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use itertools::{Itertools};

fn main() {
    let input = include_str!("input.txt");
    let grid = input.lines().map(|line| line.trim().as_bytes().iter().map(|&byte| byte - b'0').collect_vec()).collect_vec();

    let minimum_heat_loss = find_minimum_heat_loss(&grid, 0, 3);
    println!("Part 1: {}", minimum_heat_loss);

    let minimum_heat_loss = find_minimum_heat_loss(&grid, 4, 10);
    println!("Part 2: {}", minimum_heat_loss);
}

fn find_minimum_heat_loss(
    grid: &[Vec<u8>],
    minimum_consecutive_moves: usize,
    maximum_consecutive_moves: usize,
) -> u64 {
    let mut crucibles = BinaryHeap::from([Crucible {
        heat_loss: 0,
        position: (0, 0),
        direction: Direction::Right,
        consecutive_moves: 0,
    }, Crucible {
        heat_loss: 0,
        position: (0, 0),
        direction: Direction::Down,
        consecutive_moves: 0,
    }]);

    let mut minimum_heat_loss = None;
    let factory_position = (grid.len() - 1, grid[0].len() - 1);
    let mut visited = HashSet::new();
    while let Some(crucible) = crucibles.pop() {
        if crucible.position == factory_position && minimum_consecutive_moves <= crucible.consecutive_moves {
            minimum_heat_loss = Some(crucible.heat_loss);
            break;
        }

        let possible_directions = [Direction::Up, Direction::Down, Direction::Left, Direction::Right]
            .into_iter()
            .filter(|&direction| {
                direction.opposite() != crucible.direction &&
                    if direction == crucible.direction {
                        crucible.consecutive_moves < maximum_consecutive_moves
                    } else {
                        minimum_consecutive_moves <= crucible.consecutive_moves
                    }
            });

        for possible_direction in possible_directions {
            let new_position = possible_direction.apply_to_position(crucible.position);
            if let Some(new_position) = new_position {
                let tile = grid.get(new_position.0).and_then(|row| row.get(new_position.1));
                if let Some(&tile) = tile {
                    let new_consecutive_moves = if possible_direction == crucible.direction {
                        crucible.consecutive_moves + 1
                    } else {
                        1
                    };
                    let visited_key = (new_position, possible_direction, new_consecutive_moves);
                    if !visited.contains(&visited_key) {
                        visited.insert(visited_key);
                        crucibles.push(Crucible {
                            heat_loss: crucible.heat_loss + tile as u64,
                            position: new_position,
                            direction: possible_direction,
                            consecutive_moves: new_consecutive_moves,
                        });
                    }
                }
            }
        }
    }

    minimum_heat_loss.unwrap()
}


#[derive(Debug, Clone, PartialEq, Eq)]
struct Crucible {
    heat_loss: u64,
    position: (usize, usize),
    direction: Direction,
    consecutive_moves: usize,
}

impl PartialOrd for Crucible {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Crucible {
    fn cmp(&self, other: &Self) -> Ordering {
        self.heat_loss.cmp(&other.heat_loss).reverse()
    }
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn apply_to_position(&self, position: (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Direction::Up => position.0.checked_sub(1).map(|row| (row, position.1)),
            Direction::Down => position.0.checked_add(1).map(|row| (row, position.1)),
            Direction::Left => position.1.checked_sub(1).map(|col| (position.0, col)),
            Direction::Right => position.1.checked_add(1).map(|col| (position.0, col)),
        }
    }
}

impl Direction {
    pub(crate) fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
