use std::collections::VecDeque;
use std::env;
use std::fs;

type Number = u64;
type Numbers = Vec<Number>;

fn parse_numbers(content: &str) -> Numbers {
    content
        .lines()
        .map(|line: &str| line.parse().unwrap())
        .collect()
}

fn is_number_sum_of_2(
    number: Number,
    previous_numbers: &VecDeque<Number>,
) -> bool {
    for &previous_number in previous_numbers {
        if previous_number < number {
            let delta = number - previous_number;
            if previous_numbers.contains(&delta) {
                return true;
            }
        }
    }

    false
}

fn find_first_invalid_number(
    numbers: &Numbers,
    previous_numbers_size: usize,
) -> Option<Number> {
    let mut previous_numbers: VecDeque<Number> = numbers
        .iter()
        .take(previous_numbers_size)
        .cloned()
        .collect();

    for number in numbers.iter().skip(previous_numbers_size).cloned() {
        if is_number_sum_of_2(number, &previous_numbers) {
            previous_numbers.pop_front();
            previous_numbers.push_back(number);
        } else {
            return Some(number);
        }
    }

    None
}

fn find_weakness_num(numbers: &Numbers, target_overall_sum: Number) -> Number {
    let mut weakness_numbers: VecDeque<Number> = VecDeque::new();
    let mut overall_sum: Number = 0;
    let mut number_index: usize = 0;

    loop {
        if overall_sum == target_overall_sum {
            let min = weakness_numbers.iter().min().unwrap();
            let max = weakness_numbers.iter().max().unwrap();
            return min + max;
        }
        if overall_sum < target_overall_sum {
            let number = numbers[number_index];
            weakness_numbers.push_back(number);
            overall_sum += number;
            number_index += 1;
        } else {
            let number = weakness_numbers.pop_front().unwrap();
            overall_sum -= number;
        }
    }
}

fn solve_part_1(content: &str) {
    let numbers = parse_numbers(content);
    let invalid_number = find_first_invalid_number(&numbers, 25).unwrap();
    println!("Part 1: {:?}", invalid_number)
}

fn solve_part_2(content: &str) {
    let numbers = parse_numbers(content);
    let invalid_number = find_first_invalid_number(&numbers, 25).unwrap();
    let weakness_sum = find_weakness_num(&numbers, invalid_number);
    println!("Part 2: {:?}", weakness_sum)
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
    fn parse_numbers_test() {
        let assert = |s: &str, e: &[Number]| {
            assert_eq!(parse_numbers(s), e);
        };

        assert("", &[]);
        assert("1\n", &[1]);
        assert("1\n2\n3\n", &[1, 2, 3]);
    }
    #[test]
    fn is_number_sum_of_2_test() {
        let assert = |number, previous_numbers: &[Number], expected| {
            let deque = previous_numbers.iter().cloned().collect();
            assert_eq!(is_number_sum_of_2(number, &deque), expected);
        };

        assert(0, &[], false);
        assert(0, &[1, 2], false);
        assert(1, &[1, 2], false);
        assert(3, &[1, 2], true);
        assert(5, &[1, 2, 3], true);
        assert(7, &[1, 2, 3], false);
    }
}
