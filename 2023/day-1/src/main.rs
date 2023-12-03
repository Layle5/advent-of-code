fn main() {
    let input = include_str!("input.txt");
    let sum_one = compute(input, get_digits_as_characters);
    let sum_two = compute(input, get_digits_as_words);
    println!("Part 1: {sum_one}");
    println!("Part 2: {sum_two}");
}

fn compute(input: &str, get_digits: fn(&str) -> Vec<u64>) -> u64 {
    let sum = input.lines().map(|line| {
        let digits = get_digits(line);
        let first_digit = digits.iter().copied().next().unwrap_or_default();
        let last_digit = digits.iter().rev().copied().next().unwrap_or_default();
        let result = first_digit * 10 + last_digit;
        eprintln!("{get_digits:?} | {line} => {digits:?} => {result}");
        result
    }).sum::<u64>();

    sum
}

fn get_digits_as_characters(line: &str) -> Vec<u64> {
    line.as_bytes().iter().copied().filter(is_numeric).map(|b| (b - b'0') as u64).collect()
}

const DIGIT_WORDS: [&str; 9] = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];

fn get_digits_as_words(line: &str) -> Vec<u64> {
    let mut line_rest = line;
    let mut digits = vec![];
    while !line_rest.is_empty() {
        let next_char = line_rest.chars().next().unwrap();
        if next_char.is_numeric() {
            digits.push((next_char as u8 - b'0') as u64);
            line_rest = &line_rest[1..];
            continue;
        }

        let digit_option = DIGIT_WORDS.iter().enumerate().filter_map(|(index, digit_word)| {
            line_rest.strip_prefix(digit_word).map(|line_rest| index as u64 + 1)
        }).next();

        if let Some(digit) = digit_option {
            digits.push(digit);
        }

        line_rest = &line_rest[1..];
    }

    digits
}

fn is_numeric(byte: &u8) -> bool {
    b'0' <= *byte && *byte <= b'9'
}
