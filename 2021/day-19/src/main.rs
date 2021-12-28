use itertools::Itertools;
use nalgebra::{Matrix3, Point3, Vector3};
use std::collections::HashMap;

type Point = Point3<i64>;
type Beacon = Point;

fn parse_beacon(s: &str) -> Beacon {
    let mut parts = s.split(',');
    let mut beacon = Beacon::origin();
    beacon[0] = parts.next().unwrap().parse().unwrap();
    beacon[1] = parts.next().unwrap().parse().unwrap();
    beacon[2] = parts.next().unwrap().parse().unwrap();
    beacon
}

type Orientation = Matrix3<i64>;

fn generate_orientations() -> Vec<Orientation> {
    let mut results = vec![];

    for x in [-1, 1] {
        for y in [-1, 1] {
            for indexes in (0..3).permutations(3) {
                let mut x_axis = Vector3::zeros();
                x_axis[indexes[0]] = x;

                let mut y_axis = Vector3::zeros();
                y_axis[indexes[1]] = y;

                let z_axis = x_axis.cross(&y_axis);

                let orientation = Orientation::from_columns(&[x_axis, y_axis, z_axis]);
                results.push(orientation);
            }
        }
    }

    results
}

type BeaconSet = Vec<Beacon>;

#[derive(Debug)]
struct Scanner {
    position: Point,
    beacons: BeaconSet,
}

fn parse_beacons(input: &str) -> Vec<BeaconSet> {
    input
        .split("\n\n")
        .map(|s| s.lines().skip(1).map(parse_beacon).collect())
        .collect()
}

fn transform_beacon(orientation: &Orientation, beacon: &Beacon) -> Beacon {
    orientation * beacon
}

fn transform_beacons(orientation: &Orientation, beacons: &[Beacon]) -> BeaconSet {
    beacons
        .iter()
        .map(|beacon| transform_beacon(orientation, beacon))
        .collect()
}

fn manhattan_distance(p1: &Point, p2: &Point) -> i64 {
    (p1 - p2).into_iter().map(|n| n.abs()).sum()
}

fn find_overlap_delta(scanner: &Scanner, beacons: &[Beacon]) -> Option<Point> {
    let mut deltas: HashMap<Point, usize> = HashMap::new();
    for reference_beacon in &scanner.beacons {
        for target_beacon in beacons {
            let delta: Point = Point::from(reference_beacon - target_beacon);
            *deltas.entry(delta).or_default() += 1;
        }
    }

    deltas
        .into_iter()
        .filter(|(_, count)| *count >= 12)
        .map(|(d, _)| d)
        .next()
}

fn identify_scanner_from_transformed(
    scanners: &[Scanner],
    transformed_beacon_set: &[Beacon],
) -> Option<Scanner> {
    for scanner in scanners {
        if let Some(overall_delta) = find_overlap_delta(scanner, transformed_beacon_set) {
            let translate_beacon_set: BeaconSet = transformed_beacon_set
                .iter()
                .map(|b| {
                    Point::from_slice(&[
                        b.x + overall_delta.x,
                        b.y + overall_delta.y,
                        b.z + overall_delta.z,
                    ])
                })
                .collect();

            return Some(Scanner {
                position: overall_delta,
                beacons: translate_beacon_set,
            });
        }
    }

    None
}

fn identify_scanner_from_beacons(
    orientations: &[Orientation],
    scanners: &[Scanner],
    beacon_sets: &[BeaconSet],
) -> (usize, Scanner) {
    for orientation in orientations {
        for (beacon_set_index, beacon_sets) in beacon_sets.iter().enumerate() {
            let transformed_beacon_set = transform_beacons(orientation, beacon_sets);
            let new_scanner_option =
                identify_scanner_from_transformed(scanners, &transformed_beacon_set);
            if let Some(new_scanner) = new_scanner_option {
                return (beacon_set_index, new_scanner);
            }
        }
    }

    panic!("could not identity a scanner from beacons");
}

fn identify_scanners(mut remaining_beacons: Vec<BeaconSet>) -> Vec<Scanner> {
    let orientations = generate_orientations();

    let mut found_scanners: Vec<Scanner> = Vec::new();
    let first_beacons = remaining_beacons.remove(0);
    found_scanners.push(Scanner {
        position: Point::origin(),
        beacons: first_beacons,
    });

    while !remaining_beacons.is_empty() {
        let (scanner_index, scanner) =
            identify_scanner_from_beacons(&orientations, &found_scanners, &remaining_beacons);
        found_scanners.push(scanner);
        remaining_beacons.remove(scanner_index);
    }

    found_scanners
}

fn solve_part_1(identified_scannners: &[Scanner]) -> usize {
    identified_scannners
        .iter()
        .flat_map(|scanner| scanner.beacons.iter())
        .unique()
        .count()
}

fn solve_part_2(identified_scannners: &[Scanner]) -> i64 {
    identified_scannners
        .iter()
        .map(|scanner| scanner.position)
        .tuple_combinations::<(Point, Point)>()
        .map(|(p1, p2)| manhattan_distance(&p1, &p2))
        .max()
        .unwrap()
}

fn main() {
    let input = include_str!("./input.txt");
    let beacon_sets = parse_beacons(input);
    let identified_scannners = identify_scanners(beacon_sets);
    println!("Part 1: {}", solve_part_1(&identified_scannners));
    println!("Part 2: {}", solve_part_2(&identified_scannners));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("./example.txt");

    #[test]
    fn example_part_1() {
        let beacon_sets = parse_beacons(EXAMPLE_INPUT);
        let identified_scannners = identify_scanners(beacon_sets);
        assert_eq!(solve_part_1(&identified_scannners), 79);
    }

    #[test]
    fn example_part_2() {
        let beacon_sets = parse_beacons(EXAMPLE_INPUT);
        let identified_scannners = identify_scanners(beacon_sets);
        assert_eq!(solve_part_2(&identified_scannners), 3621);
    }
}
