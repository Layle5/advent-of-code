use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let image = parse_image(input);
    let shortest_distances_sum = sum_shortest_distances(&image, 2);
    println!("Part 1: {}", shortest_distances_sum);
    let shortest_distances_sum = sum_shortest_distances(&image, 1000000);
    println!("Part 2: {}", shortest_distances_sum);
}

fn sum_shortest_distances(image: &Image, displacement_factor: usize) -> usize {
    image
        .galaxies
        .iter()
        .tuple_combinations()
        .map(|(lhs, rhs)| compute_shortest_distance(image, displacement_factor, lhs, rhs))
        .sum::<usize>()
}

fn compute_shortest_distance(
    image: &Image,
    displacement_factor: usize,
    lhs: &Position,
    rhs: &Position,
) -> usize {
    let row_distance = lhs.0.abs_diff(rhs.0);
    let row_displacement =
        image.rows_displacements[lhs.0].abs_diff(image.rows_displacements[rhs.0]);

    let col_distance = lhs.1.abs_diff(rhs.1);
    let col_displacement =
        image.cols_displacements[lhs.1].abs_diff(image.cols_displacements[rhs.1]);

    row_distance + col_distance + (displacement_factor - 1) * (row_displacement + col_displacement)
}

fn parse_image(input: &str) -> Image {
    let grid = input
        .trim()
        .lines()
        .map(|row| row.trim().chars().collect_vec())
        .collect_vec();

    let mut galaxies = vec![];
    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, &tile) in row.iter().enumerate() {
            if tile == TILE_GALAXY {
                galaxies.push((row_index, col_index));
            }
        }
    }

    let rows_number = grid.len();
    let rows_displacements = (0..rows_number)
        .scan(0, |displacement, row_index| {
            let is_empty = grid[row_index].iter().all(|&tile| tile == TILE_EMPTY);
            *displacement += is_empty as usize;
            Some(*displacement)
        })
        .collect_vec();

    let cols_number = grid[0].len();
    let cols_displacements = (0..cols_number)
        .scan(0, |displacement, col_index| {
            let is_empty = grid.iter().all(|row| row[col_index] == TILE_EMPTY);
            *displacement += is_empty as usize;
            Some(*displacement)
        })
        .collect_vec();

    Image {
        galaxies,
        rows_displacements,
        cols_displacements,
    }
}

const TILE_EMPTY: char = '.';
const TILE_GALAXY: char = '#';

#[derive(Debug)]
struct Image {
    galaxies: Vec<Position>,
    rows_displacements: Vec<usize>,
    cols_displacements: Vec<usize>,
}

type Position = (usize, usize);
