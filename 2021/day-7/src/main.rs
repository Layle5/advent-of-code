fn parse(input: &str) -> Vec<u64> {
    let mut crabs = input
        .trim()
        .split(',')
        .map(str::parse::<u64>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    crabs.sort_unstable();

    crabs
}

fn compute_total_fuel<F>(crabs: &[u64], get_distance: F) -> u64
where
    F: Fn(u64) -> u64,
{
    crabs.iter().copied().map(get_distance).sum()
}

fn get_distance_part_1(crab: u64, target: u64) -> u64 {
    if crab < target {
        target - crab
    } else {
        crab - target
    }
}

fn solve_part_1(crabs: &[u64]) -> u64 {
    let nth = crabs.get(crabs.len() / 2).copied().unwrap();
    compute_total_fuel(crabs, |crab| get_distance_part_1(crab, nth))
}

fn get_distance_part_2(crab: u64, target: u64) -> u64 {
    let distance = get_distance_part_1(crab, target);
    return distance * (distance + 1) / 2;
}

fn solve_part_2(crabs: &[u64]) -> u64 {
    let min = crabs.iter().copied().min().unwrap();
    let max = crabs.iter().copied().max().unwrap();
    (min..=max)
        .into_iter()
        .map(|target| compute_total_fuel(crabs, |crab| get_distance_part_2(crab, target)))
        .min()
        .unwrap()
}

fn main() {
    let input = include_str!("./input.txt");
    let crabs = parse(input);
    println!("Part 1: {}", solve_part_1(&crabs));
    println!("Part 2: {}", solve_part_2(&crabs));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        let input = include_str!("./example.txt");
        assert_eq!(solve_part_1(&parse(input)), 37);
    }

    #[test]
    fn example_part_2() {
        let input = include_str!("./example.txt");
        assert_eq!(solve_part_2(&parse(input)), 168);
    }
}
