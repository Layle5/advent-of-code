use std::collections::{BinaryHeap, HashSet};

type RiskLevel = u64;
type Position = (usize, usize);
type Map = Vec<Vec<RiskLevel>>;

struct Cavern<'a> {
    width: usize,
    height: usize,
    repeat: usize,
    map: &'a Map,
}

fn parse_map(input: &str) -> Map {
    input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .as_bytes()
                .iter()
                .map(|&byte| (byte - b'0') as RiskLevel)
                .collect()
        })
        .collect()
}

impl<'a> Cavern<'a> {
    fn from_map(map: &'a Map, repeat: usize) -> Self {
        let height = map.len();
        let width = map.first().unwrap().len();

        Self {
            width,
            height,
            repeat,
            map,
        }
    }

    fn is_exit(&self, position: &Position) -> bool {
        position.0 == self.height * self.repeat - 1 && position.1 == self.width * self.repeat - 1
    }

    fn get(&self, position: &Position) -> Option<RiskLevel> {
        let risk_level_row = position.0 % self.height;
        let risk_level_col = position.1 % self.width;
        let repeat_row = position.0 / self.height;
        let repeat_col = position.1 / self.width;

        if self.repeat <= repeat_row || self.repeat <= repeat_col {
            None
        } else {
            self.map
                .get(risk_level_row)
                .and_then(|row| row.get(risk_level_col))
                .map(|&risk_level| {
                    (risk_level + repeat_col as RiskLevel + repeat_row as RiskLevel - 1) % 9 + 1
                })
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RiskLevelState {
    position: Position,
    total_risk: RiskLevel,
}

impl Ord for RiskLevelState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.total_risk.cmp(&other.total_risk).reverse()
    }
}

impl PartialOrd for RiskLevelState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn get_adjacent_risk_level_states(
    cavern: &Cavern,
    visited: &HashSet<Position>,
    state: &RiskLevelState,
) -> Vec<RiskLevelState> {
    let RiskLevelState {
        position,
        total_risk,
    } = state;

    let adjacent_positions = [
        (position.0.checked_sub(1), Some(position.1)),
        (Some(position.0), position.1.checked_sub(1)),
        (Some(position.0), position.1.checked_add(1)),
        (position.0.checked_add(1), Some(position.1)),
    ]
    .into_iter()
    .flat_map(|adjacent_option| match adjacent_option {
        (Some(row), Some(col)) => Some((row, col)),
        _ => None,
    });

    let adjacent_states: Vec<RiskLevelState> = adjacent_positions
        .filter(|adjacent_position| !visited.contains(adjacent_position))
        .flat_map(|adjacent_position| {
            cavern
                .get(&adjacent_position)
                .map(|risk_level| RiskLevelState {
                    position: adjacent_position,
                    total_risk: total_risk + risk_level,
                })
        })
        .collect();

    adjacent_states
}

fn solve(map: &Map, repeat: usize) -> RiskLevel {
    let cavern = Cavern::from_map(map, repeat);
    let mut visited: HashSet<Position> = HashSet::new();
    let mut heap: BinaryHeap<RiskLevelState> = BinaryHeap::new();

    visited.insert((0, 0));
    heap.push(RiskLevelState {
        position: (0, 0),
        total_risk: 0,
    });

    while let Some(state) = heap.pop() {
        if cavern.is_exit(&state.position) {
            return state.total_risk;
        }

        let adjacent_states = get_adjacent_risk_level_states(&cavern, &visited, &state);

        for adjacent_state in adjacent_states {
            visited.insert(adjacent_state.position);
            heap.push(adjacent_state);
        }
    }

    panic!("could not reach exit");
}

fn main() {
    let input = include_str!("./input.txt");
    let map = parse_map(input);
    println!("Part 1: {}", solve(&map, 1));
    println!("Part 2: {}", solve(&map, 5));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        let input = include_str!("./example.txt");
        let map = parse_map(input);
        assert_eq!(solve(&map, 1), 40);
    }

    #[test]
    fn example_part_2() {
        let input = include_str!("./example.txt");
        let map = parse_map(input);
        assert_eq!(solve(&map, 5), 315);
    }
}
