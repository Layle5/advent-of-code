use regex::Regex;

#[derive(Debug, Default)]
struct Vector {
    x: i64,
    y: i64,
}

#[derive(Debug, Default)]
struct Probe {
    position: Vector,
    velocity: Vector,
}

#[derive(Debug, Default)]
struct Target {
    min: Vector,
    max: Vector,
}

#[derive(Debug, Default)]
struct Results {
    max_y: i64,
    number: u64,
}

fn parse_target(input: &str) -> Target {
    let regex = Regex::new(r"target area: x=(\d+)..(\d+), y=(-?\d+)..(-?\d+)").unwrap();
    let captures = regex.captures_iter(input).next().unwrap();
    let mut slices = captures.iter().flatten().map(|m| m.as_str()).skip(1);
    let min_x = slices.next().unwrap().parse().unwrap();
    let max_x = slices.next().unwrap().parse().unwrap();
    let min_y = slices.next().unwrap().parse().unwrap();
    let max_y = slices.next().unwrap().parse().unwrap();

    Target {
        min: Vector { x: min_x, y: min_y },
        max: Vector { x: max_x, y: max_y },
    }
}

fn reached_target(position: &Vector, target: &Target) -> bool {
    target.min.x <= position.x
        && position.x <= target.max.x
        && target.min.y <= position.y
        && position.y <= target.max.y
}

fn simulate(target: &Target, velocity: Vector) -> Option<i64> {
    let mut probe = Probe {
        position: Vector::default(),
        velocity,
    };

    let mut max_y = 0;

    while probe.position.y >= target.min.y {
        probe.position.x += probe.velocity.x;
        probe.position.y += probe.velocity.y;
        probe.velocity.x -= probe.velocity.x.signum();
        probe.velocity.y -= 1;

        max_y = max_y.max(probe.position.y);

        if reached_target(&probe.position, target) {
            return Some(max_y);
        }
    }

    None
}

fn solve(target: Target) -> Results {
    let mut results = Results::default();

    let velocity_min_x = 0;
    let velocity_max_x = target.max.x + 1;
    let velocity_min_y = target.min.y;
    let velocity_max_y = target.min.y.abs() + 1;

    for velocity_x in velocity_min_x..=velocity_max_x {
        for velocity_y in velocity_min_y..=velocity_max_y {
            let max_y_option = simulate(
                &target,
                Vector {
                    x: velocity_x,
                    y: velocity_y,
                },
            );

            if let Some(max_y) = max_y_option {
                results.max_y = results.max_y.max(max_y);
                results.number += 1;
            }
        }
    }

    results
}

fn main() {
    let input = include_str!("./input.txt");
    let target = parse_target(input);
    let results = solve(target);
    dbg!(results);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        let input = include_str!("./example.txt");
        let target = parse_target(input);
        assert_eq!(solve(target).max_y, 45);
    }

    #[test]
    fn example_part_2() {
        let input = include_str!("./example.txt");
        let target = parse_target(input);
        assert_eq!(solve(target).number, 112);
    }
}
