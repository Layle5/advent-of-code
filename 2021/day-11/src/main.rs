use std::{collections::HashSet, mem::swap, str::FromStr};

#[derive(Clone, Debug)]
struct Cavern {
    width: usize,
    height: usize,
    octopuses: Vec<u8>,
}

impl FromStr for Cavern {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().unwrap().trim().len();
        let octopuses: Vec<u8> = s
            .lines()
            .flat_map(|line| line.trim().as_bytes())
            .map(|byte| byte - b'0')
            .collect();
        let height = octopuses.len() / width;

        Ok(Self {
            width,
            height,
            octopuses,
        })
    }
}

fn adjacents(cavern: &Cavern, index: usize) -> impl Iterator<Item = usize> + '_ {
    let row_index = index / cavern.width;
    let col_index = index % cavern.width;
    [
        (row_index.checked_sub(1), col_index.checked_sub(1)),
        (row_index.checked_sub(1), Some(col_index)),
        (row_index.checked_sub(1), col_index.checked_add(1)),
        (Some(row_index), col_index.checked_sub(1)),
        (Some(row_index), col_index.checked_add(1)),
        (row_index.checked_add(1), col_index.checked_sub(1)),
        (row_index.checked_add(1), Some(col_index)),
        (row_index.checked_add(1), col_index.checked_add(1)),
    ]
    .into_iter()
    .flat_map(|index_options| index_options.0.zip(index_options.1))
    .filter(|(adjacent_row_index, adjacent_col_index)| {
        *adjacent_row_index < cavern.height && *adjacent_col_index < cavern.width
    })
    .map(|(adjacent_row_index, adjacent_col_index)| {
        adjacent_row_index * cavern.width + adjacent_col_index
    })
}

fn find_initial_flashing(cavern: &mut Cavern) -> HashSet<usize> {
    let mut flashing_indexes = HashSet::new();

    for (index, octopus) in cavern.octopuses.iter_mut().enumerate() {
        *octopus += 1;
        if *octopus > 9 {
            flashing_indexes.insert(index);
        }
    }

    flashing_indexes
}

fn try_flashing_adjacents(
    cavern: &mut Cavern,
    flashing_indexes: &mut HashSet<usize>,
    flashing_index: usize,
) {
    let adjacent_indexes: Vec<usize> = adjacents(cavern, flashing_index).collect();
    for adjacent_index in adjacent_indexes {
        if let Some(adjacent_octopus) = cavern.octopuses.get_mut(adjacent_index) {
            *adjacent_octopus += 1;
            if *adjacent_octopus == 10 {
                flashing_indexes.insert(adjacent_index);
            }
        }
    }
}

fn reset_flashes(cavern: &mut Cavern) {
    cavern.octopuses.iter_mut().for_each(|octopus| {
        if *octopus > 9 {
            *octopus = 0;
        }
    });
}

fn step(cavern: &mut Cavern) -> u64 {
    let mut step_flashes = 0;
    let mut current_flashing_indexes = find_initial_flashing(cavern);
    let mut next_flashing_indexes = HashSet::new();

    while !current_flashing_indexes.is_empty() {
        step_flashes += current_flashing_indexes.len() as u64;

        for flashing_index in current_flashing_indexes.drain() {
            try_flashing_adjacents(cavern, &mut next_flashing_indexes, flashing_index);
        }

        swap(&mut current_flashing_indexes, &mut next_flashing_indexes);
    }

    reset_flashes(cavern);

    step_flashes
}

fn solve_part_1(mut cavern: Cavern) -> u64 {
    (1..=100).fold(0, |total_flashes, _| {
        let step_flashes = step(&mut cavern);
        total_flashes + step_flashes
    })
}

fn solve_part_2(mut cavern: Cavern) -> usize {
    (1..)
        .find(|_| {
            let step_flashes = step(&mut cavern);
            (step_flashes as usize) == cavern.octopuses.len()
        })
        .unwrap()
}

fn main() {
    let input = include_str!("./input.txt");
    let cavern = Cavern::from_str(input).unwrap();
    println!("Part 1: {}", solve_part_1(cavern.clone()));
    println!("Part 2: {}", solve_part_2(cavern));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        let input = include_str!("./example.txt");
        let cavern = Cavern::from_str(input).unwrap();
        assert_eq!(solve_part_1(cavern), 1656);
    }

    #[test]
    fn example_part_2() {
        let input = include_str!("./example.txt");
        let cavern = Cavern::from_str(input).unwrap();
        assert_eq!(solve_part_2(cavern), 195);
    }
}
