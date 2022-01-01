use std::{fmt::Display, iter::repeat, str::FromStr};

use regex::Regex;

#[derive(Clone, Debug)]
struct Range {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
    z_min: isize,
    z_max: isize,
    on: bool,
}

#[derive(Clone, Copy, Debug, Default)]
struct Region {
    is_lit: bool,
}

#[derive(Debug)]
struct Reactor {
    x_regions: Vec<isize>,
    y_regions: Vec<isize>,
    z_regions: Vec<isize>,
    regions: Vec<Region>,
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex =
            Regex::new(r"(\w+) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)")
                .unwrap();

        let captures = regex.captures(s).unwrap();
        let mut matches_str = captures.iter().skip(1).map(|o| o.unwrap().as_str());
        let on = matches_str.next().unwrap() == "on";

        let mut matches_numbers = matches_str.map(|s| s.parse().unwrap());
        let x_min: isize = matches_numbers.next().unwrap();
        let x_max: isize = matches_numbers.next().unwrap();
        let y_min: isize = matches_numbers.next().unwrap();
        let y_max: isize = matches_numbers.next().unwrap();
        let z_min: isize = matches_numbers.next().unwrap();
        let z_max: isize = matches_numbers.next().unwrap();

        Ok(Self {
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
            on,
        })
    }
}

fn parse(input: &str) -> Vec<Range> {
    input.trim().lines().flat_map(Range::from_str).collect()
}

impl Reactor {
    fn from_ranges(ranges: &[Range]) -> Self {
        fn insert_sorted(v: &mut Vec<isize>, n: isize) {
            match v.binary_search(&n) {
                Ok(_) => {}
                Err(i) => v.insert(i, n),
            }
        }

        let mut x_regions = vec![];
        let mut y_regions = vec![];
        let mut z_regions = vec![];
        for range in ranges {
            insert_sorted(&mut x_regions, range.x_min);
            insert_sorted(&mut x_regions, range.x_max + 1);
            insert_sorted(&mut y_regions, range.y_min);
            insert_sorted(&mut y_regions, range.y_max + 1);
            insert_sorted(&mut z_regions, range.z_min);
            insert_sorted(&mut z_regions, range.z_max + 1);
        }

        let regions = vec![Region::default(); x_regions.len() * y_regions.len() * z_regions.len()];

        Self {
            x_regions,
            y_regions,
            z_regions,
            regions,
        }
    }
}

fn get_region_index(reactor: &Reactor, tuple: (usize, usize, usize)) -> usize {
    let x = tuple.0;
    let y = tuple.1;
    let z = tuple.2;
    let sy = reactor.y_regions.len();
    let sz = reactor.z_regions.len();
    (x * sy + y) * sz + z
}

fn get_region(reactor: &Reactor, tuple: (usize, usize, usize)) -> &Region {
    &reactor.regions[get_region_index(reactor, tuple)]
}

fn get_region_mut(reactor: &mut Reactor, tuple: (usize, usize, usize)) -> &mut Region {
    let index = get_region_index(reactor, tuple);
    &mut reactor.regions[index]
}

fn iter_regions_indexes<'a>(
    reactor: &'a Reactor,
    range: &'a Range,
) -> impl Iterator<Item = (usize, usize, usize)> {
    let x_index_min = reactor.x_regions.binary_search(&range.x_min).unwrap();
    let x_index_max = reactor.x_regions.binary_search(&(range.x_max + 1)).unwrap();
    let y_index_min = reactor.y_regions.binary_search(&range.y_min).unwrap();
    let y_index_max = reactor.y_regions.binary_search(&(range.y_max + 1)).unwrap();
    let z_index_min = reactor.z_regions.binary_search(&range.z_min).unwrap();
    let z_index_max = reactor.z_regions.binary_search(&(range.z_max + 1)).unwrap();

    (x_index_min..x_index_max)
        .flat_map(move |x| (y_index_min..y_index_max).map(move |y| (x, y)))
        .flat_map(move |(x, y)| (z_index_min..z_index_max).map(move |z| (x, y, z)))
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} x={}..{},y={}..{},z={}..{}",
            if self.on { "on" } else { "off" },
            self.x_min,
            self.x_max,
            self.y_min,
            self.y_max,
            self.z_min,
            self.z_max,
        )?;
        Ok(())
    }
}

fn solve(ranges: &[Range]) -> usize {
    let mut reactor = Reactor::from_ranges(&ranges);
    for range in ranges {
        for tuple in iter_regions_indexes(&reactor, &range) {
            let region = get_region_mut(&mut reactor, tuple);
            region.is_lit = range.on;
        }
    }

    let mut count = 0usize;
    for (x_index, xs) in reactor.x_regions.windows(2).enumerate() {
        for (y_index, ys) in reactor.y_regions.windows(2).enumerate() {
            for (z_index, zs) in reactor.z_regions.windows(2).enumerate() {
                let region = get_region(&reactor, (x_index, y_index, z_index));
                if region.is_lit {
                    let x_delta = (xs[1] - xs[0]) as usize;
                    let y_delta = (ys[1] - ys[0]) as usize;
                    let z_delta = (zs[1] - zs[0]) as usize;
                    count += x_delta * y_delta * z_delta;
                }
            }
        }
    }

    count
}

fn is_in_initialization(range: &Range) -> bool {
    [
        range.x_min,
        range.x_max,
        range.y_min,
        range.y_max,
        range.z_min,
        range.z_max,
    ]
    .into_iter()
    .all(|n| -50 <= n && n <= 50)
}

fn solve_part_1(ranges: &[Range]) -> usize {
    let filtered_ranges: Vec<Range> = ranges
        .iter()
        .cloned()
        .filter(is_in_initialization)
        .collect();

    solve(&filtered_ranges)
}

fn solve_part_2(ranges: &[Range]) -> usize {
    solve(ranges)
}

fn main() {
    let input = include_str!("./input.txt");
    let ranges = parse(input);
    println!("Part 1: {}", solve_part_1(&ranges));
    println!("Part 2: {}", solve_part_2(&ranges));
}
