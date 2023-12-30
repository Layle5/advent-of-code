fn main() {
    let input = include_str!("input.txt");
    let hash = sum_hashes(input);
    println!("Part 1: {hash}");

    let filled_boxes = process_operations(input);
    let total_focusing_power = sum_focusing_power(filled_boxes);
    println!("Part 2: {}", total_focusing_power);
}

fn sum_focusing_power(boxes: Vec<Box>) -> u64 {
    boxes.into_iter().enumerate().flat_map(|(box_index, current_box)| {
        current_box.lenses.into_iter().enumerate().map(move |(lens_index, current_lens)| {
            (box_index as u64 + 1) * (lens_index as u64 + 1) * (current_lens.focal_length)
        })
    }).sum::<u64>()
}

fn process_operations(input: &str) -> Vec<Box> {
    input.trim().split(',').fold(vec![Box::default(); 256], |mut boxes, operation_str| {
        match parse_operation(operation_str) {
            Operation::Add { label, focal_length } => {
                let hash = hash_word(&label);
                let target_box = &mut boxes[hash as usize];
                if let Some(lens) = target_box.lenses.iter_mut().find(|lens| lens.label == label) {
                    lens.focal_length = focal_length;
                } else {
                    target_box.lenses.push(Lens { label, focal_length });
                }
            }
            Operation::Remove { label } => {
                let hash = hash_word(&label);
                let target_box = &mut boxes[hash as usize];
                target_box.lenses.retain(|lens| lens.label != label);
            }
        }

        boxes
    })
}

fn sum_hashes(input: &str) -> u64 {
    input
        .trim()
        .split(',')
        .map(hash_word)
        .sum::<u64>()
}

fn hash_word(word: &str) -> u64 {
    word.as_bytes()
        .iter()
        .fold(0u64, |hash, &byte| ((hash + byte as u64) * 17) % 256)
}

fn parse_operation(s: &str) -> Operation {
    if let Some(label) = s.strip_suffix("-") {
        Operation::Remove { label: label.to_string() }
    } else if let Some((label, focal_length)) = s.split_once('=') {
        Operation::Add { label: label.to_string(), focal_length: focal_length.parse().expect("focal length is not valid") }
    } else {
        panic!("unrecognized operation")
    }
}

#[derive(Debug, Default, Clone)]
struct Box {
    lenses: Vec<Lens>,
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: u64,
}

#[derive(Debug)]
enum Operation {
    Add { label: String, focal_length: u64 },
    Remove { label: String },
}