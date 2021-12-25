use std::collections::HashMap;

#[derive(Default)]
struct DeterministicDie {
    current: u64,
}

impl DeterministicDie {
    fn roll(&mut self) -> u64 {
        self.current += 1;
        self.current
    }
}

#[derive(Default)]
struct DiracDie;

impl DiracDie {
    fn roll(&mut self) -> [u64; 3] {
        [1, 2, 3]
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Player {
    position: u64,
    score: u64,
}

impl Player {
    fn new(position: u64) -> Player {
        Self { position, score: 0 }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Situation {
    players: [Player; 2],
}

impl Situation {
    fn new(positions: (u64, u64)) -> Self {
        let players = [Player::new(positions.0), Player::new(positions.1)];
        Self { players }
    }
}

fn get_frequency_per_roll(die: &mut DiracDie) -> HashMap<u64, u64> {
    let rolls_for_one_die = die.roll();
    let mut frequency_per_roll: HashMap<u64, u64> = HashMap::default();

    for a in rolls_for_one_die {
        for b in rolls_for_one_die {
            for c in rolls_for_one_die {
                *frequency_per_roll.entry(a + b + c).or_default() += 1;
            }
        }
    }

    frequency_per_roll
}

#[derive(Clone, Debug, Default)]
struct Results {
    wins: [u64; 2],
}

fn play_part_1(die: &mut DeterministicDie, situation: Situation) -> Situation {
    let mut results = Results::default();
    let roll: u64 = [die.roll(), die.roll(), die.roll()].into_iter().sum();
    let player_1 = situation.players[0].clone();
    let player_2 = situation.players[1].clone();
    let next_position = (player_1.position + roll - 1) % 10 + 1;
    let next_score = player_1.score + next_position;

    let next_player_1 = Player {
        position: next_position,
        score: next_score,
    };

    let next_situation = Situation {
        players: [player_2, next_player_1],
    };

    if next_score >= 1000 {
        results.wins[0] += 1;
        return next_situation;
    }

    play_part_1(die, next_situation)
}

type Cache = HashMap<Situation, Results>;

fn play_part_2(die: &mut DiracDie, situation: Situation, cache: &mut Cache) -> Results {
    if let Some(cached_results) = cache.get(&situation) {
        return cached_results.clone();
    }

    let frequency_per_roll = get_frequency_per_roll(die);
    let mut results = Results::default();
    for (roll, frequency) in frequency_per_roll {
        let player_1 = situation.players[0].clone();
        let player_2 = situation.players[1].clone();
        let next_position = (player_1.position + roll - 1) % 10 + 1;
        let next_score = player_1.score + next_position;

        if next_score >= 21 {
            results.wins[0] += frequency;
            continue;
        }

        let next_player_1 = Player {
            position: next_position,
            score: next_score,
        };

        let next_situation = Situation {
            players: [player_2, next_player_1],
        };

        let next_results = play_part_2(die, next_situation, cache);
        results.wins[0] += next_results.wins[1] * frequency;
        results.wins[1] += next_results.wins[0] * frequency;
    }

    cache.insert(situation, results.clone());
    results
}

fn solve_part_1(positions: (u64, u64)) -> u64 {
    let mut die = DeterministicDie::default();
    let winning_situation = play_part_1(&mut die, Situation::new(positions));
    let losing_score = winning_situation
        .players
        .into_iter()
        .map(|p| p.score)
        .min()
        .unwrap();
    losing_score * die.current
}

fn solve_part_2(positions: (u64, u64)) -> u64 {
    let mut die = DiracDie::default();
    let mut cache = Cache::default();
    let results = play_part_2(&mut die, Situation::new(positions), &mut cache);
    results.wins.into_iter().max().unwrap()
}

fn parse(input: &str) -> (u64, u64) {
    let mut positions = input.trim().lines().map(|line| {
        let semicolon_index = line.find(':').unwrap();
        let position_index = semicolon_index + 2;
        let position_str = line[position_index..].trim();
        position_str.parse().unwrap()
    });

    (positions.next().unwrap(), positions.next().unwrap())
}

fn main() {
    let input = include_str!("./input.txt");
    let positions = parse(input);
    println!("Part 1: {}", solve_part_1(positions));
    println!("Part 2: {}", solve_part_2(positions));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        let input = include_str!("./example.txt");
        let positions = parse(input);
        assert_eq!(solve_part_1(positions), 739785);
    }

    #[test]
    fn example_part_2() {
        let input = include_str!("./example.txt");
        let positions = parse(input);
        assert_eq!(solve_part_2(positions), 444356092776315);
    }
}
