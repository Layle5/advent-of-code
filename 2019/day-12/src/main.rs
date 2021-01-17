use itertools::{FoldWhile, Itertools};
use regex::Regex;
use std::{
    collections::HashSet,
    env,
    iter::FromIterator,
    ops::{Add, Div, Mul, Rem},
};
use std::{fs, iter::Sum};

#[derive(Debug)]
struct Vector(isize, isize, isize);

struct VectorIntoIterator {
    vector: Vector,
    index: usize,
}

struct VectorIterator<'a> {
    vector: &'a Vector,
    index: usize,
}

impl Vector {
    fn iter(&self) -> VectorIterator {
        VectorIterator {
            vector: self,
            index: 0,
        }
    }

    fn absolute_sum(&self) -> isize {
        self.iter().map(isize::abs).sum()
    }
}

impl Default for Vector {
    fn default() -> Self {
        Vector(0, 0, 0)
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        (&self).add(&rhs)
    }
}

impl Add for &Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        self.iter().zip(rhs.iter()).map(|(a, b)| a + b).collect()
    }
}

impl Sum for Vector {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold1(|lhs, rhs| lhs.add(rhs)).unwrap_or_default()
    }
}

impl IntoIterator for Vector {
    type Item = isize;

    type IntoIter = VectorIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        VectorIntoIterator {
            vector: self,
            index: 0,
        }
    }
}

impl FromIterator<isize> for Vector {
    fn from_iter<T: IntoIterator<Item = isize>>(iter: T) -> Self {
        let mut i = iter.into_iter();
        let x = i.next().unwrap();
        let y = i.next().unwrap();
        let z = i.next().unwrap();
        Vector(x, y, z)
    }
}

impl<'a> Iterator for VectorIterator<'a> {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        let item = match self.index {
            0 => Some(self.vector.0),
            1 => Some(self.vector.1),
            2 => Some(self.vector.2),
            _ => return None,
        };
        self.index += 1;
        item
    }
}

impl Iterator for VectorIntoIterator {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        let item = match self.index {
            0 => Some(self.vector.0),
            1 => Some(self.vector.1),
            2 => Some(self.vector.2),
            _ => return None,
        };
        self.index += 1;
        item
    }
}

#[derive(Debug)]
struct Planet {
    position: Vector,
    velocity: Vector,
}

impl Planet {
    fn new(position: Vector) -> Planet {
        Planet {
            position,
            velocity: Vector(0, 0, 0),
        }
    }

    fn energy(&self) -> isize {
        self.position.absolute_sum() + self.velocity.absolute_sum()
    }
}

fn parse_planets(content: &str) -> Vec<Planet> {
    let regex = Regex::new(r"<x=(-?\d*), y=(-?\d*), z=(-?\d*)>").unwrap();
    content
        .lines()
        .map(|line| {
            let captures = regex.captures(line).unwrap();
            let parse = |i| captures.get(i).unwrap().as_str().parse().unwrap();
            Planet::new(Vector(parse(1), parse(2), parse(3)))
        })
        .collect_vec()
}

fn get_direction(from: &Vector, to: &Vector) -> Vector {
    from.iter()
        .zip(to.iter())
        .map(|(f, t)| t - f)
        .map(isize::signum)
        .collect::<Vector>()
}

fn step(planets: Vec<Planet>) -> Vec<Planet> {
    planets
        .iter()
        .map(|planet| {
            let velocity = &planet.velocity
                + &planets
                    .iter()
                    .filter(|other| !std::ptr::eq(planet, *other))
                    .map(|other| {
                        get_direction(&planet.position, &other.position)
                    })
                    .sum();
            let position = &planet.position + &velocity;
            Planet { position, velocity }
        })
        .collect_vec()
}

fn solve_part_1(content: &str) {
    let start_planets = parse_planets(content);
    let final_planets =
        (0..1000).fold(start_planets, |planets, _| step(planets));
    let energy: isize = final_planets.iter().map(Planet::energy).sum();
    println!("Part 1: {}", energy);
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

fn lcm<T>(lhs: T, rhs: T) -> T
where
    T: Copy
        + Default
        + PartialOrd
        + Rem<Output = T>
        + Mul<Output = T>
        + Div<Output = T>,
{
    (lhs * rhs) / gcd(lhs, rhs)
}

fn iterate<T, F>(init: T, mut f: F) -> T
where
    F: FnMut(T) -> FoldWhile<T>,
{
    let mut cur = init;
    loop {
        let res = f(cur);
        match res {
            FoldWhile::Done(res) => return res,
            FoldWhile::Continue(res) => cur = res,
        }
    }
}

fn solve_part_2(content: &str) {
    let start_planets = parse_planets(content);

    let mut sets = vec![HashSet::new(); 3];
    iterate(start_planets, |planets| {
        let planets = step(planets);

        let get_parts = |f: fn(&Vector) -> isize| {
            planets
                .iter()
                .map(|p| (f(&p.position), f(&p.velocity)))
                .collect_vec()
        };
        let xs = get_parts(|v| v.0);
        let ys = get_parts(|v| v.1);
        let zs = get_parts(|v| v.2);
        let ss = vec![xs, ys, zs];

        let was_partial_previous_state =
            ss.iter().zip(sets.iter()).all(|(s, set)| set.contains(s));

        if was_partial_previous_state {
            FoldWhile::Done(planets)
        } else {
            for (s, set) in ss.into_iter().zip(sets.iter_mut()) {
                set.insert(s);
            }

            FoldWhile::Continue(planets)
        }
    });

    let cycle_len = sets.into_iter().map(|s| s.len()).fold1(lcm).unwrap();
    println!("Part 2: {}", cycle_len)
}

fn main() {
    let args = env::args().collect_vec();
    let filename = args.get(1).map(|s| s.as_ref()).unwrap_or("./res/input.txt");

    let content = fs::read_to_string(filename).unwrap();

    solve_part_1(&content);
    solve_part_2(&content);
}
