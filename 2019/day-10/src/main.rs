use itertools::Itertools;
use std::{collections::HashSet, env};
use std::{fs, ops::Rem};

type Location = (isize, isize);
type Map = HashSet<Location>;

fn parse_map(content: &str) -> Map {
    content
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.bytes()
                .enumerate()
                .filter(|(_, byte)| *byte == b'#')
                .map(move |(col, _)| (row as isize, col as isize))
        })
        .collect()
}

fn gcd<T>(lhs: T, rhs: T) -> T
where
    T: Copy + Default + PartialOrd + Rem<Output = T>,
{
    let default = T::default();
    if rhs == default {
        lhs
    } else {
        gcd(rhs, lhs % rhs)
    }
}

fn visible_rank(map: &Map, lhs: &Location, rhs: &Location) -> usize {
    if lhs == rhs {
        return 0;
    }

    let delta = (rhs.0 - lhs.0, rhs.1 - lhs.1);
    let gcd = gcd(isize::abs(delta.0), isize::abs(delta.1));
    if gcd == 1 {
        return 0;
    }

    let increment = (delta.0 / gcd, delta.1 / gcd);
    (1..)
        .map(|i| (lhs.0 + i * increment.0, lhs.1 + i * increment.1))
        .take_while(|location| location != rhs)
        .filter(|location| map.contains(&location))
        .count()
}

fn is_visible(map: &Map, lhs: &Location, rhs: &Location) -> bool {
    if lhs == rhs {
        return false;
    }

    let delta = (rhs.0 - lhs.0, rhs.1 - lhs.1);
    let gcd = gcd(isize::abs(delta.0), isize::abs(delta.1));
    if gcd == 1 {
        return true;
    }

    let increment = (delta.0 / gcd, delta.1 / gcd);
    (1..)
        .map(|i| (lhs.0 + i * increment.0, lhs.1 + i * increment.1))
        .take_while(|location| location != rhs)
        .all(|location| !map.contains(&location))
}

fn count_visible(map: &Map, station: &Location) -> usize {
    map.iter()
        .filter(|asteroid| is_visible(map, station, asteroid))
        .count()
}

fn solve_part_1(content: &str) {
    let map = parse_map(content);
    let visible_count = map
        .iter()
        .map(|location| count_visible(&map, location))
        .max()
        .unwrap();

    println!("Part 1: {}", visible_count)
}

fn angle(from: &Location, to: &Location) -> f64 {
    std::f64::consts::PI
        - ((to.1 - from.1) as f64).atan2((to.0 - from.0) as f64)
}

fn solve_part_2(content: &str) {
    let map = parse_map(content);
    let station = map
        .iter()
        .max_by_key(|location| count_visible(&map, location))
        .unwrap();

    let a = map
        .iter()
        .filter(|asteroid| station != *asteroid)
        .map(|asteroid| {
            (
                visible_rank(&map, station, asteroid),
                (angle(station, asteroid) * 1000000000f64) as i64,
                asteroid,
            )
        })
        .sorted()
        .map(|(_, _, asteroid)| asteroid)
        .collect_vec();

    let asteroid_200 = a[199];
    println!("Part 2: {}", asteroid_200.1 * 100 + asteroid_200.0)
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
    #[test]
    fn angle_test() {
        let a = |v: Location, e: f64| {
            println!("{:?} {:?}", v, angle(&(0, 0), &v));
            assert!((angle(&(0, 0), &v) - e).abs() < std::f64::EPSILON);
        };

        a((-1, 0), 0f64);
        a((0, 1), std::f64::consts::FRAC_PI_2);
        a((1, 0), std::f64::consts::PI);
        a((0, -1), std::f64::consts::PI + std::f64::consts::FRAC_PI_2);
    }
}
