use std::fs;
use std::{
    collections::HashMap,
    env,
    hash::{Hash, Hasher},
};

type Borders = [u16; 4];

use itertools::Itertools;

#[derive(Debug)]
struct Tile {
    id: u64,
    image: Vec<Vec<bool>>,
    normal_borders: Borders,
    reversed_borders: Borders,
}

#[derive(Debug)]
struct Link {
    rotate_rhs: usize,
    flip_rhs: Flip,
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Eq for Tile {}

impl Hash for Tile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

#[derive(Clone, Copy, Debug)]
enum Flip {
    Normal,
    // Up becomes down, and vice versa
    Vertical,
    // Left becomes right, and vice versa
    Horizontal,
    Both,
}

fn get_border(
    image: &[Vec<bool>],
    mut row: isize,
    mut col: isize,
    row_inc: isize,
    col_inc: isize,
) -> u16 {
    let border_size = image.len() as isize;
    let mut border = 0;
    loop {
        if row < 0 || col < 0 || border_size <= row || border_size <= col {
            break;
        }

        let bit = image[row as usize][col as usize] as u16;
        border = (border << 1) | bit;

        row += row_inc;
        col += col_inc;
    }
    border
}

fn parse_tile(paragraph: &[&str]) -> Tile {
    let id: u64 = paragraph
        .first()
        .and_then(|line| line.strip_prefix("Tile "))
        .and_then(|line| line.strip_suffix(":"))
        .unwrap()
        .parse()
        .unwrap();

    let image: Vec<Vec<bool>> = paragraph
        .iter()
        .skip(1)
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

    let border_size = image.len();
    let border_isize = border_size as isize;
    let normal_borders = [
        get_border(&image, 0, 0, 0, 1),
        get_border(&image, 0, border_isize - 1, 1, 0),
        get_border(&image, border_isize - 1, border_isize - 1, 0, -1),
        get_border(&image, border_isize - 1, 0, -1, 0),
    ];
    let reversed_borders = [
        get_border(&image, 0, border_isize - 1, 0, -1),
        get_border(&image, border_isize - 1, border_isize - 1, -1, 0),
        get_border(&image, border_isize - 1, 0, 0, 1),
        get_border(&image, 0, 0, 1, 0),
    ];

    Tile {
        id,
        image,
        normal_borders,
        reversed_borders,
    }
}

fn parse_tiles(content: &str) -> Vec<Tile> {
    let lines: Vec<&str> = content.lines().collect();
    let paragraphs: Vec<&[&str]> =
        lines.split(|line| line.is_empty()).collect();

    paragraphs
        .into_iter()
        .filter(|paragraph| !paragraph.is_empty())
        .map(|paragraph| parse_tile(paragraph))
        .collect()
}

fn find_links(lhs_tile: &Tile, rhs_tile: &Tile) -> Vec<Link> {
    if lhs_tile.id == rhs_tile.id {
        return vec![];
    }

    let rhs_borders_pairs = [
        (rhs_tile.normal_borders, false),
        (rhs_tile.reversed_borders, true),
    ];

    let mut links = Vec::new();

    let lhs_borders = lhs_tile.normal_borders;
    for (lhs_border_index, lhs_border) in lhs_borders.iter().enumerate() {
        for (rhs_borders, is_rhs_reversed) in rhs_borders_pairs.iter() {
            for (rhs_border_index, rhs_border) in rhs_borders.iter().enumerate()
            {
                if lhs_border == rhs_border {
                    let rotate_rhs =
                        (lhs_border_index + 4 - rhs_border_index) % 4;
                    let flip_rhs = match (lhs_border_index % 2, is_rhs_reversed)
                    {
                        (0, false) => Flip::Vertical,
                        (0, true) => Flip::Both,
                        (_, false) => Flip::Horizontal,
                        (_, true) => Flip::Both,
                    };
                    links.push(Link {
                        rotate_rhs,
                        flip_rhs,
                    });
                }
            }
        }
    }

    links
}

fn find_possible_adjacents<'a>(
    tiles: &'a [Tile],
    tile: &Tile,
) -> Vec<&'a Tile> {
    tiles
        .iter()
        .filter(|other_tile| {
            let links = find_links(tile, other_tile);
            !links.is_empty()
        })
        .collect()
}

fn solve_part_1(content: &str) {
    let tiles = parse_tiles(content);

    let adjacents_id: HashMap<&Tile, Vec<&Tile>> = tiles
        .iter()
        .map(|tile| {
            let adjacents_id = find_possible_adjacents(&tiles, tile);
            (tile, adjacents_id)
        })
        .collect();

    let corners_id: Vec<u64> = adjacents_id
        .iter()
        .filter(|(_, count)| count.len() == 2)
        .map(|(tile, _)| tile.id)
        .collect();

    let product_id: u64 = corners_id.iter().product();

    println!("Part 1: {}", product_id);
}

fn solve_part_2(_content: &str) {}

fn get_content(index: usize, default_filename: &str) -> String {
    let args: Vec<String> = env::args().collect();
    let filename: &str = args
        .get(index)
        .map(|s| s.as_ref())
        .unwrap_or(default_filename);

    fs::read_to_string(filename).expect("Something went wrong reading the file")
}

fn main() {
    let content = get_content(1, "./res/input.txt");
    solve_part_1(&content);
    solve_part_2(&content);
}

#[cfg(test)]
mod tests {}
