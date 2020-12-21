use std::fs;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    env,
};

#[macro_use]
extern crate lazy_static;

use itertools::Itertools;
use regex::Regex;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Ingredient<'a>(&'a str);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

fn get_allergens_ingredient_map<'a>(
    foods: &'a [Food],
) -> BTreeMap<Allergen<'a>, Ingredient<'a>> {
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

    let mut allergens_ingredients_map: HashMap<Allergen, HashSet<Ingredient>> =
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

    let mut allergens_ingredient_map = BTreeMap::new();

    loop {
        let allergen_ingredients_pair_op = allergens_ingredients_map
            .iter()
            .find(|(_, ingredients)| ingredients.len() == 1);

        if allergen_ingredients_pair_op.is_none() {
            break;
        }

        let (&allergen, ingredients) = allergen_ingredients_pair_op.unwrap();
        let ingredient = ingredients.iter().copied().next().unwrap();

        allergens_ingredients_map.remove(&allergen);
        for (_, ingredients) in allergens_ingredients_map.iter_mut() {
            ingredients.remove(&ingredient);
        }

        allergens_ingredient_map.insert(allergen, ingredient);
    }

    allergens_ingredient_map
}

fn solve_part_1(content: &str) {
    let foods = parse_foods(content);

    let allergens_ingredient_map = get_allergens_ingredient_map(&foods);

    let count = foods
        .iter()
        .flat_map(|food| food.ingredients.iter())
        .filter(|ingredient| {
            !allergens_ingredient_map
                .values()
                .any(|other_ingredient| other_ingredient == *ingredient)
        })
        .count();

    println!("Part 1: {}", count);
}

fn solve_part_2(content: &str) {
    let foods = parse_foods(content);

    let allergens_ingredient_map = get_allergens_ingredient_map(&foods);

    let ingredients_string = allergens_ingredient_map
        .values()
        .map(|ingredient| ingredient.0)
        .join(",");

    println!("Part 2: {}", ingredients_string);
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
