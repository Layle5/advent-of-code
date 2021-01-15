use std::{
    collections::{HashMap, HashSet, VecDeque},
    env,
    fmt::Debug,
    str::FromStr,
};
use std::{fs, num::ParseIntError};

use itertools::Itertools;

type Graph<'a, 'b> = HashMap<&'a str, Vec<&'b str>>;

fn fold_graph<T, F>(graph: &Graph, node: &str, f: F) -> T
where
    T: Debug,
    F: Fn(Vec<T>, &str) -> T + Copy,
{
    let results = graph
        .get(node)
        .map(|v| {
            v.iter()
                .map(|child| fold_graph(graph, child, f))
                .collect_vec()
        })
        .unwrap_or_default();

    f(results, node)
}

fn solve_part_1(content: &str) {
    let graph = content
        .lines()
        .map(|line| line.split(')').collect_tuple().unwrap())
        .into_group_map();

    type Pair = (usize, usize);

    let (orbit_count, _) =
        fold_graph(&graph, "COM", |results: Vec<Pair>, _| {
            results
                .into_iter()
                .fold((0, 1), |a, r| (a.0 + r.0 + r.1, a.1 + r.1))
        });

    println!("Part 1: {}", orbit_count)
}

fn solve_part_2(content: &str) {
    let graph = content
        .lines()
        .map(|line| line.split(')').collect_tuple().unwrap())
        .into_group_map();

    type Pair = (Option<usize>, Option<usize>);

    let transfer_counts =
        fold_graph(&graph, "COM", |results: Vec<Pair>, node: &str| match node {
            "YOU" => (Some(0), None),
            "SAN" => (None, Some(0)),
            _ => {
                let result = results
                    .into_iter()
                    .fold((None, None), |a, r| (a.0.or(r.0), a.1.or(r.1)));

                if result.0.is_some() && result.1.is_some() {
                    result
                } else {
                    (result.0.map(|c| c + 1), result.1.map(|c| c + 1))
                }
            }
        });

    let transfer_count =
        transfer_counts.0.unwrap() + transfer_counts.1.unwrap();

    println!("Part 2: {}", transfer_count)
}

fn main() {
    let args = env::args().collect_vec();
    let filename = args.get(1).map(|s| s.as_ref()).unwrap_or("./res/input.txt");

    let content = fs::read_to_string(filename).unwrap();

    solve_part_1(&content);
    solve_part_2(&content);
}

#[cfg(test)]
mod tests {}
