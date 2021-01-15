use std::env;
use std::fs;

use itertools::Itertools;

type Password = u64;

struct GroupCounts<T>
where
    T: Iterator,
{
    inner: T,
    last_value_option: Option<<T as Iterator>::Item>,
}

impl<T> Iterator for GroupCounts<T>
where
    T: Iterator,
    <T as Iterator>::Item: PartialEq,
{
    type Item = usize;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.last_value_option.is_none() {
            self.last_value_option = self.inner.next();
        }
        if let Some(ref mut last_value) = self.last_value_option {
            let mut count = 1;
            loop {
                if let Some(value) = self.inner.next() {
                    if value == *last_value {
                        count += 1;
                    } else {
                        *last_value = value;
                        break;
                    }
                } else {
                    self.last_value_option = None;
                    break;
                }
            }

            Some(count)
        } else {
            None
        }
    }
}

trait IteratorExt: Iterator {
    fn group_counts(self) -> GroupCounts<Self>
    where
        Self: Sized,
        <Self as Iterator>::Item: PartialEq,
    {
        GroupCounts {
            inner: self,
            last_value_option: None,
        }
    }
}

impl<T: Iterator> IteratorExt for T {}

fn parse_range(content: &str) -> (Password, Password) {
    content
        .trim()
        .split('-')
        .map(&str::parse::<Password>)
        .map(Result::unwrap)
        .collect_tuple::<(_, _)>()
        .unwrap()
}

fn is_password_valid_part_1(password: &Password) -> bool {
    let digit_vec = (0..6)
        .rev()
        .into_iter()
        .map(|power| (10 as Password).pow(power))
        .map(|unit| password / unit % 10)
        .collect_vec();
    digit_vec.iter().tuple_windows().all(|(a, b)| a <= b)
        && digit_vec.iter().tuple_windows().any(|(a, b)| a == b)
}

fn is_password_valid_part_2(password: &Password) -> bool {
    let digit_vec = (0..6)
        .rev()
        .into_iter()
        .map(|power| (10 as Password).pow(power))
        .map(|unit| password / unit % 10)
        .collect_vec();
    digit_vec.iter().group_counts().any(|c| c == 2)
        && digit_vec.iter().tuple_windows().all(|(a, b)| a <= b)
}

fn solve(content: &str, is_password_valid: fn(&Password) -> bool) -> usize {
    let range = parse_range(content);

    (range.0..=range.1)
        .into_iter()
        .filter(is_password_valid)
        .count()
}

fn solve_part_1(content: &str) {
    let valid_password_count = solve(content, is_password_valid_part_1);
    println!("Part 1: {}", valid_password_count)
}

fn solve_part_2(content: &str) {
    let valid_password_count = solve(content, is_password_valid_part_2);
    println!("Part 2: {}", valid_password_count)
}

fn main() {
    let args = env::args().collect_vec();
    let filename = args.get(1).map(|s| s.as_ref()).unwrap_or("./res/input.txt");

    let content = fs::read_to_string(filename).unwrap();

    solve_part_1(&content);
    solve_part_2(&content);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn group_counts_assert<T: PartialEq>(v: &[T], e: &[usize]) {
        assert_eq!(v.iter().group_counts().collect_vec(), e);
    }

    #[test]
    fn group_counts_test() {
        group_counts_assert(&[] as &[i32], &[]);
        group_counts_assert(&[1], &[1]);
        group_counts_assert(&[1, 1], &[2]);
        group_counts_assert(&[1, 1, 1], &[3]);
        group_counts_assert(&[1, 2], &[1, 1]);
        group_counts_assert(&[1, 2, 3], &[1, 1, 1]);
        group_counts_assert(&[1, 1, 2, 3, 3, 3], &[2, 1, 3]);
        group_counts_assert(&["a", "b", "b"], &[1, 2]);
    }
}
