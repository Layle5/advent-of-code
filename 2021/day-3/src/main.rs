use std::mem::swap;

fn solve_part_1(input: &str) -> u64 {
    let lines = input.lines().collect::<Vec<_>>();
    let binary_size = lines.first().unwrap().len();

    let rates = (0..binary_size).fold((0, 0), |(a, b), digit_index| {
        let one_digit_count = lines
            .iter()
            .flat_map(|line| line.as_bytes().get(digit_index))
            .filter(|byte| **byte == b'1')
            .count();

        let zero_digit_count = lines.len() - one_digit_count;
        let most_common_digit = (one_digit_count > zero_digit_count) as u64;
        let least_common_digit = (one_digit_count < zero_digit_count) as u64;
        (a * 2 + most_common_digit, b * 2 + least_common_digit)
    });

    rates.0 * rates.1
}

fn digits_to_number(bytes: &[u8]) -> u64 {
    bytes
        .iter()
        .map(|&b| match b {
            b'0' => 0,
            _ => 1,
        })
        .fold(0, |r, b| r * 2 + b)
}

fn compute_rating(all_lines: &[&[u8]], invert_lines_selection: bool) -> u64 {
    let binary_size = all_lines.first().unwrap().len();

    let mut current_lines: Vec<&[u8]> = all_lines.to_owned();
    let mut lines_digit_one: Vec<&[u8]> = Vec::with_capacity(all_lines.len());
    let mut lines_digit_zero: Vec<&[u8]> = Vec::with_capacity(all_lines.len());
    for digit_index in 0..binary_size {
        for line in &current_lines {
            let lines_digit = if line[digit_index] == b'0' {
                &mut lines_digit_zero
            } else {
                &mut lines_digit_one
            };
            lines_digit.push(line);
        }

        let one_digit_count = lines_digit_one.len();
        let zero_digit_count = lines_digit_zero.len();
        let use_zero_lines = zero_digit_count > one_digit_count;

        swap(
            &mut current_lines,
            if use_zero_lines ^ invert_lines_selection {
                &mut lines_digit_zero
            } else {
                &mut lines_digit_one
            },
        );

        lines_digit_one.clear();
        lines_digit_zero.clear();

        if current_lines.len() < 2 {
            let result_str = current_lines.first().unwrap();
            return digits_to_number(result_str);
        }
    }

    panic!("could not find rating")
}

fn solve_part_2(input: &str) -> u64 {
    let lines = input.lines().map(str::as_bytes).collect::<Vec<_>>();

    let oxygen_generator_rating = compute_rating(&lines, false);
    let life_support_rating = compute_rating(&lines, true);

    oxygen_generator_rating * life_support_rating
}

fn main() {
    let input = include_str!("./input.txt");
    println!("Part 1: {}", solve_part_1(input));
    println!("Part 2: {}", solve_part_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        let input = include_str!("./example.txt");
        assert_eq!(solve_part_1(input), 198);
    }

    #[test]
    fn example_part_2() {
        let input = include_str!("./example.txt");
        assert_eq!(solve_part_2(input), 230);
    }
}
