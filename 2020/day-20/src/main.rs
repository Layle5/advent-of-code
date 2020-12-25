use std::fs;
use std::{collections::HashMap, env};

use itertools::Itertools;

type Image = Vec<Vec<bool>>;

#[derive(Clone, Debug)]
struct Tile {
    id: u64,
    image: Image,
    transform: Transform,
}

type Flip = u8;
type Rotation = u8;
type Position = (isize, isize);
type Border = u16;
type Borders = Vec<(Position, Border)>;

#[derive(Clone, Copy, Debug)]
struct Transform {
    flip: Flip,
    rotation: Rotation,
}

impl Transform {
    fn new(flip: Flip, rotation: Rotation) -> Transform {
        Transform { flip, rotation }
    }

    fn identity() -> Transform {
        Transform {
            flip: 0,
            rotation: 0,
        }
    }

    fn all() -> Vec<Transform> {
        vec![
            Transform::new(0, 0),
            Transform::new(0, 1),
            Transform::new(0, 2),
            Transform::new(0, 3),
            Transform::new(1, 0),
            Transform::new(1, 1),
            Transform::new(1, 2),
            Transform::new(1, 3),
        ]
    }
}

fn parse_tile(paragraph: &[&str]) -> Tile {
    let id: u64 = paragraph
        .first()
        .and_then(|line| line.strip_prefix("Tile "))
        .and_then(|line| line.strip_suffix(":"))
        .unwrap()
        .parse()
        .unwrap();

    let image: Image = paragraph
        .iter()
        .skip(1)
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

    Tile {
        id,
        image,
        transform: Transform::identity(),
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

fn flip_vertical(image: &mut Image) {
    image.reverse()
}

fn flip_horizontal(image: &mut Image) {
    image.iter_mut().for_each(|line| line.reverse());
}

fn rotate(image: &mut Image) {
    let h = image.len();
    let w = image.first().unwrap().len();
    let mut new_image = vec![];
    for c in 0..w {
        new_image.push(vec![]);
        let line = new_image.last_mut().unwrap();
        for r in (0..h).rev() {
            line.push(image[r][c]);
        }
    }
    *image = new_image;
}

fn apply_transform(mut image: Image, transform: Transform) -> Image {
    let should_flip_vertical = [1, 3].contains(&transform.flip);
    let should_flip_horizontal = [2, 3].contains(&transform.flip);

    if should_flip_vertical {
        flip_vertical(&mut image);
    }
    if should_flip_horizontal {
        flip_horizontal(&mut image);
    }

    for _ in 0..transform.rotation {
        rotate(&mut image);
    }

    image
}

fn apply_transform_tile(tile: Tile, transform: Transform) -> Tile {
    let id = tile.id;
    let image = apply_transform(tile.image, transform);
    Tile {
        id,
        image,
        transform,
    }
}

fn get_borders(tile: &Tile) -> Borders {
    let h = tile.image.len() as isize;
    let w = tile.image.first().unwrap().len() as isize;
    [
        ((-1, 0), (0, 0), (0, 1)),
        ((0, 1), (0, w - 1), (1, 0)),
        ((1, 0), (h - 1, 0), (0, 1)),
        ((0, -1), (0, 0), (1, 0)),
    ]
    .iter()
    .map(|&(position, start, increment)| {
        let mut border = 0;
        let mut current = start;
        while 0 <= current.0 && current.0 < h && 0 <= current.1 && current.1 < w
        {
            let bit = tile.image[current.0 as usize][current.1 as usize];
            border = (border << 1) | bit as Border;
            current.0 += increment.0;
            current.1 += increment.1;
        }
        (position, border)
    })
    .collect()
}

fn find_valid_transforms(
    current_tile: &Tile,
    other_tile: &Tile,
) -> Option<(Position, Tile)> {
    let transformed_tiles: Vec<Tile> = Transform::all()
        .into_iter()
        .map(|transform| apply_transform_tile(other_tile.clone(), transform))
        .collect_vec();

    let borders: Borders = get_borders(current_tile);
    for transformed_tile in transformed_tiles {
        let other_borders: Borders = get_borders(&transformed_tile);
        for (index, &(position, border)) in borders.iter().enumerate() {
            let (_, other_border) = other_borders[(index + 2) % 4];
            if border == other_border {
                return Some((position, transformed_tile));
            }
        }
    }

    None
}

fn get_possible_neighbors(
    tiles: &[Tile],
    tile: &Tile,
) -> Vec<(Position, Tile)> {
    tiles
        .iter()
        .filter(|other_tile| other_tile.id != tile.id)
        .filter_map(|other_tile| find_valid_transforms(tile, other_tile))
        .collect()
}

fn insert_tile_in_image(
    tiles: &[Tile],
    image_info: &mut HashMap<Position, Tile>,
    current_position: Position,
) {
    let current_tile = image_info.get(&current_position).unwrap();
    let possible_neighbors = get_possible_neighbors(tiles, current_tile);
    for (delta_position, neighbor) in possible_neighbors {
        let neighbor_position = (
            current_position.0 + delta_position.0,
            current_position.1 + delta_position.1,
        );
        if !image_info.contains_key(&neighbor_position) {
            image_info.insert(neighbor_position, neighbor);
            insert_tile_in_image(tiles, image_info, neighbor_position);
        }
    }
}

fn parse_monster() -> Image {
    include_str!("../res/monster.txt")
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect()
}

fn is_monster_here(
    image: &Image,
    monster: &Image,
    image_row: usize,
    image_col: usize,
) -> bool {
    for (monster_row, monster_line) in monster.iter().enumerate() {
        for (monster_col, monster_bit) in monster_line.iter().enumerate() {
            if !monster_bit {
                continue;
            }
            let image_bit =
                image[image_row + monster_row][image_col + monster_col];
            if !image_bit {
                return false;
            }
        }
    }
    true
}

fn solve_part_1(content: &str) {
    let tiles = parse_tiles(content);
    let neighbors: HashMap<u64, usize> = tiles
        .iter()
        .map(|tile| (tile.id, get_possible_neighbors(&tiles, tile).len()))
        .collect();

    let corners_id = neighbors
        .into_iter()
        .filter(|(_, neighbor_count)| *neighbor_count == 2)
        .map(|(tile_id, _)| tile_id)
        .collect_vec();

    let corner_product: u64 = corners_id.into_iter().product();
    println!("Part 1: {}", corner_product);
}

fn get_image_tiles_grid(image_info: HashMap<Position, Tile>) -> Vec<Vec<Tile>> {
    let image_tile_height = (image_info.len() as f64).sqrt().round() as usize;
    let image_tile_width = image_tile_height;
    let mut tile_pairs = image_info.into_iter().collect_vec();
    tile_pairs.sort_by_key(|(position, _)| *position);

    (&tile_pairs
        .into_iter()
        .map(|(_, tile)| tile)
        .chunks(image_tile_width))
        .into_iter()
        .map(|chunk| chunk.collect_vec())
        .collect_vec()
}

fn assemble_full_image(tiles: &[Tile], image_tiles: Vec<Vec<Tile>>) -> Image {
    let mut image = vec![];
    let tile_height = tiles.first().unwrap().image.len();
    let tile_width = tiles.first().unwrap().image.first().unwrap().len();
    for image_tiles_line in image_tiles.into_iter() {
        for tile_row in 1..tile_height - 1 {
            let mut image_line = vec![];
            for tile in image_tiles_line.iter() {
                let tile_line = tile.image.get(tile_row).unwrap();
                for tile_col in 1..tile_width - 1 {
                    let bit = tile_line[tile_col];
                    image_line.push(bit);
                }
            }
            image.push(image_line);
        }
    }
    image
}

fn count_monsters(image: &Image, monster: &Image) -> usize {
    let monster_height = monster.len();
    let monster_width = monster.first().unwrap().len();
    let image_height = image.len();
    let image_width = image.first().unwrap().len();
    let mut monster_count = 0;
    for image_row in 0..=image_height - monster_height {
        for image_col in 0..=image_width - monster_width {
            if is_monster_here(&image, &monster, image_row, image_col) {
                monster_count += 1;
            }
        }
    }
    monster_count
}

fn count_monsters_in_transforms(
    image: &Image,
    monster: &Image,
) -> Option<usize> {
    Transform::all()
        .into_iter()
        .map(|transform| apply_transform(image.clone(), transform))
        .map(|transformed_image| count_monsters(&transformed_image, monster))
        .find(|monster_count| *monster_count > 0)
}

fn count_set_bits(image: &Image) -> usize {
    image
        .iter()
        .flat_map(|line| line.iter())
        .filter(|b| **b)
        .count()
}

fn solve_part_2(content: &str) {
    let tiles = parse_tiles(content);

    let mut image_info_map: HashMap<Position, Tile> = HashMap::new();
    let start_tile = tiles.first().unwrap();
    let start_position = (0, 0);
    image_info_map.insert(start_position, start_tile.clone());
    insert_tile_in_image(&tiles, &mut image_info_map, start_position);

    let image_tiles = get_image_tiles_grid(image_info_map);
    let image = assemble_full_image(&tiles, image_tiles);
    let monster = parse_monster();
    let monster_count = count_monsters_in_transforms(&image, &monster).unwrap();
    let monster_size = count_set_bits(&monster);
    let image_count = count_set_bits(&image);
    
    println!("Part 2: {}", image_count - monster_count * monster_size);
}

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
