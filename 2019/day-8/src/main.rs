use itertools::Itertools;
use std::env;
use std::fs;

type Pixel = char;
type Layer = Vec<Pixel>;

fn parse_layers(content: &str, width: usize, height: usize) -> Vec<Layer> {
    content
        .trim()
        .chars()
        .chunks(width * height)
        .into_iter()
        .map(Itertools::collect_vec)
        .collect_vec()
}

fn solve_part_1(content: &str) {
    let width = 25;
    let height = 6;
    let layers = parse_layers(content, width, height);

    let product = layers
        .iter()
        .map(|layer| layer.iter().counts())
        .min_by_key(|counts| counts[&'0'])
        .map(|counts| counts[&'1'] * counts[&'2'])
        .unwrap();

    println!("Part 1: {}", product)
}

fn solve_part_2_impl(content: &str, width: usize, height: usize) -> String {
    let layers = parse_layers(content, width, height);

    let size = width * height;
    (0..size)
        .map(|index| {
            layers
                .iter()
                .map(|layer| layer[index])
                .find(|pixel| *pixel != '2')
                .unwrap()
        })
        .map(|pixel| if pixel == '1' { '\u{2588}' } else { ' ' })
        .chunks(width)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .join("\n")
}

fn solve_part_2(content: &str) {
    let image = solve_part_2_impl(content, 25, 6);
    println!("Part 2:\n{}", image)
}

fn main() {
    let args = env::args().collect_vec();
    let filename = args.get(1).map(|s| s.as_ref()).unwrap_or("./res/input.txt");

    let content = fs::read_to_string(filename).unwrap();

    solve_part_1(&content);
    solve_part_2(&content);
}
