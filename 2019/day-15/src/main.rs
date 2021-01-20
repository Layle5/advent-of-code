mod computer;

#[macro_use]
extern crate num_derive;

use computer::{run, Program, ProgramState};
use itertools::Itertools;
use std::{
    cmp::max,
    collections::{HashMap, HashSet, VecDeque},
    env,
    fmt::Display,
    ops::{Add, Sub},
};
use std::{fs, iter};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Position {
    row: isize,
    col: isize,
}

impl Position {
    const NORTH: Position = Position::new(-1, 0);
    const SOUTH: Position = Position::new(1, 0);
    const WEST: Position = Position::new(0, -1);
    const EAST: Position = Position::new(0, 1);

    const fn new(row: isize, col: isize) -> Position {
        Position { row, col }
    }

    fn directions() -> impl Iterator<Item = Position> {
        iter::once(Position::new(-1, 0))
            .chain(iter::once(Position::new(1, 0)))
            .chain(iter::once(Position::new(0, -1)))
            .chain(iter::once(Position::new(0, 1)))
    }

    fn neighboring_positions<'a>(
        &'a self,
    ) -> impl Iterator<Item = Position> + 'a {
        Position::directions().map(move |direction| self + direction)
    }
}

impl Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Position::new(self.row + rhs.row, self.col + rhs.col)
    }
}

impl Add<Position> for &Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Position::new(self.row + rhs.row, self.col + rhs.col)
    }
}

impl Sub<&Position> for Position {
    type Output = Position;

    fn sub(self, rhs: &Position) -> Self::Output {
        Position::new(self.row - rhs.row, self.col - rhs.col)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Tile {
    Wall,
    Empty,
    Oxygen,
}

#[derive(Clone, Debug)]
struct Grid {
    tiles: HashMap<Position, Tile>,
    unknown_positions: HashSet<Position>,
    robot: Position,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            tiles: iter::once((Position::default(), Tile::Empty)).collect(),
            unknown_positions: Position::directions().collect(),
            robot: Position::default(),
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min_row = self.tiles.keys().map(|p| p.row).min().unwrap();
        let min_col = self.tiles.keys().map(|p| p.col).min().unwrap();
        let height =
            self.tiles.keys().map(|p| p.row).max().unwrap() - min_row + 1;
        let width =
            self.tiles.keys().map(|p| p.col).max().unwrap() - min_col + 1;

        let mut grid_str = vec![vec![' '; width as usize]; height as usize];

        for (position, tile) in &self.tiles {
            let c = match (*position, *tile) {
                (p, _) if p == self.robot => 'X',
                (Position { row: 0, col: 0 }, _) => 'S',
                (_, Tile::Wall) => '#',
                (_, Tile::Empty) => '.',
                (_, Tile::Oxygen) => 'O',
            };
            let row = position.row - min_row;
            let col = position.col - min_col;
            grid_str[row as usize][col as usize] = c;
        }
        let string = grid_str
            .into_iter()
            .map(|line| line.into_iter().collect::<String>())
            .join("\n");
        write!(f, "{}", string)?;
        Ok(())
    }
}

fn get_directions(
    grid: &Grid,
    start: &Position,
    destination: &Position,
    previous: Option<&Position>,
) -> Option<VecDeque<Position>> {
    if start == destination {
        return Some(VecDeque::new());
    }

    let directions = Position::directions().collect_vec();

    for direction in &directions {
        let neighbor = start + *direction;
        if *destination == neighbor {
            return Some(iter::once(*direction).collect());
        }
    }

    for direction in directions {
        let neighbor = start + direction;

        if previous.is_some() && neighbor == *previous.unwrap() {
            continue;
        }

        if !grid.tiles.contains_key(&neighbor)
            || grid.tiles[&neighbor] == Tile::Wall
        {
            continue;
        }

        let directions_op =
            get_directions(grid, &neighbor, destination, Some(start));
        if let Some(mut directions) = directions_op {
            directions.push_front(direction);
            return Some(directions);
        }
    }

    None
}

fn find_distance(grid: &Grid) -> Option<usize> {
    let start = Position::default();

    if grid.tiles.get(&start) == Some(&Tile::Oxygen) {
        return Some(0);
    }

    let mut visited: HashSet<_> = iter::once(start).collect();
    let mut queue: VecDeque<_> = iter::once((start, 0)).collect();

    while let Some((current, distance)) = queue.pop_front() {
        let neighbors = current
            .neighboring_positions()
            .filter(|p| grid.tiles.contains_key(p))
            .filter(|p| grid.tiles[p] != Tile::Wall);

        for neighbor in neighbors {
            if grid.tiles.get(&neighbor) == Some(&Tile::Oxygen) {
                return Some(distance + 1);
            }

            let already_visited = visited.contains(&neighbor);
            if !already_visited {
                visited.insert(neighbor);
                queue.push_back((neighbor, distance + 1));
            }
        }
    }

    None
}

fn move_robot(
    program: &mut Program,
    directions: &VecDeque<Position>,
) -> Option<Tile> {
    let mut tile_op = None;

    for direction in directions {
        let input = match *direction {
            Position::NORTH => 1,
            Position::SOUTH => 2,
            Position::WEST => 3,
            Position::EAST => 4,
            _ => panic!(),
        };
        program.input(input);
        let state = run(program);
        match state {
            ProgramState::Output => {
                let output = program.output().unwrap();
                tile_op = Some(match output {
                    0 => Tile::Wall,
                    1 => Tile::Empty,
                    2 => Tile::Oxygen,
                    _ => panic!(),
                });
            }
            _ => panic!(),
        }
    }

    tile_op
}

fn get_filled_grid(content: &str) -> Grid {
    let mut program: Program = content.parse().unwrap();
    let mut grid = Grid::new();

    while let Some(&next_unknown_position) =
        grid.unknown_positions.iter().next()
    {
        // println!("{}\n", grid);

        let directions =
            get_directions(&grid, &grid.robot, &next_unknown_position, None)
                .unwrap();

        let tile = move_robot(&mut program, &directions).unwrap();
        grid.tiles.insert(next_unknown_position, tile);
        grid.unknown_positions.remove(&next_unknown_position);
        grid.robot = match tile {
            Tile::Wall => next_unknown_position - directions.back().unwrap(),
            _ => next_unknown_position,
        };

        if tile != Tile::Wall {
            for neighbor in next_unknown_position.neighboring_positions() {
                if !grid.tiles.contains_key(&neighbor) {
                    grid.unknown_positions.insert(neighbor);
                }
            }
        }
    }

    grid
}

fn solve_part_1(content: &str) {
    let grid = get_filled_grid(content);
    let distance = find_distance(&grid);
    println!("Part 1: {}", distance.unwrap());
}

fn find_max_distance(grid: &Grid) -> usize {
    let oxygen = grid
        .tiles
        .iter()
        .find(|(_, t)| **t == Tile::Oxygen)
        .unwrap()
        .0;

    let mut visited: HashSet<_> = iter::once(*oxygen).collect();
    let mut queue: VecDeque<_> = iter::once((*oxygen, 0)).collect();

    let mut max_distance = 0;

    while let Some((current, distance)) = queue.pop_front() {
        max_distance = max(max_distance, distance);

        let neighbors = current
            .neighboring_positions()
            .filter(|p| grid.tiles.contains_key(p))
            .filter(|p| grid.tiles[p] != Tile::Wall);

        for neighbor in neighbors {
            let already_visited = visited.contains(&neighbor);
            if !already_visited {
                visited.insert(neighbor);
                queue.push_back((neighbor, distance + 1));
            }
        }
    }

    max_distance
}

fn solve_part_2(content: &str) {
    let grid = get_filled_grid(content);
    let distance = find_max_distance(&grid);
    println!("Part 2: {}", distance);
}

fn main() {
    let args = env::args().collect_vec();
    let filename = args.get(1).map(|s| s.as_ref()).unwrap_or("./res/input.txt");

    let content = fs::read_to_string(filename).unwrap();

    solve_part_1(&content);
    solve_part_2(&content);
}
