mod grid;

use grid::{Grid, Position, Tile};
use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::{BTreeSet, BinaryHeap, HashSet},
    fs,
};
use std::{
    collections::{HashMap, VecDeque},
    env,
    iter::once,
};

type KeyDistances = HashMap<char, usize>;
type Graph = HashMap<char, KeyDistances>;

fn compute_key_distances(grid: &Grid, start_pos: &Position) -> KeyDistances {
    let mut queue: VecDeque<_> = once((*start_pos, 0)).collect();
    let mut visited: HashSet<_> = once(*start_pos).collect();
    let mut distances = KeyDistances::new();

    while let Some((cur_pos, cur_distance)) = queue.pop_front() {
        for (n_pos, n_tile) in grid.neighbors(&cur_pos) {
            if !visited.contains(&n_pos) {
                visited.insert(n_pos);

                let n_distance = cur_distance + 1;

                match n_tile {
                    t if t.is_node() => {
                        let chr = n_tile.into();
                        distances.insert(chr, n_distance);
                    }
                    Tile::Empty => {
                        queue.push_back((n_pos, n_distance));
                    }
                    _ => {}
                }
            }
        }
    }

    distances
}

fn compute_graph(grid: &Grid) -> Graph {
    let mut graph = Graph::new();

    let start_distances = compute_key_distances(grid, &grid.start());
    graph.insert('@', start_distances);

    for (pos, tile) in grid.iter_tiles() {
        match tile {
            t if t.is_node() => {
                let chr = t.into();
                let distances = compute_key_distances(grid, &pos);
                graph.insert(chr, distances);
            }
            _ => {}
        }
    }
    graph
}

type KeySet = BTreeSet<char>;

#[derive(Debug, PartialEq, Eq)]
struct State {
    robots: Vec<char>,
    distance: usize,
    keys: KeySet,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .distance
            .cmp(&self.distance)
            .then(self.keys.len().cmp(&other.keys.len()))
    }
}

fn iter(grid: &Grid, graph: &Graph, starts: Vec<char>) -> Option<usize> {
    let mut heap = BinaryHeap::new();
    let mut distances: HashMap<(Vec<char>, KeySet), usize> = HashMap::new();
    let mut cache: HashMap<(char, KeySet), KeyDistances> = HashMap::new();

    heap.push(State {
        robots: starts.clone(),
        distance: 0,
        keys: KeySet::new(),
    });

    distances.insert((starts, KeySet::new()), 0);

    while let Some(cur) = heap.pop() {
        if cur.keys.len() >= grid.keys.len() {
            return Some(cur.distance);
        }

        if let Some(&best_distance) =
            distances.get(&(cur.robots.clone(), cur.keys.clone()))
        {
            if cur.distance > best_distance {
                continue;
            }
        }

        for (index, &robot) in cur.robots.iter().enumerate() {
            let next_nodes = cache
                .entry((robot, cur.keys.clone()))
                .or_insert_with(|| find_next_nodes(graph, robot, &cur.keys));

            for (&n_node, &n_delta) in next_nodes.iter() {
                let n_robots = {
                    let mut n_robots = cur.robots.clone();
                    n_robots[index] = n_node;
                    n_robots
                };
                let n_distance = cur.distance + n_delta;
                let n_keys = {
                    let mut n_keys = cur.keys.clone();
                    n_keys.insert(n_node);
                    n_keys
                };

                let distances_entry = distances
                    .entry((n_robots.clone(), n_keys.clone()))
                    .or_insert(usize::max_value());

                if n_distance < *distances_entry {
                    *distances_entry = n_distance;
                    heap.push(State {
                        robots: n_robots,
                        distance: n_distance,
                        keys: n_keys,
                    });
                }
            }
        }
    }

    None
}

#[derive(PartialEq, Eq)]
struct PartialState {
    node: char,
    distance: usize,
}

impl PartialOrd for PartialState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PartialState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

fn find_next_nodes(graph: &Graph, start: char, keys: &KeySet) -> KeyDistances {
    let mut heap: BinaryHeap<PartialState> = BinaryHeap::new();
    let mut distances = KeyDistances::new();
    let mut new_keys = KeySet::new();

    heap.push(PartialState {
        node: start,
        distance: 0,
    });

    while let Some(cur) = heap.pop() {
        if cur.node.is_lowercase() && !keys.contains(&cur.node) {
            new_keys.insert(cur.node);
            continue;
        }

        if distances
            .get(&cur.node)
            .map(|d| *d < cur.distance)
            .unwrap_or(false)
        {
            continue;
        }

        for (n_node, n_delta) in &graph[&cur.node] {
            if n_node.is_uppercase()
                && !keys.contains(&n_node.to_ascii_lowercase())
            {
                continue;
            }

            let n_distance = cur.distance + *n_delta;

            if distances
                .get(n_node)
                .map(|d| n_distance < *d)
                .unwrap_or(true)
            {
                distances.insert(*n_node, n_distance);
                heap.push(PartialState {
                    node: *n_node,
                    distance: n_distance,
                });
            }
        }
    }

    new_keys.into_iter().map(|k| (k, distances[&k])).collect()
}

fn solve_part_1(content: &str) {
    let grid = content.parse::<Grid>().unwrap();
    let graph = compute_graph(&grid);
    let distance = iter(&grid, &graph, vec!['@']).unwrap();
    println!("Part 1: {}", distance);
}

fn solve_part_2(content: &str) {
    let grid = content.parse::<Grid>().unwrap().split_four();
    let graph = compute_graph(&grid);
    let distance = iter(&grid, &graph, vec!['@', '$', '%', '&']).unwrap();
    println!("Part 2: {}", distance);
}

fn main() {
    let args = env::args().collect_vec();
    let filename = args.get(1).map(|s| s.as_ref()).unwrap_or("./res/input.txt");

    let content = fs::read_to_string(filename).unwrap();

    solve_part_1(&content);
    solve_part_2(&content);
}
