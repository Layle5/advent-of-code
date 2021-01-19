#![feature(str_split_once)]

use itertools::Itertools;
use std::{collections::HashMap, fs};
use std::{env, str::FromStr};

type Chemical<'a> = &'a str;

#[derive(Debug)]
struct RecipePart<'a> {
    quantity: usize,
    chemical: Chemical<'a>,
}

#[derive(Debug)]
struct Recipe<'a> {
    inputs: Vec<RecipePart<'a>>,
    output: RecipePart<'a>,
}

type Book<'a> = HashMap<Chemical<'a>, Recipe<'a>>;

type QuantityMap<'a> = HashMap<Chemical<'a>, usize>;

fn parse_recipe_part(s: &str) -> RecipePart {
    let (quantity_str, chemical) = s.trim().split_once(" ").unwrap();
    let quantity = quantity_str.parse().unwrap();
    RecipePart { quantity, chemical }
}

fn check_storage<'a>(
    book: &Book<'a>,
    produced: &mut QuantityMap<'a>,
    storage: &mut QuantityMap<'a>,
    quantity_to_have: usize,
    chemical: &'a str,
) -> bool {
    let quantity_stored = storage.get(chemical).copied().unwrap_or(0);
    if quantity_to_have <= quantity_stored {
        return true;
    }

    produce_to_storage(book, produced, storage, quantity_to_have, chemical)
}

fn produce_to_storage<'a>(
    book: &Book<'a>,
    produced: &mut QuantityMap<'a>,
    storage: &mut QuantityMap<'a>,
    quantity_to_have: usize,
    chemical: &'a str,
) -> bool {
    let quantity_stored = storage.get(chemical).copied().unwrap_or(0);
    let recipe_op = book.get(chemical);
    if recipe_op.is_none() {
        return false;
    }

    let recipe = recipe_op.unwrap();

    let quantity_delta = quantity_to_have - quantity_stored;

    let count_reaction = |p, q| (p + q - 1) / q;
    let number_reaction =
        count_reaction(quantity_delta, recipe.output.quantity);

    let mut storage_clone = storage.clone();
    for part in &recipe.inputs {
        let input_quantity_to_have = number_reaction * part.quantity;

        let produced_input = check_storage(
            book,
            produced,
            &mut storage_clone,
            input_quantity_to_have,
            part.chemical,
        );

        if !produced_input {
            return false;
        }

        *storage_clone.get_mut(part.chemical).unwrap() -=
            input_quantity_to_have;
    }

    *storage = storage_clone;

    let quantity_to_produce = number_reaction * recipe.output.quantity;

    produced
        .entry(chemical)
        .and_modify(|q| *q += quantity_to_produce)
        .or_insert(quantity_to_produce);

    storage
        .entry(chemical)
        .and_modify(|q| *q += quantity_to_produce)
        .or_insert(quantity_to_produce);

    true
}

fn parse_recipe_book(content: &str) -> Book {
    content
        .lines()
        .map(|line| {
            let mid = line.find(" => ").unwrap();
            let (inputs_str, output_str) = line.split_at(mid);
            let inputs = inputs_str
                .split(',')
                .map(|s| parse_recipe_part(s))
                .collect_vec();
            let output =
                parse_recipe_part(output_str.strip_prefix(" => ").unwrap());
            Recipe { inputs, output }
        })
        .map(|recipe| (recipe.output.chemical, recipe))
        .collect()
}

fn solve_part_1(content: &str) {
    let mut book = parse_recipe_book(content);

    book.insert(
        "ORE",
        Recipe {
            inputs: vec![],
            output: RecipePart {
                quantity: 1,
                chemical: "ORE",
            },
        },
    );

    let mut storage = QuantityMap::new();
    let mut produced = QuantityMap::new();

    check_storage(&book, &mut produced, &mut storage, 1, "FUEL");

    println!("Part 1: {}", produced["ORE"]);
}

fn solve_part_2(content: &str) {
    let book = parse_recipe_book(content);

    let mut min = 0;
    let mut max = 1000000000000;

    loop {
        let mut storage = QuantityMap::new();
        let mut produced = QuantityMap::new();
        storage.insert("ORE", 1000000000000);

        let mid = min + (max - min) / 2;
        if min == mid {
            break;
        }

        let produced =
            produce_to_storage(&book, &mut produced, &mut storage, mid, "FUEL");
        if produced {
            min = mid;
        } else {
            max = mid;
        }
    }

    println!("Part 2: {}", min);
}

fn main() {
    let args = env::args().collect_vec();
    let filename = args.get(1).map(|s| s.as_ref()).unwrap_or("./res/input.txt");

    let content = fs::read_to_string(filename).unwrap();

    solve_part_1(&content);
    solve_part_2(&content);
}
