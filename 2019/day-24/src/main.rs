use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::{
    collections::HashSet,
    env,
    hash::Hash,
    iter::{once, successors},
    str::FromStr,
};
use std::{fmt::Display, fs};

#[derive(Clone, Debug)]
struct Eris {
    grids: Vec<u32>,
    size: usize,
}

impl FromStr for Eris {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect_vec();

        let size = lines.len();

        let bugs = lines
            .into_iter()
            .flat_map(|line| line.as_bytes())
            .copied()
            .map(|byte| byte == b'#');

        let grid = (0..)
            .map(|shift| 1 << shift)
            .zip(bugs)
            .map(|(m, b)| if b { m } else { 0 })
            .fold(0, |r, m| r | m);

        let grids = vec![grid];

        Ok(Eris { grids, size })
    }
}

impl Display for Eris {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, grid) in self.grids.iter().enumerate() {
            let s = (0..self.size * self.size)
                .map(|shift| 1 << shift)
                .map(|mask| grid & mask)
                .map(|rating| if rating > 0 { '#' } else { '.' })
                .chunks(self.size)
                .into_iter()
                .map(|line| line.collect::<String>())
                .join("\n");

            writeln!(f, "Depth {}:", index)?;
            for chr in s.chars() {
                write!(f, "{}", chr)?;
            }
            if index != self.grids.len() - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl PartialEq for Eris {
    fn eq(&self, other: &Self) -> bool {
        self.grids.eq(&other.grids)
    }
}

impl Eq for Eris {}

impl Hash for Eris {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.grids.hash(state)
    }
}

impl Eris {
    fn grid<I>(i: I) -> u32
    where
        I: IntoIterator<Item = bool>,
    {
        (0..25)
            .map(|shift| 1 << shift)
            .zip(i.into_iter())
            .map(|(mask, infested)| if infested { mask } else { 0 })
            .fold(0, |g, m| g | m)
    }

    fn from<I>(size: usize, i: I) -> Eris
    where
        I: IntoIterator<Item = bool>,
    {
        let grid = Eris::grid(i);
        let grids = vec![grid];
        Eris { size, grids }
    }

    fn rating(&self) -> u32 {
        self.grids[0]
    }

    fn len(&self) -> usize {
        self.size * self.size
    }

    fn grid_get(grid: u32, index: usize) -> Option<bool> {
        (0..)
            .take(25)
            .map(|shift| 1 << shift)
            .map(|mask| grid & mask)
            .map(|rating| rating > 0)
            .nth(index)
    }

    fn get(&self, grid_index: usize, index: usize) -> Option<bool> {
        let grid = self.grids[grid_index];
        Eris::grid_get(grid, index)
    }

    fn neighbors(&self, index: usize) -> impl Iterator<Item = usize> + '_ {
        once(if index % self.size == 0 {
            None
        } else {
            index.checked_sub(1)
        })
        .chain(once(if index % self.size == self.size - 1 {
            None
        } else {
            index.checked_add(1)
        }))
        .chain(once(index.checked_sub(self.size)))
        .chain(once(index.checked_add(self.size)))
        .filter_map(|o| o)
        .filter(move |neighbor| *neighbor < self.len())
    }

    fn recursive_neighbors(
        &self,
        index: usize,
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        let mut neighbors = vec![];

        if index < self.size {
            neighbors.push((0, 7));
        } else if 20 <= index && index < 25 {
            neighbors.push((0, 17));
        }
        if index % 5 == 0 {
            neighbors.push((0, 11));
        } else if index % 5 == 4 {
            neighbors.push((0, 13));
        }

        neighbors.extend(self.neighbors(index).map(|n| (1, n)));

        if index == 7 {
            neighbors.extend((0..5).map(|i| (2, i)));
        } else if index == 11 {
            neighbors.extend((0..5).map(|i| (2, i * 5)));
        } else if index == 13 {
            neighbors.extend((0..5).map(|i| (2, i * 5 + 4)));
        } else if index == 17 {
            neighbors.extend((0..5).map(|i| (2, i + 20)));
        }

        neighbors
            .into_iter()
            .filter(|t| *t != (1, 12))
    }

    fn step(self) -> Eris {
        let iter = (0..self.len()).map(|current| {
            let count = self
                .neighbors(current)
                .filter_map(|neighbor| self.get(0, neighbor))
                .filter(|b| *b)
                .count();

            match self.get(0, current).unwrap() {
                true => count == 1,
                false => 1 <= count && count <= 2,
            }
        });

        Eris::from(self.size, iter)
    }

    fn recursive_step(self) -> Eris {
        let step = |outer: u32, middle: u32, inner: u32| {
            let iter = (0..self.len()).map(|current| {
                if current == 12 {
                    return false;
                }

                let count = self
                    .recursive_neighbors(current)
                    .filter_map(|(grid_index, neighbor)| {
                        let grid = [outer, middle, inner][grid_index];
                        Eris::grid_get(grid, neighbor)
                    })
                    .filter(|b| *b)
                    .count();

                match Eris::grid_get(middle, current).unwrap() {
                    true => count == 1,
                    false => 1 <= count && count <= 2,
                }
            });

            Eris::grid(iter)
        };

        let mut grids = vec![];

        grids.push(step(0, 0, *self.grids.first().unwrap()));
        for index in 0..self.grids.len() {
            let get = |o: Option<usize>| {
                o.and_then(|i| self.grids.get(i)).copied().unwrap_or(0)
            };

            let outer = get(index.checked_sub(1));
            let inner = get(index.checked_add(1));

            grids.push(step(outer, self.grids[index], inner));
        }
        grids.push(step(*self.grids.last().unwrap(), 0, 0));

        Eris {
            grids,
            size: self.size,
        }
    }
}

fn solve_part_1(content: &str) {
    let eris: Eris = content.parse().unwrap();
    let set: HashSet<Eris> = once(eris.clone()).collect();

    let (eris, _) = (0..)
        .fold_while((eris, set), |(e, mut s), _| {
            let n = e.step();
            if s.contains(&n) {
                Done((n, s))
            } else {
                s.insert(n.clone());
                Continue((n, s))
            }
        })
        .into_inner();

    println!("Part 1: {}", eris.rating());
}

fn solve_part_2(content: &str) {
    let eris: Eris = content.parse().unwrap();

    let eris = (0..200).fold(eris, |e, _| {
        e.recursive_step()
    });

    let eris_ref = &eris;

    let count = (0..eris.grids.len())
        .flat_map(|grid_index| {
            (0..25).map(move |index| eris_ref.get(grid_index, index))
        })
        .filter_map(|o| o)
        .filter(|b| *b)
        .count();

    println!("Part 2: {}", count);
}

fn main() {
    let args = env::args().collect_vec();
    let filename = args.get(1).map(|s| s.as_ref()).unwrap_or("./res/input.txt");

    let content = fs::read_to_string(filename).unwrap();

    solve_part_1(&content);
    solve_part_2(&content);
}
