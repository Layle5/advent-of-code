use itertools::Itertools;
use std::{
    env,
    iter::{once, successors},
};
use std::{fs, iter::Map};

type Digit = i8;
type Number = Vec<Digit>;
type NumberSlice<'a> = &'a [Digit];

#[derive(Clone)]
struct RepeatElement<I, T>
where
    I: Iterator<Item = T>,
    T: Copy,
{
    inner: I,
    number: usize,
    current: Option<(T, usize)>,
}

impl<I, T> Iterator for RepeatElement<I, T>
where
    I: Iterator<Item = T>,
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.number == 0 {
            None
        } else if let Some((current_item, current_number)) = self.current {
            if current_number == 1 {
                self.current = None
            } else {
                self.current = Some((current_item, current_number - 1))
            }
            Some(current_item)
        } else if let Some(current_item) = self.inner.next() {
            self.current = Some((current_item, self.number));
            self.next()
        } else {
            None
        }
    }
}

struct FoldScan<I, T, F>
where
    I: Iterator<Item = T>,
    T: Copy,
    F: FnMut(T, T) -> T,
{
    inner: I,
    f: F,
    did_first: bool,
    current: T,
}

impl<I, T, F> Iterator for FoldScan<I, T, F>
where
    I: Iterator<Item = T>,
    T: Copy,
    F: FnMut(T, T) -> T,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.did_first {
            if let Some(item) = self.inner.next() {
                self.current = (self.f)(self.current, item);
                Some(self.current)
            } else {
                None
            }
        } else {
            self.did_first = true;
            Some(self.current)
        }
    }
}

trait IteratorExtended<T>: Iterator<Item = T> + Sized {
    fn repeat_element(self, number: usize) -> RepeatElement<Self, T>
    where
        T: Copy;

    fn fold_scan<F>(self, current: T, f: F) -> FoldScan<Self, T, F>
    where
        T: Copy,
        F: FnMut(T, T) -> T;
}

impl<I, T> IteratorExtended<T> for I
where
    I: Iterator<Item = T>,
{
    fn repeat_element(self, number: usize) -> RepeatElement<Self, T>
    where
        T: Copy,
    {
        RepeatElement {
            inner: self,
            number,
            current: None,
        }
    }

    fn fold_scan<F>(self, current: T, f: F) -> FoldScan<Self, T, F>
    where
        T: Copy,
        F: FnMut(T, T) -> T,
    {
        FoldScan {
            inner: self,
            current,
            did_first: false,
            f,
        }
    }
}

fn parse_number(content: &str) -> Number {
    content
        .trim()
        .bytes()
        .map(|byte| byte - b'0')
        .map(|byte| byte as i8)
        .collect_vec()
}

fn step_phase(number: NumberSlice) -> Number {
    (0..number.len())
        .map(|index| {
            let pattern = [0, 1, 0, -1]
                .iter()
                .repeat_element(index + 1)
                .cycle()
                .skip(1);
            i64::abs(
                number
                    .iter()
                    .zip(pattern)
                    .map(|(input_digit, pattern_digit)| {
                        *input_digit as i64 * *pattern_digit as i64
                    })
                    .sum::<i64>()
                    % 10,
            ) as Digit
        })
        .collect_vec()
}

fn solve_part_1(content: &str) {
    let mut number = parse_number(content);

    for _ in 0..100 {
        number = step_phase(&number);
    }

    let first_digits = number
        .into_iter()
        .take(8)
        .map(|digit| (digit as u8))
        .map(|digit| digit + b'0')
        .map(|digit| digit as char)
        .collect::<String>();

    println!("Part 1: {}", first_digits)
}

fn solve_part_2(content: &str) {
    let mut number = parse_number(content);
    let number_len = number.len();
    number = number
        .into_iter()
        .cycle()
        .take(10000 * number_len)
        .collect_vec();

    let number_len = number.len();

    let message_offset = number
        .iter()
        .take(7)
        .fold(0, |res, digit| res * 10 + (*digit as usize));

    number = number.into_iter().skip(message_offset).collect_vec();

    assert!(number_len / 2 < message_offset);

    for _ in 0..100 {
        let sums = number
            .iter()
            .map(|digit| *digit as i64)
            .fold_scan(0, |a, b| a + b)
            .collect_vec();

        number = number
            .iter()
            .enumerate()
            .map(|(index, _)| sums.last().unwrap() - sums[index])
            .map(|digit| digit.abs() % 10)
            .map(|digit| digit as i8)
            .collect_vec();
    }

    let first_digits = number
        .into_iter()
        .take(8)
        .map(|digit| (digit as u8))
        .map(|digit| digit + b'0')
        .map(|digit| digit as char)
        .collect::<String>();

    println!("Part 2: {}", first_digits)
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
    use itertools::assert_equal;

    use super::*;

    #[test]
    fn repeat_element_test() {
        let empty = [] as [i32; 0];
        assert_equal(empty.iter().repeat_element(0), empty.iter());
        assert_equal(empty.iter().repeat_element(1), empty.iter());
        assert_equal(empty.iter().repeat_element(2), empty.iter());

        assert_equal([1].iter().repeat_element(0), empty.iter());
        assert_equal([1].iter().repeat_element(1), [1].iter());
        assert_equal([1].iter().repeat_element(2), [1, 1].iter());

        assert_equal([1, 2].iter().repeat_element(0), empty.iter());
        assert_equal([1, 2].iter().repeat_element(1), [1, 2].iter());
        assert_equal([1, 2].iter().repeat_element(2), [1, 1, 2, 2].iter());
    }
}
