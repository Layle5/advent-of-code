use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let races = parse_races(input);
    
    let result = races
        .iter()
        .map(count_possible_ways_of_winning)
        .product::<u64>();
    println!("Part 1: {}", result);
    
    let combined_race = combine_races(races);
    let combined_result = count_possible_ways_of_winning(&combined_race);
    println!("Part 2: {}", combined_result);
}

fn count_possible_ways_of_winning(race: &Race) -> u64 {
    let t = race.time as f64;
    let d = race.distance as f64;
    let distance_hold = (t - (t * t - 4.0 * d).sqrt()) / 2.0;
    let minimum_hold = distance_hold.floor() as u64 + 1;
    let result = (race.time + 1) - 2 * minimum_hold;
    result
}

fn combine_races(races: Vec<Race>) -> Race {
    races
        .into_iter()
        .fold(Race::default(), |combined_race, current_race| {
            let combine = |lhs: u64, rhs: u64| {
                lhs * 10u64.pow(rhs.ilog10() + 1) + rhs
            };
            Race {
                time: combine(combined_race.time, current_race.time),
                distance: combine(combined_race.distance, current_race.distance),
            }
        })
}

fn parse_races(input: &str) -> Vec<Race> {
    let parse_numbers = |line: &str| {
        line.trim()
            .split(' ')
            .filter(|part| !part.is_empty())
            .skip(1)
            .map(|part| part.parse::<u64>().expect("valid number"))
            .collect_vec()
    };
    let mut lines = input.lines();
    let times = parse_numbers(lines.next().expect("time line"));
    let distances = parse_numbers(lines.next().expect("distance line"));
    times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, distance)| Race { time, distance })
        .collect_vec()
}

#[derive(Debug, Default)]
struct Race {
    time: u64,
    distance: u64,
}
