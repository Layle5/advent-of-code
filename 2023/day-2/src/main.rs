fn main() {
    let input = include_str!("input.txt");
    let games = parse_games(input);
    let sum_ids = sum_ids_of_valid_games(&games);
    println!("Part 1: {sum_ids}");
    let sum_power_sets = sum_power_sets_of_games(&games);
    println!("Part 2: {sum_power_sets}");
}

fn parse_games(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let (game_str, reveals_str) = line.split_once(':').unwrap();
            let id = game_str.trim().split_once(' ').unwrap().1.parse().unwrap();
            let reveals = reveals_str.split(";").map(|pull| {
                pull.split(",").map(|balls_str| {
                    let (number_str, color_str) = balls_str.trim().split_once(' ').unwrap();
                    Pull { number: number_str.parse().unwrap(), color: color_str }
                }).collect()
            }).collect();

            Game { id, reveals }
        })
        .collect()
}

struct Game<'a> {
    id: usize,
    reveals: Vec<Vec<Pull<'a>>>,
}

struct Pull<'a> {
    number: usize,
    color: &'a str,
}

fn sum_ids_of_valid_games(games: &[Game]) -> usize {
    games.iter().filter(|game| {
        game.reveals.iter().all(|reveal| {
            reveal.iter().all(|pull| {
                pull.number <= match pull.color {
                    "red" => 12,
                    "green" => 13,
                    "blue" => 14,
                    color => panic!("unrecognized color {color}")
                }
            })
        })
    }).map(|game| game.id).sum::<usize>()
}

fn sum_power_sets_of_games(games: &[Game]) -> usize {
    games.iter().map(|game| {
        let mut minimums = [0usize; 3];

        game.reveals.iter().for_each(|reveal| {
            reveal.iter().for_each(|pull| {
                let minimum = match pull.color {
                    "red" => &mut minimums[0],
                    "green" => &mut minimums[1],
                    "blue" => &mut minimums[2],
                    color => panic!("unrecognized color {color}")
                };

                *minimum = pull.number.max(*minimum);
            })
        });

        minimums.iter().product::<usize>()
    }).sum::<usize>()
}