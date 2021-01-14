use std::env;
use std::{collections::HashSet, fs, ops::Add};

use itertools::Itertools;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point(isize, isize);

type Set = HashSet<Point>;

impl Point {
    fn distance(&self) -> usize {
        (isize::abs(self.0)) as usize + (isize::abs(self.1)) as usize
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn parse_path(line: &str) -> impl Iterator<Item = Point> + '_ {
    line.split(',')
        .map(|s| (s.chars().next().unwrap(), &s[1..]))
        .map(|(c, s)| (c, s.parse::<isize>().unwrap()))
        .map(|(c, n)| {
            (1..=n).map(match c {
                'R' => |c: isize| Point(0, c),
                'D' => |r: isize| Point(r, 0),
                'L' => |c: isize| Point(0, -c),
                'U' => |r: isize| Point(-r, 0),
                _ => unreachable!(),
            })
        })
        .scan(Point(0, 0), |point, m| {
            let s = *point;
            let res = m.map(|p| p + s).collect_vec();
            *point = *res.last().unwrap();
            Some(res)
        })
        .flatten()
}

fn parse_content(
    content: &str,
) -> impl Iterator<Item = impl Iterator<Item = Point> + '_> + '_ {
    content.lines().map(crate::parse_path)
}

fn solve_part_1(content: &str) {
    let set_vec = parse_content(content)
        .map(|v| v.collect::<Set>())
        .collect_vec();

    let min_distance = set_vec[0]
        .intersection(&set_vec[1])
        .map(Point::distance)
        .min()
        .unwrap();

    println!("Part 1: {}", min_distance)
}

fn count_steps(path: &[Point], point: &Point) -> usize {
    path.iter().position(|p| p == point).unwrap() + 1
}

fn solve_part_2(content: &str) {
    let path_vec = parse_content(content)
        .map(|i| i.collect_vec())
        .collect_vec();

    let set_vec = path_vec
        .iter()
        .map(|v| v.iter().copied().collect::<Set>())
        .collect_vec();

    let min_steps = set_vec[0]
        .intersection(&set_vec[1])
        .map(|point| {
            count_steps(&path_vec[0], point) + count_steps(&path_vec[1], point)
        })
        .min()
        .unwrap();

    println!("Part 2: {}", min_steps)
}

fn main() {
    let args = env::args().collect_vec();
    let filename = args.get(1).map(|s| s.as_ref()).unwrap_or("./res/input.txt");

    let content = fs::read_to_string(filename).unwrap();

    solve_part_1(&content);
    solve_part_2(&content);
}

#[cfg(test)]
mod tests {}
