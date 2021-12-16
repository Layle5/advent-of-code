use std::{collections::HashSet, fmt::Display, str::FromStr};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Position {
    col: usize,
    row: usize,
}

impl FromStr for Position {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (col_str, row_str) = s.trim().split_once(',').ok_or(())?;
        let col = col_str.parse().map_err(|_| ())?;
        let row = row_str.parse().map_err(|_| ())?;
        Ok(Self { row, col })
    }
}

impl Position {
    fn fold(&self, fold: Fold) -> Option<Self> {
        let options = match fold {
            Fold::Left(fold_col) => {
                let col_option = if self.col <= fold_col {
                    Some(self.col)
                } else {
                    let col_delta = self.col - fold_col;
                    self.col.checked_sub(col_delta * 2)
                };
                (col_option, Some(self.row))
            }
            Fold::Up(fold_row) => {
                let row_option = if self.row <= fold_row {
                    Some(self.row)
                } else {
                    let row_delta = self.row - fold_row;
                    self.row.checked_sub(row_delta * 2)
                };
                (Some(self.col), row_option)
            }
        };

        if let (Some(col), Some(row)) = options {
            Some(Self { col, row })
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Paper {
    dots: HashSet<Position>,
}

impl FromStr for Paper {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dots = s.trim().lines().map(str::parse).collect::<Result<_, _>>()?;
        Ok(Self { dots })
    }
}

#[derive(Clone, Copy, Debug)]
enum Fold {
    Left(usize),
    Up(usize),
}

impl FromStr for Fold {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().strip_prefix("fold along ") {
            None => Err(()),
            Some(rest) => {
                let (axis, number_str) = rest.split_once('=').ok_or(())?;
                let f = match axis {
                    "x" => Fold::Left,
                    "y" => Fold::Up,
                    _ => return Err(()),
                };
                let number = number_str.parse().map_err(|_| ())?;
                Ok(f(number))
            }
        }
    }
}

impl Paper {
    fn fold(&self, fold: Fold) -> Self {
        let dots = HashSet::from_iter(self.dots.iter().flat_map(|dot| dot.fold(fold)));
        Self { dots }
    }
}

impl Display for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (width, height) = self.dots.iter().fold((1, 1), |(w, h), dot| {
            (w.max(dot.col + 1), h.max(dot.row + 1))
        });

        for row in 0..height {
            for col in 0..width {
                let is_marked = self.dots.contains(&Position { row, col });
                let c = if is_marked { '#' } else { '.' };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn parse(s: &str) -> (Paper, Vec<Fold>) {
    let (paper_str, folds_str) = s.split_once("\n\n").unwrap();
    let paper = paper_str.parse().unwrap();
    let folds = folds_str
        .trim()
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    (paper, folds)
}

fn main() {
    let input = include_str!("./input.txt");
    let (initial_paper, folds) = parse(input);

    let folded_paper = initial_paper.fold(*folds.first().unwrap());
    println!("Part 1: {}", folded_paper.dots.len());

    let final_paper = folds
        .into_iter()
        .fold(folded_paper, |temp_paper, fold| temp_paper.fold(fold));
    print!("Part 2:\n{}", final_paper);
}
