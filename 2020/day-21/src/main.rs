use std::fs;
use std::{
    collections::{HashMap, HashSet},
    env,
};

#[macro_use]
extern crate lazy_static;

use itertools::Itertools;
use regex::Regex;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Ingredient<'a>(&'a str);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Allergen<'a>(&'a str);

#[derive(Debug)]
struct Food<'a> {
    ingredients: Vec<Ingredient<'a>>,
    allergens: Vec<Allergen<'a>>,
}

fn parse_foods(content: &str) -> Vec<Food> {
    lazy_static! {
        static ref REGEX: Regex =
            Regex::new(r"^(.*) \(contains (.*)\)$").unwrap();
    }

    content
        .lines()
        .map(|line| {
            let captures = REGEX.captures(line).unwrap();
            let ingredients_str = captures.get(1).unwrap().as_str();
            let allergens_str = captures.get(2).unwrap().as_str();
            let ingredients = ingredients_str
                .split(' ')
                .map(|ingredient_str| Ingredient(ingredient_str))
                .collect_vec();
            let allergens = allergens_str
                .split(", ")
                .map(|allergen_str| Allergen(allergen_str))
                .collect_vec();
            Food {
                ingredients,
                allergens,
            }
        })
        .collect_vec()
}

fn solve_part_1(content: &str) {
    let foods = parse_foods(content);
    let allergens: HashSet<Allergen> = foods
        .iter()
        .flat_map(|food| food.allergens.iter().copied())
        .collect();

    let allergens_foods_map: HashMap<Allergen, Vec<&Food>> = allergens
        .iter()
        .copied()
        .map(|allergen| {
            let allergen_foods = foods
                .iter()
                .filter(|food| food.allergens.contains(&allergen))
                .collect();
            (allergen, allergen_foods)
        })
        .collect();

    let allergens_ingredients_map: HashMap<Allergen, HashSet<Ingredient>> =
        allergens_foods_map
            .iter()
            .map(|(&allergen, foods)| {
                let allergen_ingredients = foods
                    .iter()
                    .flat_map(|food| food.ingredients.iter().copied())
                    .filter(|ingredient| {
                        foods
                            .iter()
                            .all(|food| food.ingredients.contains(ingredient))
                    })
                    .collect();
                (allergen, allergen_ingredients)
            })
            .collect();

    let count = foods
        .iter()
        .flat_map(|food| food.ingredients.iter())
        .filter(|ingredient| {
            !allergens_ingredients_map
                .values()
                .any(|ingredient_set| ingredient_set.contains(ingredient))
        })
        .count();

    println!("Part 1: {}", count);
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
