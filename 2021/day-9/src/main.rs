use std::str::FromStr;

#[derive(Debug)]
struct Position {
    row_index: usize,
    col_index: usize,
}

#[derive(Debug)]
struct Map {
    points: Vec<Vec<u8>>,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points = s
            .trim()
            .lines()
            .map(|line| line.as_bytes().iter().map(|byte| byte - b'0').collect())
            .collect::<Vec<_>>();

        Ok(Self { points })
    }
}

impl Map {
    fn get(&self, position: &Position) -> Option<u8> {
        self.points
            .get(position.row_index)
            .and_then(|row| row.get(position.col_index))
            .copied()
    }
}

impl Map {
    fn width(&self) -> usize {
        self.points.first().unwrap().len()
    }
    fn height(&self) -> usize {
        self.points.len()
    }
    fn up(&self, position: &Position) -> Option<Position> {
        position.row_index.checked_sub(1).map(|row_index| Position {
            row_index,
            col_index: position.col_index,
        })
    }

    fn down(&self, position: &Position) -> Option<Position> {
        position.row_index.checked_add(1).and_then(|row_index| {
            if row_index < self.height() {
                Some(Position {
                    row_index,
                    col_index: position.col_index,
                })
            } else {
                None
            }
        })
    }

    fn left(&self, position: &Position) -> Option<Position> {
        position.col_index.checked_sub(1).map(|col_index| Position {
            col_index,
            row_index: position.row_index,
        })
    }

    fn right(&self, position: &Position) -> Option<Position> {
        position.col_index.checked_add(1).and_then(|col_index| {
            if col_index < self.width() {
                Some(Position {
                    col_index,
                    row_index: position.row_index,
                })
            } else {
                None
            }
        })
    }

    fn iter_adjacent_positions(&self, position: &Position) -> impl Iterator<Item = Position> + '_ {
        [
            self.up(position),
            self.left(position),
            self.right(position),
            self.down(position),
        ]
        .into_iter()
        .flatten()
    }

    fn iter_adjacent(&self, position: &Position) -> impl Iterator<Item = u8> + '_ {
        self.iter_adjacent_positions(position)
            .filter_map(|adjacent| self.get(&adjacent))
    }

    fn is_low_point(&self, position: &Position) -> bool {
        let point = self.get(position).unwrap();
        self.iter_adjacent(position)
            .all(|adjacent| point < adjacent)
    }
}

fn solve_part_1(map: &Map) -> u64 {
    let mut risk_level = 0;
    for row_index in 0..map.height() {
        for col_index in 0..map.width() {
            let position = Position {
                row_index,
                col_index,
            };
            if map.is_low_point(&position) {
                let point = map.get(&position).unwrap();
                risk_level += point as u64 + 1;
            }
        }
    }
    risk_level
}

fn fill_basin(map: &Map, marks: &mut Vec<Vec<bool>>, position: &Position) -> u64 {
    let mark = marks
        .get_mut(position.row_index)
        .unwrap()
        .get_mut(position.col_index)
        .unwrap();

    if *mark {
        return 0;
    }
    *mark = true;

    if let None | Some(9) = map.get(position) {
        return 0;
    };

    map.iter_adjacent_positions(position)
        .map(|adjacent| fill_basin(map, marks, &adjacent))
        .sum::<u64>()
        + 1
}

fn add_basin_size(largest_basin_sizes: &mut [u64], current_basin_size: u64) {
    let pos_option = largest_basin_sizes
        .iter()
        .position(|size| current_basin_size <= *size);

    if let Some(pos) = pos_option {
        if pos > 0 {
            largest_basin_sizes[pos - 1] = current_basin_size;
        }
    } else {
        largest_basin_sizes.rotate_left(1);
        *largest_basin_sizes.last_mut().unwrap() = current_basin_size;
    }
}

fn solve_part_2(map: &Map) -> u64 {
    let mut largest_basin_sizes = [0, 0, 0];
    let mut marks: Vec<Vec<bool>> = vec![vec![false; map.width()]; map.height()];
    for row_index in 0..map.height() {
        for col_index in 0..map.width() {
            let position = Position {
                row_index,
                col_index,
            };

            let current_basin_size = fill_basin(map, &mut marks, &position);

            add_basin_size(&mut largest_basin_sizes, current_basin_size);
        }
    }

    largest_basin_sizes
        .into_iter()
        .try_fold(1, u64::checked_mul)
        .unwrap()
}

fn main() {
    let input = include_str!("./input.txt");
    let map = Map::from_str(input).unwrap();
    println!("Part 1: {}", solve_part_1(&map));
    println!("Part 2: {}", solve_part_2(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        let input = include_str!("./example.txt");
        let map = Map::from_str(input).unwrap();
        assert_eq!(solve_part_1(&map), 15);
    }

    #[test]
    fn example_part_2() {
        let input = include_str!("./example.txt");
        let map = Map::from_str(input).unwrap();
        assert_eq!(solve_part_2(&map), 1134);
    }
}
