use std::env;
use std::fs;

#[derive(Debug)]
enum Token {
    Num(u64),
    Add,
    Mul,
    Open,
    Close,
}

fn parse(content: &str) -> Vec<Vec<Token>> {
    content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '+' => Some(Token::Add),
                    '*' => Some(Token::Mul),
                    '(' => Some(Token::Open),
                    ')' => Some(Token::Close),
                    _ => c.to_digit(10).map(u32::into).map(Token::Num),
                })
                .filter(Option::is_some)
                .map(Option::unwrap)
                .collect()
        })
        .collect()
}

fn pop_operators(
    operands: &mut Vec<u64>,
    operators: &mut Vec<Token>,
    pop_mul: bool,
) {
    while let Some(last) = operators.last() {
        match last {
            Token::Add => {
                let b = operands.pop().unwrap();
                let a = operands.pop().unwrap();
                operands.push(a + b);
                operators.pop();
            }
            Token::Mul if pop_mul => {
                let b = operands.pop().unwrap();
                let a = operands.pop().unwrap();
                operands.push(a * b);
                operators.pop();
            }
            _ => break,
        }
    }
}

fn compute(line: Vec<Token>, is_add_prioritised: bool) -> u64 {
    let mut operands: Vec<u64> = Vec::new();
    let mut operators: Vec<Token> = Vec::new();
    for token in line {
        match token {
            Token::Num(n) => operands.push(n),
            Token::Add => {
                pop_operators(
                    &mut operands,
                    &mut operators,
                    !is_add_prioritised,
                );
                operators.push(token);
            }
            Token::Mul => {
                pop_operators(&mut operands, &mut operators, true);
                operators.push(token);
            }
            Token::Open => {
                operators.push(token);
            }
            Token::Close => {
                pop_operators(&mut operands, &mut operators, true);
                operators.pop();
            }
        }
    }
    pop_operators(&mut operands, &mut operators, true);

    operands.pop().unwrap()
}

fn solve_part_1(content: &str) {
    let lines = parse(content);
    let compute_line = |line| compute(line, false);
    let total: u64 = lines.into_iter().map(compute_line).sum();
    println!("Part 1: {}", total)
}

fn solve_part_2(content: &str) {
    let lines = parse(content);
    let compute_line = |line| compute(line, true);
    let total: u64 = lines.into_iter().map(compute_line).sum();
    println!("Part 2: {}", total)
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
mod tests {}
