use itertools::Itertools;
use std::fmt::Display;
use std::{iter::once, str::FromStr};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Tile {
    Empty,
    Wall,
    Start(char),
    Key(char),
    Door(char),
}

impl Tile {
    pub fn is_node(&self) -> bool {
        match self {
            Tile::Empty | Tile::Wall => false,
            Tile::Start(_) | Tile::Key(_) | Tile::Door(_) => true,
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chr: char = self.into();
        write!(f, "{}", chr)?;
        Ok(())
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '@' => Tile::Start(c),
            '#' => Tile::Wall,
            c if b'a' <= c as u8 && c as u8 <= b'z' => Tile::Key(c),
            c if b'A' <= c as u8 && c as u8 <= b'Z' => Tile::Door(c),
            _ => panic!(),
        }
    }
}

impl Into<char> for &Tile {
    fn into(self) -> char {
        match self {
            Tile::Empty => '.',
            Tile::Wall => '#',
            Tile::Start(c) => *c,
            Tile::Key(c) => *c,
            Tile::Door(c) => *c,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Position {
        Position { row, col }
    }

    pub fn neighbors(&self) -> impl Iterator<Item = Position> + '_ {
        once((-1, 0))
            .chain(once((1, 0)))
            .chain(once((0, -1)))
            .chain(once((0, 1)))
            .filter(move |(d_row, _)| *d_row >= 0 || self.row >= 1)
            .filter(move |(_, d_col)| *d_col >= 0 || self.col >= 1)
            .map(move |(d_row, d_col)| {
                (self.row as isize + d_row, self.col as isize + d_col)
            })
            .map(|(n_row, n_col)| Position::new(n_row as usize, n_col as usize))
    }
}

#[derive(Debug)]
pub struct Grid {
    pub starts: Vec<Position>,
    pub keys: Vec<Position>,
    pub tiles: Vec<Vec<Tile>>,
}

impl Grid {
    pub fn start(&self) -> &Position {
        self.starts.first().unwrap()
    }

    pub fn iter_tiles(&self) -> impl Iterator<Item = (Position, &Tile)> + '_ {
        self.tiles.iter().enumerate().flat_map(|(row, line)| {
            line.iter()
                .enumerate()
                .map(move |(col, tile)| (Position::new(row, col), tile))
        })
    }

    pub fn neighbors<'a>(
        &'a self,
        pos: &'a Position,
    ) -> impl Iterator<Item = (Position, &Tile)> + 'a {
        pos.neighbors().filter_map(move |n| {
            self.tiles
                .get(n.row)
                .and_then(|line| line.get(n.col))
                .map(|tile| (n, tile))
        })
    }

    pub fn split_four(mut self) -> Grid {
        let start = self.starts.first().unwrap();
            self.tiles[start.row][start.col] = Tile::Wall;
        for neighbor in start.neighbors() {
            self.tiles[neighbor.row][neighbor.col] = Tile::Wall;
        }
        let new_starts = vec![
            Position::new(start.row - 1, start.col - 1),
            Position::new(start.row - 1, start.col + 1),
            Position::new(start.row + 1, start.col - 1),
            Position::new(start.row + 1, start.col + 1),
        ];
        let chrs = vec!['@', '$', '%', '&'];
        for (new_start, start_chr) in new_starts.iter().zip(chrs.into_iter()) {
            self.tiles[new_start.row][new_start.col] = Tile::Start(start_chr);
        }
        Grid {
            starts: new_starts,
            keys: self.keys,
            tiles: self.tiles,
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for tile in self.tiles.first().unwrap() {
            write!(f, "{}", tile)?;
        }
        for line in self.tiles.iter().skip(1) {
            writeln!(f)?;
            for tile in line {
                write!(f, "{}", tile)?;
            }
        }
        Ok(())
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start_op = None;
        let mut keys = vec![];

        let tiles = s
            .as_bytes()
            .split(|byte| *byte == b'\n')
            .enumerate()
            .filter(|(_, line)| !line.is_empty())
            .map(|(row, line)| {
                line.iter()
                    .copied()
                    .enumerate()
                    .inspect(|(col, byte)| {
                        if *byte == b'@' {
                            start_op = Some(Position::new(row, *col))
                        }
                    })
                    .map(|(col, byte)| (col, (byte as char).into()))
                    .inspect(|(col, tile)| {
                        if let Tile::Key(_) = tile {
                            keys.push(Position::new(row, *col))
                        }
                    })
                    .map(|(_, tile)| tile)
                    .collect_vec()
            })
            .collect_vec();

        Ok(Grid {
            starts: vec![start_op.unwrap()],
            keys,
            tiles,
        })
    }
}
