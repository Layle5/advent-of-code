fn main() {
    let input = include_str!("./input.txt");
    let rounds: Vec<(u8, u8)> = input
        .lines()
        .map(|l| (l.as_bytes()[0], l.as_bytes()[2]))
        .collect();

    let total_score: u64 = rounds
        .iter()
        .map(|r| {
            let left_hand = r.0 - b'A';
            let right_hand = r.1 - b'X';
            let shape_score = right_hand + 1;
            let outcome_score = (right_hand + 4 - left_hand) % 3 * 3;
            (shape_score + outcome_score) as u64
        })
        .sum();
    println!("Part 1: {}", total_score);

    let new_total_score: u64 = rounds
        .iter()
        .map(|r| {
            let outcome = r.1 - b'X';
            let outcome_score = outcome * 3;
            let left_hand = r.0 - b'A';
            let right_hand = (left_hand + outcome + 2) % 3;
            let shape_score = right_hand + 1;
            (shape_score + outcome_score) as u64
        })
        .sum();
    println!("Part 2: {}", new_total_score);
}
