mod computer;

#[macro_use]
extern crate num_derive;

use computer::{run, Int, Program, ProgramState};
use itertools::Itertools;
use std::{
    env,
    iter::{once, repeat_with},
};
use std::{fs, ops::Add};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Tile {
    Scaffold,
    Empty,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

impl From<u8> for Direction {
    fn from(byte: u8) -> Self {
        match byte {
            b'^' => Direction::Up,
            b'v' => Direction::Down,
            b'<' => Direction::Left,
            b'>' => Direction::Right,
            _ => panic!(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Position {
    row: isize,
    col: isize,
}

impl Position {
    fn new(row: isize, col: isize) -> Position {
        Position { row, col }
    }
}

impl From<Direction> for Position {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => Position::new(-1, 0),
            Direction::Down => Position::new(1, 0),
            Direction::Left => Position::new(0, -1),
            Direction::Right => Position::new(0, 1),
        }
    }
}

impl Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Position::new(self.row + rhs.row, self.col + rhs.col)
    }
}

#[derive(Debug)]
struct Grid {
    tiles: Vec<Vec<Tile>>,
}

impl Grid {
    fn height(&self) -> usize {
        self.tiles.len()
    }

    fn width(&self) -> usize {
        self.tiles.first().unwrap().len()
    }

    fn tiles(&self) -> impl Iterator<Item = (usize, usize, Tile)> + '_ {
        self.tiles.iter().enumerate().flat_map(|(row, line)| {
            line.iter()
                .enumerate()
                .map(move |(col, tile)| (row, col, *tile))
        })
    }

    fn neighbors(
        &self,
        row: usize,
        col: usize,
    ) -> impl Iterator<Item = Tile> + '_ {
        let irow = row as isize;
        let icol = col as isize;
        once((-1, 0))
            .chain(once((1, 0)))
            .chain(once((0, -1)))
            .chain(once((0, 1)))
            .filter(move |(delta_row, delta_col)| {
                irow >= -*delta_row && icol >= -*delta_col
            })
            .map(move |(delta_row, delta_col)| {
                ((irow + delta_row), (icol + delta_col))
            })
            .filter_map(move |(neighbor_row, neighbor_col)| {
                self.tiles
                    .get(neighbor_row as usize)
                    .and_then(|line| line.get(neighbor_col as usize))
                    .copied()
            })
    }

    fn is_scaffold(&self, position: &Position) -> bool {
        position.row >= 0
            && position.col >= 0
            && position.row < self.height() as isize
            && position.col < self.width() as isize
            && self
                .tiles
                .get(position.row as usize)
                .and_then(|line| line.get(position.col as usize))
                .map(|tile| *tile == Tile::Scaffold)
                .unwrap_or(false)
    }
}

#[derive(Clone, Debug)]
struct Robot {
    position: Position,
    direction: Direction,
}

fn parse_grid(content: &str) -> (Grid, Robot) {
    let mut program: Program = content.parse().unwrap();
    loop {
        let state = run(&mut program);
        match state {
            ProgramState::Output => {}
            _ => break,
        }
    }

    let mut position = None;
    let mut direction = None;

    let tiles = program
        .outputs
        .make_contiguous()
        .split(|output| *output == b'\n'.into())
        .enumerate()
        .map(|(row, output_part)| {
            output_part
                .iter()
                .enumerate()
                .map(|(col, output)| match *output as u8 {
                    b'#' => Tile::Scaffold,
                    b'.' => Tile::Empty,
                    byte => {
                        position =
                            Some(Position::new(row as isize, col as isize));
                        direction = Some(byte.into());
                        Tile::Scaffold
                    }
                })
                .collect_vec()
        })
        .collect_vec();

    let grid = Grid { tiles };

    let robot = Robot {
        position: position.unwrap(),
        direction: direction.unwrap(),
    };

    (grid, robot)
}

fn solve_part_1(content: &str) {
    let (grid, _) = parse_grid(content);

    let sum: usize = grid
        .tiles()
        .filter(|(_, _, tile)| *tile == Tile::Scaffold)
        .filter(|(row, col, _)| {
            grid.neighbors(*row, *col)
                .filter(|neighbor| *neighbor == Tile::Scaffold)
                .count()
                >= 3
        })
        .map(|(row, col, _)| row * col)
        .sum();

    println!("Part 1: {:?}", sum);
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Instruction {
    Left,
    Right,
    Forward(usize),
}

type Routine = Vec<Instruction>;
type RoutineSlice<'a> = &'a [Instruction];

fn find_routine(grid: &Grid, mut robot: Robot) -> Routine {
    let find_next_instruction = || -> Option<Instruction> {
        let mut next_instruction = None;

        let possibilities = [
            (robot.direction, Instruction::Forward(1)),
            (robot.direction.left(), Instruction::Left),
            (robot.direction.right(), Instruction::Right),
        ];

        for &(next_direction, instruction) in &possibilities {
            let delta: Position = (next_direction).into();
            let next_position: Position = robot.position + delta;
            if grid.is_scaffold(&next_position) {
                if next_direction == robot.direction {
                    robot.position = next_position;
                } else {
                    robot.direction = next_direction;
                }
                next_instruction = Some(instruction);
                break;
            }
        }

        next_instruction
    };

    repeat_with(find_next_instruction)
        .while_some()
        .collect_vec()
}

fn routine_to_string(routine: RoutineSlice) -> String {
    routine
        .iter()
        .copied()
        .coalesce(|l, r| match (l, r) {
            (Instruction::Forward(l), Instruction::Forward(r)) => {
                Ok(Instruction::Forward(l + r))
            }
            (_, _) => Err((l, r)),
        })
        .map(|instruction| match instruction {
            Instruction::Left => "L".to_string(),
            Instruction::Right => "R".to_string(),
            Instruction::Forward(n) => n.to_string(),
        })
        .join(",")
}

fn coalesce_routine(routine: Routine) -> Routine {
    routine
        .into_iter()
        .coalesce(|l, r| match (l, r) {
            (Instruction::Forward(ln), Instruction::Forward(rn)) => {
                Ok(Instruction::Forward(ln + rn))
            }
            (_, _) => Err((l, r)),
        })
        .collect_vec()
}

fn is_routine_valid(routine: &[Instruction]) -> bool {
    routine_to_string(routine).len() <= 20
}

type Routines = Vec<Routine>;

fn advance_routine_rec(
    instructions: RoutineSlice,
    routines: &[&Routine],
    routine_current_index: usize,
    valid_routine_indexes: Vec<usize>,
) -> Vec<usize> {
    if let Some(instruction) = instructions.first() {
        let next_valid_routine_indexes = valid_routine_indexes
            .into_iter()
            .filter(|valid_routine_index| {
                let valid_routine = routines[*valid_routine_index];
                let routine_instruction_op =
                    valid_routine.get(routine_current_index);
                if let Some(routine_instruction) = routine_instruction_op {
                    if *instruction == *routine_instruction {
                        return true;
                    }
                }
                false
            })
            .collect_vec();

        if next_valid_routine_indexes.is_empty() {
            return vec![];
        }

        let completed_routine_index_op = next_valid_routine_indexes
            .iter()
            .copied()
            .find(|valid_routine_index| {
                routine_current_index + 1
                    == routines[*valid_routine_index].len()
            });

        if let Some(completed_routine_index) = completed_routine_index_op {
            let mut rec = advance_routine_rec(
                &instructions[1..],
                routines,
                0,
                (0..routines.len()).collect_vec(),
            );

            rec.insert(0, completed_routine_index);
            rec
        } else {
            advance_routine_rec(
                &instructions[1..],
                routines,
                routine_current_index + 1,
                next_valid_routine_indexes,
            )
        }
    } else {
        vec![]
    }
}

fn advance_routine(
    instructions: RoutineSlice,
    routines: &[&Routine],
) -> Vec<usize> {
    advance_routine_rec(
        instructions,
        routines,
        0,
        (0..routines.len()).collect_vec(),
    )
}

fn split_routine(
    mut overall_routine: RoutineSlice,
    number: usize,
    current_sub_routines: Vec<&Routine>,
) -> Vec<Routines> {
    let indexes = advance_routine(overall_routine, &current_sub_routines);
    let mut start_index = 0;
    for &index in &indexes {
        start_index += current_sub_routines[index].len();
    }

    if start_index == overall_routine.len() && number == 0 {
        let main_routine_len = indexes.len() * 2 - 1;
        if 20 < main_routine_len {
            return vec![];
        }

        let r = current_sub_routines.into_iter().cloned().collect_vec();
        return vec![r];
    }

    if start_index == overall_routine.len() || number == 0 {
        return vec![];
    }

    if overall_routine.len() < number {
        return vec![];
    }

    overall_routine = &overall_routine[start_index..];

    let mut results = vec![];

    for end_index in 1..=overall_routine.len() - number + 1 {
        let sub_routine =
            overall_routine[0..end_index].iter().copied().collect_vec();

        if !is_routine_valid(&sub_routine) {
            continue;
        }

        let mut next_current_sub_routines =
            current_sub_routines.iter().copied().collect_vec();
        next_current_sub_routines.push(&sub_routine);
        let mut sub_routines_vec = split_routine(
            &overall_routine[end_index..],
            number - 1,
            next_current_sub_routines,
        );

        results.append(&mut sub_routines_vec);

        if !results.is_empty() {
            break;
        }
    }

    results
}

fn solve_part_2(content: &str) {
    let (grid, robot) = parse_grid(content);

    let total_routine = find_routine(&grid, robot);
    let total_routine = coalesce_routine(total_routine);

    let sub_routines_vec = split_routine(&total_routine, 3, vec![])
        .into_iter()
        .next()
        .unwrap();

    let sub_routines_ref = sub_routines_vec.iter().collect_vec();
    let main_routine_indexes =
        advance_routine(&total_routine, &sub_routines_ref);

    let main_routine = main_routine_indexes
        .into_iter()
        .map(|sub_routine_index| b'A' + sub_routine_index as u8)
        .map(|byte| byte as char)
        .intersperse(',')
        .collect();

    let sub_routines = sub_routines_vec.into_iter().map(|sub_routine_vec| {
        sub_routine_vec
            .into_iter()
            .map(|instruction| match instruction {
                Instruction::Left => "L".to_string(),
                Instruction::Right => "R".to_string(),
                Instruction::Forward(n) => n.to_string(),
            })
            .join(",")
    });

    let video_feed = "n".to_string();

    let input_string = once(main_routine)
        .chain(sub_routines)
        .chain(once(video_feed))
        .map(|mut s| {
            s.push('\n');
            s
        })
        .join("");

    let mut program: Program = content.parse().unwrap();
    program.memory[0] = 2;

    for byte in input_string.into_bytes() {
        program.input(byte as Int)
    }

    let mut state = ProgramState::Output;
    while state == ProgramState::Output {
        state = run(&mut program);
    }

    let dust_collected = program.outputs.back().unwrap();

    println!("Part 2: {}", dust_collected);
}

fn main() {
    let args = env::args().collect_vec();
    let filename = args.get(1).map(|s| s.as_ref()).unwrap_or("./res/input.txt");

    let content = fs::read_to_string(filename).unwrap();

    solve_part_1(&content);
    solve_part_2(&content);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn str_to_routine(s: &str) -> Routine {
        s.trim()
            .split(',')
            .flat_map(|s| match s {
                "L" => vec![Instruction::Left],
                "R" => vec![Instruction::Right],
                _ => {
                    let n: usize = s.parse().unwrap();
                    vec![Instruction::Forward(n); 1]
                }
            })
            .collect_vec()
    }

    #[test]
    fn test() {
        let instructions = str_to_routine(
            "R,8,R,8,R,4,R,4,R,8,L,6,L,2,R,4,R,4,R,8,R,8,R,8,L,6,L,2",
        );
        let a = str_to_routine("R,8,R,8");
        let b = str_to_routine("R,4,R,4,R,8");
        let c = str_to_routine("L,6,L,2");
        let routines = vec![&a, &b, &c];
        let r = advance_routine(&instructions, &routines);
        assert_eq!(r, vec![0, 1, 2, 1, 0, 2]);
    }
}
