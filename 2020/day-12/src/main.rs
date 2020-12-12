use std::env;
use std::fs;

extern crate num;
#[macro_use]
extern crate num_derive;

#[derive(Clone, Copy, Debug, FromPrimitive, PartialEq)]
enum Direction {
    East,
    South,
    West,
    North,
}

#[derive(Clone, Copy, Debug, Default)]
struct Position {
    north: isize,
    east: isize,
}

#[derive(Debug)]
struct Ship {
    position: Position,
    direction: Direction,
}

fn turn(direction: &Direction, number: isize) -> Direction {
    let direction_increment = ((((number / 90) % 4) + 4) % 4) as usize;
    let direction_index = *direction as usize;
    let new_direction_index = (direction_index + direction_increment) % 4;
    let new_direction = num::FromPrimitive::from_usize(new_direction_index);
    new_direction.unwrap()
}

fn move_ship(ship: &mut Ship, number: isize) {
    match ship.direction {
        Direction::East => ship.position.east += number,
        Direction::South => ship.position.north -= number,
        Direction::West => ship.position.east -= number,
        Direction::North => ship.position.north += number,
    }
}

fn parse_content<'a>(
    content: &'a str,
) -> impl Iterator<Item = (char, isize)> + 'a {
    content.lines().map(|line| {
        let operation_letter = line.chars().next().unwrap();
        let operation_number = (&line[1..]).parse().unwrap();
        (operation_letter, operation_number)
    })
}

fn rotate(position: &Position, number: isize) -> Position {
    let rotation_increment = (((number / 90) % 4) + 4) % 4;

    let mut new_position = *position;
    for _ in 0..rotation_increment {
        let prev_east = new_position.east;
        new_position.east = new_position.north;
        new_position.north = -prev_east;
    }

    new_position
}

fn move_ship_to(ship: &mut Ship, position: &Position, number: usize) {
    for _ in 0..number {
        ship.position.north += position.north;
        ship.position.east += position.east;
    }
}

fn solve_part_1(content: &str) {
    let mut ship = Ship {
        position: Position::default(),
        direction: Direction::East,
    };

    for (operation_letter, operation_number) in parse_content(content) {
        match operation_letter {
            'N' => ship.position.north += operation_number,
            'S' => ship.position.north -= operation_number,
            'E' => ship.position.east += operation_number,
            'W' => ship.position.east -= operation_number,
            'L' => ship.direction = turn(&ship.direction, -operation_number),
            'R' => ship.direction = turn(&ship.direction, operation_number),
            'F' => move_ship(&mut ship, operation_number),
            _ => panic!("Could not recognize {} operation", operation_letter),
        }
    }

    let distance =
        isize::abs(ship.position.north) + isize::abs(ship.position.east);
    println!("Part 1: {}", distance)
}

fn solve_part_2(content: &str) {
    let mut ship = Ship {
        position: Position::default(),
        direction: Direction::East,
    };
    let mut waypoint = Position { north: 1, east: 10 };

    for (letter, number) in parse_content(content) {
        match letter {
            'N' => waypoint.north += number,
            'S' => waypoint.north -= number,
            'E' => waypoint.east += number,
            'W' => waypoint.east -= number,
            'L' => waypoint = rotate(&waypoint, -number),
            'R' => waypoint = rotate(&waypoint, number),
            'F' => move_ship_to(&mut ship, &waypoint, number as usize),
            _ => panic!("Could not recognize {} operation", letter),
        }
    }

    let distance =
        isize::abs(ship.position.north) + isize::abs(ship.position.east);
    println!("Part 2: {}", distance)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let default_filename: &str = "./res/input.txt";
    let filename: &str =
        args.get(1).map(|s| s.as_ref()).unwrap_or(default_filename);

    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    solve_part_1(&content);
    solve_part_2(&content);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn turn_direction_test() {
        let assert = |d, n, e| {
            assert_eq!(turn(&d, n), e);
        };

        assert(Direction::East, 90, Direction::South);
        assert(Direction::East, 180, Direction::West);
        assert(Direction::East, 270, Direction::North);
        assert(Direction::East, -90, Direction::North);
        assert(Direction::East, -180, Direction::West);
        assert(Direction::East, -270, Direction::South);

        assert(Direction::South, 90, Direction::West);
        assert(Direction::South, 180, Direction::North);
        assert(Direction::South, 270, Direction::East);
        assert(Direction::South, -90, Direction::East);
        assert(Direction::South, -180, Direction::North);
        assert(Direction::South, -270, Direction::West);

        assert(Direction::West, 90, Direction::North);
        assert(Direction::West, 180, Direction::East);
        assert(Direction::West, 270, Direction::South);
        assert(Direction::West, -90, Direction::South);
        assert(Direction::West, -180, Direction::East);
        assert(Direction::West, -270, Direction::North);

        assert(Direction::North, 90, Direction::East);
        assert(Direction::North, 180, Direction::South);
        assert(Direction::North, 270, Direction::West);
        assert(Direction::North, -90, Direction::West);
        assert(Direction::North, -180, Direction::South);
        assert(Direction::North, -270, Direction::East);
    }
}
