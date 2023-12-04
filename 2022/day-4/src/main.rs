use std::io::BufRead;

fn contains(outer: &[u64; 2], inner: &[u64; 2]) -> bool {
    outer[0] <= inner[0] && inner[1] <= outer[1]
}

fn overlaps(left: &[u64; 2], right: &[u64; 2]) -> bool {
    left[0] <= right[1] && right[0] <= left[1]
}

fn main() {
    let input = include_str!("./input.txt");
    let pairs: Vec<_> = input
        .lines()
        .map(|l| {
            let parts: Vec<u64> = l
                .split(|c| c == '-' || c == ',')
                .map(str::parse::<u64>)
                .map(Result::unwrap)
                .collect();

            let left = [parts[0], parts[1]];
            let right = [parts[2], parts[3]];
            (left, right)
        })
        .collect();

    let count = pairs
        .iter()
        .filter(|(left, right)| contains(left, right) || contains(right, left))
        .count();
    println!("Part 1: {}", count);

    let count = pairs
        .iter()
        .filter(|(left, right)| overlaps(left, right))
        .count();
    println!("Part 2: {}", count);
}
