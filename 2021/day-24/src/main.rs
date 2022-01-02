use std::iter::once;

#[derive(Debug)]
struct Group {
    pop: bool,
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Context<'a> {
    groups: &'a [Group],
}

type Number = Vec<i64>;

#[derive(Debug)]
struct Results<'a> {
    groups: &'a [Group],
    max: Number,
    min: Number,
}

fn parse(input: &str) -> Vec<Group> {
    let lines: Vec<Vec<&str>> = input
        .trim()
        .lines()
        .map(|line| line.split(' ').collect())
        .collect();

    lines
        .chunks(18)
        .map(|chunk| {
            let pop = chunk[4][2] == "26";
            let x = chunk[5][2].parse().unwrap();
            let y = chunk[15][2].parse().unwrap();
            Group { pop, x, y }
        })
        .collect()
}

fn concat_number(
    left_digit: i64,
    after_left_digits: Vec<i64>,
    right_digit: i64,
    after_right_digits: Vec<i64>,
) -> Vec<i64> {
    once(left_digit)
        .chain(after_left_digits.into_iter())
        .chain(once(right_digit))
        .chain(after_right_digits.into_iter())
        .collect()
}

fn do_something(context: Context) -> Results {
    if context.groups.is_empty() || context.groups[0].pop {
        return Results {
            groups: context.groups,
            max: vec![],
            min: vec![],
        };
    }

    let (left_group, after_left_groups) = context.groups.split_first().unwrap();

    let after_left_results = do_something(Context {
        groups: after_left_groups,
    });

    let (right_group, after_right_groups) = after_left_results.groups.split_first().unwrap();

    let delta = left_group.y + right_group.x;
    let left_max_digit = 9.min(9 - delta);
    let left_min_digit = 1.max(1 - delta);
    let right_max_digit = left_max_digit + delta;
    let right_min_digit = left_min_digit + delta;

    let after_right_results = do_something(Context {
        groups: after_right_groups,
    });

    Results {
        groups: after_right_results.groups,
        max: concat_number(
            left_max_digit,
            after_left_results.max,
            right_max_digit,
            after_right_results.max,
        ),
        min: concat_number(
            left_min_digit,
            after_left_results.min,
            right_min_digit,
            after_right_results.min,
        ),
    }
}

fn build_number(number: Vec<i64>) -> u64 {
    number.into_iter().fold(0u64, |n, d| n * 10 + d as u64)
}

fn main() {
    let input = include_str!("./input.txt");
    let groups = parse(input);
    let results = do_something(Context { groups: &groups });
    let max_number = build_number(results.max);
    println!("Part 1: {}", max_number);
    let min_number = build_number(results.min);
    println!("Part 2: {}", min_number);
}
