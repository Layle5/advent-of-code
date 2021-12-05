use std::{mem::swap, num::ParseIntError, str::FromStr};

#[derive(Clone, Debug)]
struct Cell {
    number: u64,
    is_marked: bool,
}

impl FromStr for Cell {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let number = s.parse()?;
        Ok(Self {
            number,
            is_marked: false,
        })
    }
}

#[derive(Clone, Debug)]
struct Board {
    cells: Vec<Cell>,
}

impl FromStr for Board {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells = s
            .lines()
            .flat_map(|line| line.split(' ').filter(|part| !part.is_empty()))
            .map(str::parse::<Cell>)
            .collect::<Result<_, _>>()?;

        Ok(Self { cells })
    }
}

impl Board {
    fn mark(&mut self, number: u64) -> bool {
        let cell_index_option = self.cells.iter().position(|cell| cell.number == number);

        if let Some(cell_index) = cell_index_option {
            let cell: &mut Cell = self.cells.get_mut(cell_index).unwrap();
            cell.is_marked = true;
            self.has_won(cell_index)
        } else {
            false
        }
    }

    fn has_won(&self, cell_index: usize) -> bool {
        let cell_row = cell_index / 5;
        let is_row_marked = (0..5).all(|col| self.cells[cell_row * 5 + col].is_marked);
        if is_row_marked {
            return true;
        }

        let cell_col = cell_index % 5;
        (0..5).all(|row| self.cells[row * 5 + cell_col].is_marked)
    }

    fn score(&self) -> u64 {
        self.cells
            .iter()
            .filter(|cell| !cell.is_marked)
            .map(|cell| cell.number)
            .sum()
    }
}

fn run(numbers: &[u64], boards: Vec<Board>, find_first: bool) -> u64 {
    let mut current_boards: Vec<Board> = boards;
    let mut remaining_boards: Vec<Board> = Vec::with_capacity(current_boards.len());

    for &number in numbers {
        let is_last_board = current_boards.len() == 1;
        for mut board in current_boards {
            let has_won = board.mark(number);
            if has_won {
                if find_first || is_last_board {
                    return board.score() * number;
                }
            } else {
                remaining_boards.push(board);
            }
        }

        current_boards = Vec::new();
        swap(&mut current_boards, &mut remaining_boards);
    }

    panic!("could not win on a board");
}

fn parse(input: &str) -> (Vec<u64>, Vec<Board>) {
    let mut paragraphs = input.split("\n\n");
    let numbers = paragraphs
        .next()
        .unwrap()
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect::<Vec<u64>>();

    let boards = paragraphs
        .map(str::parse)
        .map(Result::unwrap)
        .collect::<Vec<Board>>();

    (numbers, boards)
}

fn main() {
    let input = include_str!("./input.txt");
    let (numbers, boards) = parse(input);
    let boards_cloned = boards.clone();
    println!("Part 1: {}", run(&numbers, boards, true));
    println!("Part 2: {}", run(&numbers, boards_cloned, false));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        let input = include_str!("./example.txt");
        let (numbers, boards) = parse(input);
        assert_eq!(run(&numbers, boards, true), 4512);
    }

    #[test]
    fn example_part_2() {
        let input = include_str!("./example.txt");
        let (numbers, boards) = parse(input);
        assert_eq!(run(&numbers, boards, false), 1924);
    }
}
