fn main() {
    let input = include_str!("./input.txt");
    let numbers: Vec<Vec<u64>> = input
        .split("\n\n")
        .map(|p| p.lines().map(|l| l.parse().unwrap()).collect())
        .collect();

    let mut sums: Vec<u64> = numbers.iter().map(|ns| ns.iter().sum()).collect();
    sums.sort_by(|a, b| b.cmp(a));

    println!("Part 1: {}", sums[0]);
    println!("Part 2: {}", sums[0] + sums[1] + sums[2]);
}
