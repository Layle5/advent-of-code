use std::env;
use std::fs;

use itertools::Itertools;

type Mass = i64;

fn compute_fuel_simple(mass: Mass) -> Mass {
    mass / 3 - 2
}

fn compute_fuel_recursive(mass: Mass) -> Mass {
    let fuel_mass = compute_fuel_simple(mass);
    if fuel_mass <= 0 {
        0
    } else {
        fuel_mass + compute_fuel_recursive(fuel_mass)
    }
}

fn solve(content: &str, compute_fuel: fn(Mass) -> Mass) -> Mass {
    content
        .lines()
        .map(&str::parse::<Mass>)
        .map(Result::unwrap)
        .map(compute_fuel)
        .sum()
}

fn solve_part_1(content: &str) {
    println!("Part 1: {}", solve(content, compute_fuel_simple))
}

fn solve_part_2(content: &str) {
    println!("Part 2: {}", solve(content, compute_fuel_recursive))
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
