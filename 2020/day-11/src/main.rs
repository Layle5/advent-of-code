use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq)]
enum GridTile {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

#[derive(Debug)]
struct Grid {
    tiles: Vec<Vec<GridTile>>,
}

fn add(u: usize, i: i64) -> usize {
    if i < 0 {
        u - ((-i) as usize)
    } else {
        u + (i as usize)
    }
}

impl Grid {
    fn new(height: usize, width: usize) -> Grid {
        Grid {
            tiles: vec![vec![GridTile::Floor; width]; height],
        }
    }

    fn height(&self) -> usize {
        self.tiles.len()
    }

    fn width(&self) -> usize {
        self.tiles.first().unwrap().len()
    }

    fn get_immediate_neightbors(
        &self,
        row: usize,
        col: usize,
    ) -> Vec<(usize, usize)> {
        (-1..=1)
            .into_iter()
            .filter(|r| row > 0 || *r >= 0)
            .filter(|r| add(row, *r) < self.height())
            .flat_map(|r| {
                (-1..=1)
                    .into_iter()
                    .filter(move |c| col > 0 || *c >= 0)
                    .filter(move |c| add(col, *c) < self.width())
                    .map(move |c| (add(row, r), add(col, c)))
            })
            .filter(|n| *n != (row, col))
            .collect()
    }

    fn get_far_neighbor(
        &self,
        row: usize,
        col: usize,
        increments: (i64, i64),
    ) -> Option<(usize, usize)> {
        let mut neighbor_row: usize = row;
        let mut neighbor_col: usize = col;
        let (row_increment, col_increment) = increments;
        loop {
            if (neighbor_row as i64) < -row_increment
                || (neighbor_col as i64) < -col_increment
            {
                break;
            }

            let next_neighbor_row = add(neighbor_row, row_increment);
            let next_neighbor_col = add(neighbor_col, col_increment);

            if self.height() <= next_neighbor_row
                || self.width() <= next_neighbor_col
            {
                break;
            }

            neighbor_row = next_neighbor_row;
            neighbor_col = next_neighbor_col;

            let neighbor_tile = self.tiles[neighbor_row][neighbor_col];
            if neighbor_tile != GridTile::Floor {
                break;
            }
        }

        if neighbor_row == row && neighbor_col == col {
            return None;
        }

        let neighbor_tile = self.tiles[neighbor_row][neighbor_col];
        if neighbor_tile != GridTile::Floor {
            Some((neighbor_row, neighbor_col))
        } else {
            None
        }
    }

    fn get_far_neightbors(
        &self,
        row: usize,
        col: usize,
    ) -> Vec<(usize, usize)> {
        (-1..=1)
            .into_iter()
            .flat_map(|r| (-1..=1).into_iter().map(move |c| (r, c)))
            .filter(|(r, c)| *r != 0 || *c != 0)
            .map(|direction| self.get_far_neighbor(row, col, direction))
            .filter(|n| n.is_some())
            .map(|n| n.unwrap())
            .collect()
    }

    fn count_occupied_seat(&self) -> usize {
        self.tiles
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|&&tile| tile == GridTile::OccupiedSeat)
                    .count()
            })
            .sum()
    }
}

impl FromStr for Grid {
    type Err = ();
    fn from_str(content: &str) -> Result<Self, <Self as FromStr>::Err> {
        let tiles = content
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => GridTile::Floor,
                        'L' => GridTile::EmptySeat,
                        '#' => GridTile::OccupiedSeat,
                        _ => panic!("Could not recognized {} tile", c),
                    })
                    .collect()
            })
            .collect();

        Ok(Grid { tiles })
    }
}

impl ToString for Grid {
    fn to_string(&self) -> String {
        self.tiles
            .iter()
            .flat_map(|row| {
                row.iter()
                    .map(|tile| match tile {
                        GridTile::Floor => '.',
                        GridTile::EmptySeat => 'L',
                        GridTile::OccupiedSeat => '#',
                    })
                    .chain(['\n'].iter().cloned())
            })
            .collect()
    }
}

type GridIteratorItem = (usize, usize, GridTile);
struct GridIterator<'a> {
    grid: &'a Grid,
    row: usize,
    col: usize,
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = GridIteratorItem;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.grid.height() <= self.row {
            None
        } else {
            let item_tile = self.grid.tiles[self.row][self.col];
            let item = (self.row, self.col, item_tile);
            self.col += 1;
            if self.grid.width() <= self.col {
                self.col = 0;
                self.row += 1;
            }
            Some(item)
        }
    }
}

impl<'a> IntoIterator for &'a Grid {
    type Item = (usize, usize, GridTile);
    type IntoIter = GridIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        GridIterator {
            grid: self,
            row: 0,
            col: 0,
        }
    }
}

fn apply_grid_rules(
    old_grid: &Grid,
    new_grid: &mut Grid,
    tolerance: usize,
    get_neighbors: fn(&Grid, usize, usize) -> Vec<(usize, usize)>,
) -> usize {
    let mut number_changes = 0;

    for (tile_row, tile_col, old_tile) in old_grid {
        let neighbors = get_neighbors(old_grid, tile_row, tile_col);
        let occupied_neighbors_count = neighbors
            .into_iter()
            .map(|(r, c)| old_grid.tiles[r][c])
            .filter(|&t| t == GridTile::OccupiedSeat)
            .count();

        let new_tile = match old_tile {
            GridTile::EmptySeat if occupied_neighbors_count == 0 => {
                number_changes += 1;
                GridTile::OccupiedSeat
            }
            GridTile::OccupiedSeat if occupied_neighbors_count >= tolerance => {
                number_changes += 1;
                GridTile::EmptySeat
            }
            _ => old_tile,
        };

        new_grid.tiles[tile_row][tile_col] = new_tile;
    }

    number_changes
}

fn converge_grid(
    mut old_grid: Grid,
    tolerance: usize,
    get_neighbors: fn(&Grid, usize, usize) -> Vec<(usize, usize)>,
) -> Grid {
    let mut new_grid = Grid::new(old_grid.height(), old_grid.width());
    loop {
        let number_changes = apply_grid_rules(
            &old_grid,
            &mut new_grid,
            tolerance,
            get_neighbors,
        );
        if number_changes == 0 {
            return old_grid;
        }

        let tmp = old_grid;
        old_grid = new_grid;
        new_grid = tmp;
    }
}

fn solve_part_1(content: &str) {
    let start_grid: Grid = content.parse().unwrap();
    let final_grid = converge_grid(start_grid, 4, |grid, row, col| {
        grid.get_immediate_neightbors(row, col)
    });
    println!("Part 1: {}", final_grid.count_occupied_seat());
}

fn solve_part_2(content: &str) {
    let start_grid: Grid = content.parse().unwrap();
    let final_grid = converge_grid(start_grid, 5, |grid, row, col| {
        grid.get_far_neightbors(row, col)
    });
    println!("Part 2: {}", final_grid.count_occupied_seat());
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
    use super::*;
    #[test]
    fn get_immediate_neightbors_test() {
        let grid = Grid::from_str("...\n...\n...\n").unwrap();
        let assert = |r, c, e: &[(usize, usize)]| {
            assert_eq!(grid.get_immediate_neightbors(r, c), e);
        };

        assert(0, 0, &[(0, 1), (1, 0), (1, 1)]);
        assert(0, 1, &[(0, 0), (0, 2), (1, 0), (1, 1), (1, 2)]);
    }
    #[test]
    fn get_far_neightbors_test() {
        let grid =
            Grid::from_str("#...#\n.....\n.....\n.....\n#...#\n").unwrap();

        let assert = |r, c, e: &[(usize, usize)]| {
            assert_eq!(grid.get_far_neightbors(r, c), e);
        };

        assert(0, 0, &[(0, 4), (4, 0), (4, 4)]);
        assert(0, 4, &[(0, 0), (4, 0), (4, 4)]);
    }
}
