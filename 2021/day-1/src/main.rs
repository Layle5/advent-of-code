fn count_increases(numbers: &[u64], window_size: usize) -> usize {
    let mut count = 0;
    for index in window_size..numbers.len() {
        if numbers[index - window_size] < numbers[index] {
            count += 1;
        }
    }

    count
}

fn main() {
    let input: &str = include_str!("./input.txt");
    let numbers = input
        .lines()
        .map(str::parse::<u64>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    println!("Part 1: {}", count_increases(&numbers, 1));
    println!("Part 2: {}", count_increases(&numbers, 3));
}
