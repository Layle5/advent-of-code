use std::fmt::Debug;
use crate::parser::parse_monkeys_and_worry_levels;
use itertools::Itertools;

mod parser;

fn main() {
    let input = include_str!("input.txt");
    let monkeys_and_worry_levels = parse_monkeys_and_worry_levels(input);
    let (monkeys, worry_levels, modulo_items) = create_modulo_items(monkeys_and_worry_levels);

    let easy_monkey_business_level = compute_monkey_business_level_from_worry_levels(
        &monkeys,
        worry_levels);

    println!("Part 1: {}", easy_monkey_business_level);

    let hard_monkey_business_level = compute_monkey_business_level_from_modulo_items(
        &monkeys,
        modulo_items);

    println!("Part 2: {}", hard_monkey_business_level);
}

macro_rules! item_log {
    ($($arg:tt)*) => {
        // eprintln!($($arg)*);
    };
}

macro_rules! round_log {
    ($($arg:tt)*) => {
        // eprintln!($($arg)*);
    };
}

fn compute_monkey_business_level_from_worry_levels(
    monkeys: &[Monkey],
    monkeys_worry_levels: Vec<Vec<u64>>,
) -> usize {
    compute_generic_monkey_business_level::<u64>(
        monkeys,
        monkeys_worry_levels,
        20,
        |_, operation, worry_level| *worry_level = apply_operation_to_worry_level(operation, *worry_level) / 3,
        |_, monkey, worry_level| worry_level % monkey.divisibility_number == 0,
    )
}

fn compute_monkey_business_level_from_modulo_items(
    monkeys: &[Monkey],
    monkeys_modulo_items: Vec<Vec<ModuloItem>>,
) -> usize {
    /* For part 2, one way to manage worry levels between rounds is to only care about moduli.
     * For each monkey and for each worry level, only consider the worry level modulo the monkey's test number.
     * Using multiple moduli for a single item allows:
     * - checking easily if the item passes a monkey's test (equals 0?) ;
     * - additions/multiplications without causing issues for subsequent monkey tests ;
     * - small numbers for all of 10,000 rounds.
     */
    compute_generic_monkey_business_level::<ModuloItem>(
        monkeys,
        monkeys_modulo_items,
        10_000,
        apply_operation_to_modulo_item,
        |monkey_index, _, modulo_item| modulo_item.moduli[monkey_index] == 0,
    )
}

fn compute_generic_monkey_business_level<T: Debug>(
    monkeys: &[Monkey],
    mut monkeys_items: Vec<Vec<T>>,
    number_rounds: usize,
    apply_operation: fn(&[Monkey], &Operation, &mut T),
    test_item: fn(usize, &Monkey, &T) -> bool,
) -> usize {
    let mut number_inspections = vec![0; monkeys.len()];

    for round in 1..=number_rounds {
        for current_monkey_index in 0..monkeys.len() {
            let current_monkey = &monkeys[current_monkey_index];
            item_log!("Monkey {}:", current_monkey.id);

            let items = monkeys_items[current_monkey_index].drain(..).collect::<Vec<_>>();
            for mut item in items {
                number_inspections[current_monkey_index] += 1;
                item_log!("  Monkey inspects an item {item:?}.");

                apply_operation(monkeys, &current_monkey.operation, &mut item);
                item_log!("    Monkey gets bored with item. Worry level is managed to {item:?}.");

                let test_succeeded = test_item(current_monkey_index, current_monkey, &item);
                let next_monkey_id =
                    if test_succeeded {
                        item_log!("    Current worry level is divisible by {}.", current_monkey.test_divisible);
                        current_monkey.true_monkey_id
                    } else {
                        item_log!("    Current worry level is not divisible by {}.", current_monkey.test_divisible);
                        current_monkey.false_monkey_id
                    };

                item_log!("    Item {item:?} is thrown to monkey {next_monkey_id}.");
                monkeys_items[next_monkey_id].push(item);
            }
        }

        show_round(round, &monkeys, &number_inspections);
    }

    best_inspections_product(number_inspections)
}

fn best_inspections_product(number_inspections: Vec<usize>) -> usize {
    let mut most_inspections = number_inspections.into_iter().sorted_by(|lhs, rhs| rhs.cmp(lhs));
    let first_most_inspections = most_inspections.next().expect("at least two inspection numbers");
    let second_most_inspections = most_inspections.next().expect("at least two inspection numbers");
    first_most_inspections * second_most_inspections
}

#[allow(unused_variables)]
fn show_round(round: usize, monkeys: &[Monkey], number_inspections: &[usize]) {
    if round == 1 || round == 20 || round % 1000 == 0 {
        round_log!("== After round {} ==", round);
        for monkey_index in 0..monkeys.len() {
            round_log!(
                "Monkey {} inspected items {} times.",
                monkey_index,
                number_inspections[monkey_index]);
        }
    }
}

fn apply_operation_to_worry_level(operation: &Operation, worry_level: u64) -> u64 {
    match operation {
        Operation { operator: Operator::Add, operand: Operand::Old } => {
            let new_worry_level = worry_level + worry_level;
            item_log!("    Worry level is added by itself to {new_worry_level}.");
            new_worry_level
        }
        Operation { operator: Operator::Add, operand: Operand::Number(number) } => {
            let new_worry_level = worry_level + number;
            item_log!("    Worry level is added by {number} to {new_worry_level}.");
            new_worry_level
        }
        Operation { operator: Operator::Multiply, operand: Operand::Old } => {
            let new_worry_level = worry_level * worry_level;
            item_log!("    Worry level is multiplied by itself to {new_worry_level}.");
            new_worry_level
        }
        Operation { operator: Operator::Multiply, operand: Operand::Number(number) } => {
            let new_worry_level = worry_level * number;
            item_log!("    Worry level is multiplied by {number} to {new_worry_level}.");
            new_worry_level
        }
    }
}

fn apply_operation_to_modulo_item(monkeys: &[Monkey], operation: &Operation, item: &mut ModuloItem) {
    match operation {
        Operation { operator: Operator::Add, operand: Operand::Old } => {
            item.moduli.iter_mut().enumerate().for_each(|(index, modulo)| *modulo = (*modulo + *modulo) % monkeys[index].divisibility_number);
            item_log!("    Worry level is added by itself to {item:?}.");
        }
        Operation { operator: Operator::Add, operand: Operand::Number(number) } => {
            item.moduli.iter_mut().enumerate().for_each(|(index, modulo)| *modulo = (*modulo + number) % monkeys[index].divisibility_number);
            item_log!("    Worry level is added by {number} to {item:?}.");
        }
        Operation { operator: Operator::Multiply, operand: Operand::Old } => {
            item.moduli.iter_mut().enumerate().for_each(|(index, modulo)| *modulo = (*modulo * *modulo) % monkeys[index].divisibility_number);
            item_log!("    Worry level is multiplied by itself to {item:?}.");
        }
        Operation { operator: Operator::Multiply, operand: Operand::Number(number) } => {
            item.moduli.iter_mut().enumerate().for_each(|(index, modulo)| *modulo = (*modulo * number) % monkeys[index].divisibility_number);
            item_log!("    Worry level is multiplied by {number} to {item:?}.");
        }
    }
}

fn create_modulo_items(monkeys_and_worry_levels: Vec<(Monkey, Vec<u64>)>) -> (Vec<Monkey>, Vec<Vec<u64>>, Vec<Vec<ModuloItem>>) {
    let tests_divisible_numbers = monkeys_and_worry_levels
        .iter()
        .map(|(monkey, _)| monkey.divisibility_number)
        .collect_vec();

    monkeys_and_worry_levels.into_iter().fold(
        (vec![], vec![], vec![]),
        |(mut monkeys, mut monkeys_worry_levels, mut monkeys_items), (monkey, worry_levels)| {
            monkeys.push(monkey);
            monkeys_worry_levels.push(worry_levels.clone());
            monkeys_items.push(
                worry_levels.into_iter().map(|worry_level| {
                    let moduli = tests_divisible_numbers
                        .iter()
                        .map(|&test_divisible_number| {
                            worry_level % test_divisible_number
                        })
                        .collect_vec();

                    ModuloItem { moduli }
                }).collect_vec()
            );
            (monkeys, monkeys_worry_levels, monkeys_items)
        },
    )
}

#[derive(Debug, Clone)]
struct ModuloItem {
    // 'moduli' is the plural of 'modulo'. Live and learn.
    moduli: Vec<u64>,
}

#[derive(Debug, Clone)]
struct Monkey {
    #[allow(dead_code)]
    id: usize,
    operation: Operation,
    divisibility_number: u64,
    true_monkey_id: usize,
    false_monkey_id: usize,
}

#[derive(Debug, Clone)]
struct Operation {
    operator: Operator,
    operand: Operand,
}

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug, Clone)]
enum Operand {
    Old,
    Number(u64),
}
