use itertools::Itertools;
use std::{
    collections::{BinaryHeap, VecDeque},
    fs,
    iter::once,
};
use std::{
    collections::{HashMap, HashSet},
    fmt,
};
use std::{env, fmt::Display};

type Position = (usize, usize);
type Portal = (Circle, String);

enum Tile {
    Empty,
    Wall,
    Portal(Portal),
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Circle {
    Outer,
    Inner,
}

impl Circle {
    fn other(&self) -> Circle {
        match self {
            Circle::Outer => Circle::Inner,
            Circle::Inner => Circle::Outer,
        }
    }
}

struct Donut {
    grid: Vec<Vec<Tile>>,
    portals: HashMap<Portal, Position>,
}

impl Donut {
    pub fn neighbors(
        &self,
        pos: Position,
    ) -> impl Iterator<Item = (Position, &Tile)> + '_ {
        once((-1, 0))
            .chain(once((1, 0)))
            .chain(once((0, -1)))
            .chain(once((0, 1)))
            .filter(move |(d_row, _)| *d_row >= 0 || pos.0 >= 1)
            .filter(move |(_, d_col)| *d_col >= 0 || pos.1 >= 1)
            .map(move |(d_row, d_col)| {
                (pos.0 as isize + d_row, pos.1 as isize + d_col)
            })
            .map(|(n_row, n_col)| (n_row as usize, n_col as usize))
            .filter_map(move |n| {
                self.grid
                    .get(n.0)
                    .and_then(|line| line.get(n.1))
                    .map(|tile| (n, tile))
            })
            .filter(|(_, tile)| !matches!(tile, Tile::Wall))
    }
}

impl Display for Donut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let write_line =
            |f: &mut fmt::Formatter<'_>, line: &[Tile]| -> fmt::Result {
                for tile in line {
                    let chr = match tile {
                        Tile::Empty => '.',
                        Tile::Wall => '#',
                        Tile::Portal(_) => 'P',
                    };
                    write!(f, "{}", chr)?;
                }
                Ok(())
            };

        write_line(f, self.grid.first().unwrap())?;
        for line in self.grid.iter().skip(1) {
            writeln!(f)?;
            write_line(f, line)?;
        }

        Ok(())
    }
}

fn parse(content: &str) -> Donut {
    let get_dimensions = |lines: &[Vec<u8>]| {
        lines
            .iter()
            .flat_map(|line| {
                line.split(|&byte| byte != b'#' && byte != b'.')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.len())
            })
            .minmax()
            .into_option()
            .unwrap()
    };

    let lines = content
        .lines()
        .map(|line| line.as_bytes().to_owned())
        .collect_vec();
    let (inner_width, total_width) = get_dimensions(&lines);

    let transposed = {
        let mut res = vec![];
        for col in 0..lines[0].len() {
            res.push(vec![]);
            for line in &lines {
                res.last_mut().unwrap().push(line[col]);
            }
        }
        res
    };
    let (inner_height, total_height) = get_dimensions(&transposed);

    let mut grid = lines
        .iter()
        .map(|line| {
            line.iter()
                .copied()
                .map(|byte| {
                    if byte == b'.' {
                        Tile::Empty
                    } else {
                        Tile::Wall
                    }
                })
                .collect_vec()
        })
        .collect_vec();

    let mut portals: HashMap<Portal, Position> = HashMap::new();
    let mut make_portal =
        |row: usize, col: usize, circle: Circle, portal: [u8; 2]| {
            let name: String =
                portal.iter().copied().map(|b| b as char).collect();
            let position = (row, col);
            portals
                .entry((circle, name.clone()))
                .and_modify(|_| panic!())
                .or_insert_with(|| position);
            grid[row][col] = Tile::Portal((circle, name));
        };

    let i = 0;
    for col in 0..lines[i].len() {
        let a = lines[i][col];
        let b = lines[i + 1][col];
        if a != b' ' && b != b' ' {
            make_portal(i + 2, col, Circle::Outer, [a, b]);
        }
    }
    let i = total_height + 2;
    for col in 0..lines[i].len() {
        let a = lines[i][col];
        let b = lines[i + 1][col];
        if a != b' ' && b != b' ' {
            make_portal(i - 1, col, Circle::Outer, [a, b]);
        }
    }
    let j = 0;
    for row in 0..lines.len() {
        let a = lines[row][j];
        let b = lines[row][j + 1];
        if a != b' ' && b != b' ' {
            make_portal(row, j + 2, Circle::Outer, [a, b]);
        }
    }
    let j = total_width + 2;
    for row in 0..lines.len() {
        let a = lines[row][j];
        let b = lines[row][j + 1];
        if a != b' ' && b != b' ' {
            make_portal(row, j - 1, Circle::Outer, [a, b]);
        }
    }
    let i = inner_height + 2;
    for col in (inner_width + 2)..(lines[i].len() - (inner_width + 2)) {
        let a = lines[i][col];
        let b = lines[i + 1][col];
        if a != b' ' && b != b' ' {
            make_portal(i - 1, col, Circle::Inner, [a, b]);
        }
    }
    let i = total_height - inner_height;
    for col in (inner_width + 2)..(lines[i].len() - (inner_width + 2)) {
        let a = lines[i][col];
        let b = lines[i + 1][col];
        if a != b' ' && b != b' ' {
            make_portal(i + 2, col, Circle::Inner, [a, b]);
        }
    }
    let j = inner_width + 2;
    for row in (inner_height + 2)..(lines.len() - (inner_height + 2)) {
        let a = lines[row][j];
        let b = lines[row][j + 1];
        if a != b' ' && b != b' ' {
            make_portal(row, j - 1, Circle::Inner, [a, b]);
        }
    }
    let j = total_width - inner_width;
    for row in (inner_height + 2)..(lines.len() - (inner_height + 2)) {
        let a = lines[row][j];
        let b = lines[row][j + 1];
        if a != b' ' && b != b' ' {
            make_portal(row, j + 2, Circle::Inner, [a, b]);
        }
    }

    Donut { grid, portals }
}

type Graph = HashMap<Portal, HashMap<Portal, usize>>;

fn make_graph(donut: &Donut) -> Graph {
    let make_distances = |start: &Position| {
        let mut distances: HashMap<Portal, usize> = HashMap::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back((*start, 0));
        visited.insert(*start);

        while let Some((current_pos, current_distance)) = queue.pop_front() {
            for (next_pos, next_tile) in donut.neighbors(current_pos) {
                let next_distance = current_distance + 1;
                if !visited.contains(&next_pos) {
                    queue.push_back((next_pos, next_distance));
                    visited.insert(next_pos);

                    if let Tile::Portal(next_portal) = next_tile {
                        distances.insert(next_portal.clone(), next_distance);
                    }
                }
            }
        }

        distances
    };

    donut
        .portals
        .iter()
        .map(|(portal, portal_position)| {
            (
                portal.clone(),
                make_distances(portal_position)
                    .into_iter()
                    .into_group_map()
                    .into_iter()
                    .map(|(portal, distances)| {
                        (portal, distances.into_iter().min().unwrap())
                    })
                    .collect(),
            )
        })
        .collect()
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    portal: Portal,
    steps: usize,
    level: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.steps
            .cmp(&other.steps)
            .reverse()
            .then_with(|| self.level.cmp(&other.level).reverse())
            .then_with(|| self.portal.cmp(&other.portal))
    }
}

fn get_next_state(
    current: &State,
    entrance: &Portal,
    cost: usize,
) -> Option<State> {
    if entrance.1 == "AA" {
        return None;
    }

    Some(State {
        portal: (entrance.0.other(), entrance.1.clone()),
        steps: current.steps + cost + 1,
        level: current.level,
    })
}

fn get_next_state_with_level(
    current: &State,
    entrance: &Portal,
    cost: usize,
) -> Option<State> {
    let next_level_op = match entrance.0 {
        Circle::Outer => {
            if entrance.1 == "AA" {
                None
            } else if (current.level > 1) ^ (entrance.1 == "ZZ") {
                Some(current.level - 1)
            } else {
                None
            }
        }
        Circle::Inner => Some(current.level + 1),
    };

    next_level_op.map(|next_level| {
        let next_portal_exit = (entrance.0.other(), entrance.1.clone());
        let next_steps = current.steps + cost + 1;

        State {
            portal: next_portal_exit,
            steps: next_steps,
            level: next_level,
        }
    })
}

fn find_path(
    graph: &Graph,
    get_next_state: fn(&State, &Portal, usize) -> Option<State>,
) -> Option<usize> {
    let mut heap = BinaryHeap::new();
    let mut steps: HashMap<(Portal, usize), usize> = HashMap::new();

    let start_portal = (Circle::Outer, "AA".to_string());
    heap.push(State {
        portal: start_portal.clone(),
        steps: 0,
        level: 1,
    });
    steps.insert((start_portal, 0), 0);

    while let Some(current) = heap.pop() {
        if current.portal.1 == "ZZ" {
            return Some(current.steps - 1);
        }

        for (next_portal_entrance, &next_cost) in &graph[&current.portal] {
            let next_state_op =
                get_next_state(&current, next_portal_entrance, next_cost);
            if let Some(next_state) = next_state_op {
                let next_stored_steps = steps
                    .entry((next_state.portal.clone(), next_state.level))
                    .or_insert(usize::MAX);

                if next_state.steps < *next_stored_steps {
                    *next_stored_steps = next_state.steps;
                    heap.push(next_state);
                }
            }
        }
    }

    None
}

fn solve_part_1(content: &str) {
    let donut = parse(content);
    let graph = make_graph(&donut);
    let steps = find_path(&graph, get_next_state).unwrap();
    println!("Part 1: {}", steps);
}

fn solve_part_2(content: &str) {
    let donut = parse(content);
    let graph = make_graph(&donut);
    let steps = find_path(&graph, get_next_state_with_level).unwrap();
    println!("Part 2: {}", steps);
}

fn main() {
    let args = env::args().collect_vec();
    let filename = args.get(1).map(|s| s.as_ref()).unwrap_or("./res/input.txt");

    let content = fs::read_to_string(filename).unwrap();

    solve_part_1(&content);
    solve_part_2(&content);
}
