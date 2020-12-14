use std::collections::HashMap;
use std::env;
use std::fs;

#[macro_use]
extern crate lazy_static;
use regex::Regex;

#[derive(Debug, Default)]
struct Program<'a> {
    mask_str: &'a str,
    mask_and: u64,
    mask_or: u64,
    mem: HashMap<u64, u64>,
}

enum Line<'a> {
    Mask(&'a str),
    Mem(u64, u64),
}

fn parse_line(line: &str) -> Line {
    lazy_static! {
        static ref REGEX: Regex =
            Regex::new(r"^mem\[(\d*)\] = (\d*)$").unwrap();
    }

    if let Some(mask_str) = line.strip_prefix("mask = ") {
        Line::Mask(mask_str)
    } else if let Some(captures) = REGEX.captures(line) {
        let address_str = captures.get(1).unwrap().as_str();
        let value_str = captures.get(2).unwrap().as_str();
        let address = address_str.parse().unwrap();
        let value = value_str.parse().unwrap();
        Line::Mem(address, value)
    } else {
        panic!("Could not parse line {}", line)
    }
}

fn apply_mask_part_1<'a>(
    mut program: Program<'a>,
    mask_str: &str,
) -> Program<'a> {
    let mask_and_str = mask_str.to_string().replace('X', "1");
    let mask_or_str = mask_str.to_string().replace('X', "0");
    program.mask_and = u64::from_str_radix(&mask_and_str, 2).unwrap();
    program.mask_or = u64::from_str_radix(&mask_or_str, 2).unwrap();
    program
}

fn apply_mem_part_1(mut program: Program, address: u64, value: u64) -> Program {
    let masked_value = value & program.mask_and | program.mask_or;
    program.mem.insert(address, masked_value);
    program
}

fn apply_mask_part_2<'a>(
    mut program: Program<'a>,
    mask_str: &'a str,
) -> Program<'a> {
    program.mask_str = mask_str;
    program
}

fn generate_addresses(address: u64, mask_str: &str) -> Vec<u64> {
    match mask_str.chars().last() {
        None => vec![address],
        Some(c) => {
            let mask_len = mask_str.len();
            let remaining_address = address >> 1;
            let remaining_mask = &mask_str[..mask_len - 1];
            let addresses: Vec<_> =
                generate_addresses(remaining_address, remaining_mask)
                    .iter()
                    .map(|a| (a << 1) | (address & 1))
                    .collect();
            match c {
                '0' => addresses,
                '1' => addresses.iter().map(|a| *a | 1).collect(),
                'X' => {
                    let addresses_pairs: Vec<[u64; 2]> = addresses
                        .into_iter()
                        .map(|a| [a & !1, a | 1])
                        .collect();
                    addresses_pairs
                        .iter()
                        .flat_map(|a| a.iter())
                        .copied()
                        .collect()
                }
                c => panic!("Unrecognized char {}", c),
            }
        }
    }
}

fn apply_mem_part_2(mut program: Program, address: u64, value: u64) -> Program {
    let addresses = generate_addresses(address, program.mask_str);
    for address in addresses {
        program.mem.insert(address, value);
    }
    program
}

fn solve_part_1(content: &str) {
    let program = content.lines().map(crate::parse_line).fold(
        Program::default(),
        |program, line| match line {
            Line::Mask(mask_str) => apply_mask_part_1(program, mask_str),
            Line::Mem(address, value) => {
                apply_mem_part_1(program, address, value)
            }
        },
    );

    let final_sum: u64 = program.mem.iter().map(|(_, value)| value).sum();

    println!("Part 1: {}", final_sum);
}

fn solve_part_2(content: &str) {
    let program = content.lines().map(crate::parse_line).fold(
        Program::default(),
        |program, line| match line {
            Line::Mask(mask_str) => apply_mask_part_2(program, mask_str),
            Line::Mem(address, value) => {
                apply_mem_part_2(program, address, value)
            }
        },
    );

    let final_sum: u64 = program.mem.iter().map(|(_, value)| value).sum();

    println!("Part 2: {}", final_sum);
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
    use super::generate_addresses;
    #[test]
    fn generate_addresses_test() {
        let assert = |a, m, v: &[_]| {
            assert_eq!(generate_addresses(a, m), v);
        };

        assert(0, "0", &[0]);
        assert(1, "0", &[1]);
        assert(5, "0", &[5]);

        assert(0, "1", &[1]);
        assert(1, "1", &[1]);
        assert(4, "1", &[5]);
        assert(5, "1", &[5]);

        assert(0, "X", &[0, 1]);
        assert(1, "X", &[0, 1]);
        assert(4, "X", &[4, 5]);
        assert(5, "X", &[4, 5]);

        assert(42, "X1001X", &[26, 27, 58, 59]);
        assert(26, "X0XX", &[16, 17, 18, 19, 24, 25, 26, 27]);
    }
}
