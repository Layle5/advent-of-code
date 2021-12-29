use std::iter::repeat;

type Algorithm<'a> = &'a [u8];
type Pixel = (usize, usize);

#[derive(Clone)]
struct Image {
    pixels: Vec<Vec<bool>>,
    is_background_lit: bool,
}

fn get_surronding_square(pixel: &Pixel) -> [Pixel; 9] {
    [
        (pixel.0 - 1, pixel.1 - 1),
        (pixel.0 - 1, pixel.1),
        (pixel.0 - 1, pixel.1 + 1),
        (pixel.0, pixel.1 - 1),
        *pixel,
        (pixel.0, pixel.1 + 1),
        (pixel.0 + 1, pixel.1 - 1),
        (pixel.0 + 1, pixel.1),
        (pixel.0 + 1, pixel.1 + 1),
    ]
}

fn is_pixel_lit(image: &Image, pixel: &Pixel) -> bool {
    image
        .pixels
        .get(pixel.0)
        .and_then(|row| row.get(pixel.1))
        .copied()
        .unwrap_or(image.is_background_lit)
}

fn is_output_lit_from_algo(algorithm: Algorithm, index: usize) -> bool {
    algorithm.get(index) == Some(&b'#')
}

fn is_output_lit(image: &Image, algorithm: Algorithm, center_pixel: &Pixel) -> bool {
    let algorithm_index = get_surronding_square(center_pixel)
        .into_iter()
        .map(|square_pixel| is_pixel_lit(image, &square_pixel))
        .fold(0usize, |temp_index, is_lit| {
            temp_index * 2 + is_lit as usize
        });

    is_output_lit_from_algo(algorithm, algorithm_index)
}

fn step_image(old_image: &Image, algorithm: Algorithm) -> Image {
    let mut next_image = old_image.clone();
    for row in 0..next_image.pixels.len() {
        for col in 0..next_image.pixels[row].len() {
            let pixel = (row, col);
            next_image.pixels[row][col] = is_output_lit(old_image, algorithm, &pixel);
        }
    }

    next_image.is_background_lit = is_output_lit_from_algo(
        algorithm,
        if old_image.is_background_lit {
            512 - 1
        } else {
            0
        },
    );

    next_image
}

fn parse(input: &str) -> (Algorithm, Image) {
    let mut lines = input.lines();
    let algorithm = lines.next().unwrap().trim().as_bytes();
    lines.next().unwrap();
    let pixels: Vec<Vec<bool>> = lines
        .map(|line| {
            line.trim()
                .as_bytes()
                .iter()
                .copied()
                .map(|byte| byte == b'#')
                .collect()
        })
        .collect();

    let image = Image {
        pixels,
        is_background_lit: false,
    };

    (algorithm, image)
}

fn prepare_pixels(pixels: Vec<Vec<bool>>, steps: usize) -> Vec<Vec<bool>> {
    let width = pixels.first().unwrap().len();

    repeat(vec![false; width + steps * 2])
        .take(steps)
        .chain(pixels.into_iter().map(|line| {
            repeat(false)
                .take(steps)
                .chain(line.into_iter())
                .chain(repeat(false).take(steps))
                .collect()
        }))
        .chain(repeat(vec![false; width + steps * 2]).take(steps))
        .collect()
}

fn solve(input: &str, steps: usize) -> usize {
    let (algorithm, mut image) = parse(input);
    image.pixels = prepare_pixels(image.pixels, steps);

    for _ in 0..steps {
        image = step_image(&image, algorithm);
    }

    image
        .pixels
        .into_iter()
        .flat_map(|row| row.into_iter())
        .filter(|b| *b)
        .count()
}

fn solve_part_1(input: &str) -> usize {
    solve(input, 2)
}

fn solve_part_2(input: &str) -> usize {
    solve(input, 50)
}

fn main() {
    let input = include_str!("./input.txt");
    println!("Part 1: {}", solve_part_1(input));
    println!("Part 2: {}", solve_part_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("./example.txt");

    #[test]
    fn example_part_1() {
        assert_eq!(solve_part_1(EXAMPLE), 35);
    }

    #[test]
    fn example_part_2() {
        assert_eq!(solve_part_2(EXAMPLE), 3351);
    }
}
