use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

type Number = u64;
type Numbers = HashSet<Number>;
type Differences = (usize, usize, usize);
type Cache = HashMap<Number, usize>;

fn parse_numbers(content: &str) -> Numbers {
    content.lines().map(|line| line.parse().unwrap()).collect()
}

fn get_device_joltage(numbers: &Numbers) -> Number {
    numbers.iter().max().unwrap() + 3
}

fn add_differences(lhs: Differences, rhs: Differences) -> Differences {
    let (l1, l2, l3) = lhs;
    let (r1, r2, r3) = rhs;
    (l1 + r1, l2 + r2, l3 + r3)
}

fn count_diff_to_target(
    numbers: &Numbers,
    current_joltage: Number,
    target_joltage: Number,
) -> Differences {
    let possible_differences: &[(Number, Differences)] =
        &[(1, (1, 0, 0)), (2, (0, 1, 0)), (3, (0, 0, 1))];

    for &(possible_difference, differences_increment) in possible_differences {
        let next_joltage = current_joltage + possible_difference;
        if next_joltage == target_joltage {
            return differences_increment;
        }
        if numbers.contains(&next_joltage) {
            let next_differences =
                count_diff_to_target(numbers, next_joltage, target_joltage);
            return add_differences(next_differences, differences_increment);
        }
    }

    panic!(
        "Could not find a path from {} jolts to {} jolts",
        current_joltage, target_joltage
    )
}

fn solve_part_1(content: &str) {
    let numbers = parse_numbers(content);
    let device_voltage = get_device_joltage(&numbers);

    let differences = count_diff_to_target(&numbers, 0, device_voltage);

    let (differences_1, _, differences_3) = differences;
    println!("Part 1: {}", differences_1 * differences_3)
}

fn count_arrangements(
    numbers: &Numbers,
    current_joltage: Number,
    target_joltage: Number,
    arrangements_count_cache: &mut Cache,
) -> usize {
    match arrangements_count_cache.get(&current_joltage) {
        Some(&count) => return count,
        None => {}
    }

    let mut count = 0;
    let possible_differences = &[1, 2, 3];
    for possible_difference in possible_differences {
        let next_joltage = current_joltage + possible_difference;
        if numbers.contains(&next_joltage) {
            count += count_arrangements(
                numbers,
                next_joltage,
                target_joltage,
                arrangements_count_cache,
            )
        }
        if next_joltage == target_joltage {
            count += 1
        }
    }

    if count == 0 {
        panic!(
            "Could not find a path from {} jolts to {} jolts",
            current_joltage, target_joltage
        )
    }

    arrangements_count_cache.insert(current_joltage, count);
    count
}

fn solve_part_2(content: &str) {
    let numbers = parse_numbers(content);
    let start_voltage = 0;
    let device_voltage = get_device_joltage(&numbers);
    let mut arrangements_count_cache = Cache::new();

    let count = count_arrangements(
        &numbers,
        start_voltage,
        device_voltage,
        &mut arrangements_count_cache,
    );

    println!("Part 2: {}", count)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let default_filename: &str = "./res/input.txt";
    let filename: &str =
        args.get(1).map(|s| s.as_ref()).unwrap_or(default_filename);

    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    solve_part_1(&content);
    solve_part_2(&content);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_diff_to_target_test() {
        let assert = |n: &[Number], s, t, e| {
            let numbers = n.iter().cloned().collect();
            assert_eq!(count_diff_to_target(&numbers, s, t), e);
        };

        assert(&[], 0, 1, (1, 0, 0));
        assert(&[], 0, 2, (0, 1, 0));
        assert(&[], 0, 3, (0, 0, 1));
        assert(&[1], 0, 2, (2, 0, 0));
        assert(&[1, 2, 3], 0, 4, (4, 0, 0));
    }

    #[test]
    fn count_arrangements_test() {
        let assert = |n: &[Number], s, t, e| {
            let numbers = n.iter().cloned().collect();
            let mut cache = Cache::new();
            assert_eq!(count_arrangements(&numbers, s, t, &mut cache), e);
        };

        assert(&[], 0, 1, 1);
        assert(&[], 0, 2, 1);
        assert(&[], 0, 3, 1);
        assert(&[1, 2], 0, 3, 4);
        assert(&[1, 2, 3], 0, 4, 7);
    }
}
