use std::{num::ParseIntError, str::FromStr};

#[derive(Clone, Debug)]
struct Aquarium {
    tanks: [u64; 9],
    tank_index: usize,
}

impl FromStr for Aquarium {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let fish_results = input.trim().split(',').map(str::parse);

        let mut tanks = [0; 9];
        for fish_result in fish_results {
            let fish: usize = fish_result?;
            tanks[fish] += 1;
        }

        Ok(Aquarium {
            tanks,
            tank_index: 0,
        })
    }
}

impl Aquarium {
    fn age(&mut self) {
        let len = self.tanks.len();
        let add_mod = |i, n| (i + n) % len;
        let number_fishes = self.tanks[self.tank_index];
        self.tanks[add_mod(self.tank_index, 7)] += number_fishes;
        self.tank_index = add_mod(self.tank_index, 1);
    }
}

fn solve(mut aquarium: Aquarium, days: u64) -> u64 {
    for _ in 0..days {
        aquarium.age();
    }

    aquarium.tanks.iter().sum::<u64>()
}

fn main() {
    let input = include_str!("./input.txt");
    let aquarium = Aquarium::from_str(input).unwrap();
    println!("Part 1: {}", solve(aquarium.clone(), 80));
    println!("Part 2: {}", solve(aquarium, 256));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_and_solve(input: &str, days: u64) -> u64 {
        solve(Aquarium::from_str(input).unwrap(), days)
    }

    #[test]
    fn example_18_days() {
        let input = include_str!("./example.txt");
        assert_eq!(parse_and_solve(input, 18), 26);
    }

    #[test]
    fn example_80_days() {
        let input = include_str!("./example.txt");
        assert_eq!(parse_and_solve(input, 80), 5934);
    }

    #[test]
    fn example_256_days() {
        let input = include_str!("./example.txt");
        assert_eq!(parse_and_solve(input, 256), 26984457539);
    }
}
