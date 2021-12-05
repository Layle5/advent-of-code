use std::{
    cmp::Ordering,
    fmt::Display,
    num::ParseIntError,
    ops::{Index, IndexMut},
    str::FromStr,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Position {
    col: usize,
    row: usize,
}

#[derive(Debug)]
enum ParsePositionError {
    SplitError,
    ParseIntError(ParseIntError),
}

impl FromStr for Position {
    type Err = ParsePositionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (col_str, row_str) = s.split_once(',').ok_or(ParsePositionError::SplitError)?;
        let col = col_str.parse().map_err(ParsePositionError::ParseIntError)?;
        let row = row_str.parse().map_err(ParsePositionError::ParseIntError)?;
        Ok(Self { row, col })
    }
}

#[derive(Debug)]
struct Segment {
    start: Position,
    end: Position,
}

#[derive(Debug)]
enum ParseSegmentError {
    SplitError,
    ParsePositionError(ParsePositionError),
}

impl FromStr for Segment {
    type Err = ParseSegmentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start_str, end_str) = s.split_once(" -> ").ok_or(ParseSegmentError::SplitError)?;
        let start = start_str
            .parse()
            .map_err(ParseSegmentError::ParsePositionError)?;
        let end = end_str
            .parse()
            .map_err(ParseSegmentError::ParsePositionError)?;
        Ok(Self { start, end })
    }
}

fn get_increment(start: usize, end: usize) -> isize {
    match start.cmp(&end) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    }
}

impl Segment {
    fn is_vertical(&self) -> bool {
        self.start.col == self.end.col
    }

    fn is_horizontal(&self) -> bool {
        self.start.row == self.end.row
    }

    fn is_straight(&self) -> bool {
        self.is_vertical() || self.is_horizontal()
    }

    fn iter_positions(&self) -> IterPositions {
        IterPositions::from_positions(self.start, self.end)
    }
}

struct IterPositions {
    current: Position,
    target: Position,
    increment_col: isize,
    increment_row: isize,
    done: bool,
}

impl IterPositions {
    fn from_positions(start: Position, end: Position) -> Self {
        let current = start;
        let increment_col = get_increment(start.col, end.col);
        let increment_row = get_increment(start.row, end.row);
        let target = end;
        Self {
            current,
            target,
            increment_col,
            increment_row,
            done: false,
        }
    }
}

impl Iterator for IterPositions {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            let next = self.current;
            if next != self.target {
                self.current.col = (self.current.col as isize + self.increment_col) as usize;
                self.current.row = (self.current.row as isize + self.increment_row) as usize;
            } else {
                self.done = true;
            }
            Some(next)
        }
    }
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    cells: Vec<u64>,
}

impl Grid {
    fn from_dimensions(width: usize, height: usize) -> Self {
        let cells = vec![0; width * height];
        Self {
            width,
            height,
            cells,
        }
    }

    fn cell_index(&self, position: &Position) -> usize {
        position.row * self.width + position.col
    }

    fn mark(&mut self, segment: &Segment) {
        segment
            .iter_positions()
            .for_each(|position| *self.index_mut(&position) += 1)
    }
}

impl Index<&Position> for Grid {
    type Output = u64;

    fn index(&self, index: &Position) -> &Self::Output {
        self.cells.index(self.cell_index(index))
    }
}

impl IndexMut<&Position> for Grid {
    fn index_mut(&mut self, index: &Position) -> &mut Self::Output {
        self.cells.index_mut(self.cell_index(index))
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut row_strings = self.cells.chunks(self.width).map(|row_cells| {
            row_cells
                .iter()
                .map(|&cell| {
                    if cell > 9 {
                        '+'
                    } else if cell == 0 {
                        '.'
                    } else {
                        (b'0' + cell as u8) as char
                    }
                })
                .collect::<String>()
        });

        let first_row_string = row_strings.next().unwrap();
        first_row_string.fmt(f)?;
        for row_string in row_strings {
            "\n".fmt(f)?;
            row_string.fmt(f)?;
        }

        Ok(())
    }
}

fn parse(input: &str) -> Vec<Segment> {
    input.lines().map(str::parse).map(Result::unwrap).collect()
}

fn find_max_dimension<F>(segments: &[Segment], get_dimension: F) -> usize
where
    F: Fn(&Position) -> usize,
{
    segments
        .iter()
        .map(|segment| usize::max(get_dimension(&segment.start), get_dimension(&segment.end)))
        .max()
        .unwrap()
}

fn solve(segments: &[Segment], keep_diagonals: bool) -> usize {
    let max_col = find_max_dimension(segments, |position| position.col);
    let max_row = find_max_dimension(segments, |position| position.row);
    let mut grid = Grid::from_dimensions(max_col + 1, max_row + 1);

    let s1 = segments.iter();
    let s2: Box<dyn Iterator<Item = &Segment>> = if keep_diagonals {
        Box::new(s1)
    } else {
        Box::new(s1.filter(|segment| keep_diagonals || segment.is_straight()))
    };

    s2.for_each(|segment| grid.mark(segment));

    let intersection_count = grid.cells.iter().filter(|&&cell| cell > 1).count();
    intersection_count
}

fn main() {
    let input = include_str!("./input.txt");
    let segments = parse(input);
    println!("Part 1: {}", solve(&segments, false));
    println!("Part 2: {}", solve(&segments, true));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_and_solve(input: &str, keep_diagonals: bool) -> usize {
        let segments: Vec<Segment> = parse(input);
        solve(&segments, keep_diagonals)
    }

    #[test]
    fn example_part_1() {
        let input = include_str!("./example.txt");
        assert_eq!(parse_and_solve(input, false), 5);
    }

    #[test]
    fn example_part_2() {
        let input = include_str!("./example.txt");
        assert_eq!(parse_and_solve(input, true), 12);
    }
}
