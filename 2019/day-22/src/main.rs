use itertools::{Itertools};
use std::{
    collections::{HashMap},
    env,
    num::ParseIntError,
    str::FromStr,
};
use std::{fs};
use Technique::{Cut, Increment, New};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Technique {
    New,
    Cut(isize),
    Increment(usize),
}

type Card = usize;

enum View {
    Cards(usize),
    Reverse { inner: Box<View> },
    Rotate { inner: Box<View>, n: isize },
    Zoom { inner: Box<View>, n: usize },
}

impl FromStr for Technique {
    type Err = Option<ParseIntError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "deal into new stack" {
            Ok(New)
        } else if let Some(n) = s.strip_prefix("cut ") {
            Ok(Cut(n.parse().map_err(Some)?))
        } else if let Some(n) = s.strip_prefix("deal with increment ") {
            Ok(Increment(n.parse().map_err(Some)?))
        } else {
            Err(None)
        }
    }
}

impl View {
    fn len(&self) -> usize {
        match self {
            View::Cards(len) => *len,
            View::Reverse { inner, .. }
            | View::Rotate { inner, .. }
            | View::Zoom { inner, .. } => inner.len(),
        }
    }

    fn get(&self, index: usize) -> Option<Card> {
        let len = self.len();
        match self {
            View::Cards(len) => {
                if index < *len {
                    Some(index)
                } else {
                    None
                }
            }
            View::Reverse { inner } => {
                let inner_index = len - index - 1;
                inner.get(inner_index)
            }
            View::Rotate { inner, n } => {
                let inner_index = if *n < 0 {
                    (index + (len - n.abs() as usize % len)) % len
                } else {
                    (index + *n as usize) % len
                };
                inner.get(inner_index)
            }
            View::Zoom { inner, n } => {
                let x = inverse_modulo(*n, len).unwrap();
                let inner_index = safe_mul(index, x, len);
                inner.get(inner_index)
            }
        }
    }

    fn iter(&self) -> impl Iterator<Item = Card> + '_ {
        (0..self.len())
            .map(move |index| self.get(index))
            .map(|card| card.unwrap())
    }
}

fn safe_mul(a: usize, b: usize, m: usize) -> usize {
    if let Some(r) = a.checked_mul(b) {
        r % m
    } else if b % 2 == 0 {
        safe_mul(safe_mul(a, b / 2, m), 2, m)
    } else {
        (safe_mul(safe_mul(a, b / 2, m), 2, m) + a) % m
    }
}

fn reorganize(techniques: Vec<Technique>, size: usize) -> Vec<Technique> {
    let mut reorganized_techniques = vec![];

    let mut index = 0;
    while index < techniques.len() - 1 {
        let l = techniques[index];
        let r = techniques[index + 1];

        match (l, r) {
            (New, Cut(n)) => {
                reorganized_techniques.push(Cut(-n));
                reorganized_techniques.push(New);
                index += 2;
            }
            (New, Increment(n)) => {
                reorganized_techniques.push(Increment(n));
                reorganized_techniques
                    .push(Cut(size as isize + 1 - n as isize));
                reorganized_techniques.push(New);
                index += 2;
            }
            (Cut(c), Increment(i)) => {
                let c = if c < 0 {
                    size - c.abs() as usize
                } else {
                    c.abs() as usize
                };
                reorganized_techniques.push(Increment(i));
                reorganized_techniques.push(Cut(safe_mul(i, c, size) as isize));
                index += 2;
            }
            (l, _) => {
                reorganized_techniques.push(l);
                index += 1;
            }
        }
    }

    for t in techniques.into_iter().skip(index) {
        reorganized_techniques.push(t);
    }

    reorganized_techniques
}

fn compress_one(techniques: Vec<Technique>, size: usize) -> Vec<Technique> {
    techniques
        .into_iter()
        .map(Some)
        .coalesce(|lhs_op, rhs_op| match (lhs_op, rhs_op) {
            (None, None) => Ok(None),
            (None, _) => Ok(rhs_op),
            (_, None) => Ok(lhs_op),
            (Some(lhs), Some(rhs)) => match (lhs, rhs) {
                (New, New) => Ok(None),
                (Cut(ln), Cut(rn)) => Ok(Some(Cut((ln + rn) % size as isize))),
                (Increment(ln), Increment(rn)) => {
                    Ok(Some(Increment(safe_mul(ln, rn, size))))
                }
                (_, _) => Err((Some(lhs), Some(rhs))),
            },
        })
        .filter_map(|o| o)
        .collect_vec()
}

fn compress_max(techniques: Vec<Technique>, size: usize) -> Vec<Technique> {
    let mut current_techniques = compress_one(techniques, size);

    loop {
        let current_len = current_techniques.len();
        current_techniques =
            compress_one(reorganize(current_techniques, size), size);
        if current_techniques.len() == current_len {
            break;
        }
    }

    current_techniques
}

fn compress_repeat(
    techniques: &[Technique],
    size: usize,
    cycles: usize,
    cache: &mut HashMap<usize, Vec<Technique>>,
) -> Vec<Technique> {
    if let Some(r) = cache.get(&cycles) {
        return r.clone();
    }

    let repeated = if cycles == 0 {
        vec![]
    } else if cycles == 1 {
        techniques.iter().copied().collect_vec()
    } else {
        let r_cycles = cycles / 2;
        let l_cycles = cycles - r_cycles;
        let left = compress_repeat(&techniques, size, l_cycles, cache);
        let right = compress_repeat(&techniques, size, r_cycles, cache);
        let next = left.into_iter().chain(right.into_iter()).collect_vec();
        compress_max(next, size)
    };

    cache.insert(cycles, repeated.clone());

    repeated
}

fn gcd(a: usize, b: usize) -> (isize, isize, usize) {
    if a == 0 {
        (0, 1, b)
    } else {
        let (rx, ry, g) = gcd(b % a, a);
        (ry - (b / a) as isize * rx, rx, g)
    }
}

fn inverse_modulo(n: usize, m: usize) -> Option<usize> {
    let (x, _, g) = gcd(n, m);
    if g != 1 {
        None
    } else {
        Some(((x % m as isize + m as isize) % m as isize) as usize)
    }
}

fn techniques_to_view(techniques: Vec<Technique>, size: usize) -> View {
    techniques.into_iter().fold(View::Cards(size), |v, t| {
        let inner = Box::new(v);
        match t {
            New => View::Reverse { inner },
            Cut(n) => View::Rotate { inner, n },
            Increment(n) => View::Zoom { inner, n },
        }
    })
}

fn solve_part_1(content: &str) {
    let techniques: Vec<Technique> = content
        .lines()
        .map(|line| line.parse().unwrap())
        .collect_vec();

    let size = 10007;
    let techniques = compress_max(techniques, size);
    let view = techniques_to_view(techniques, size);

    let position = view.iter().position(|card| card == 2019).unwrap();
    println!("Part 1: {}", position);
}

fn solve_part_2(content: &str) {
    let techniques: Vec<Technique> = content
        .lines()
        .map(|line| line.parse().unwrap())
        .collect_vec();

    let size = 119315717514047;
    let cycles = 101741582076661;
    let compressed_techniques = compress_repeat(
        &compress_max(techniques, size),
        size,
        cycles,
        &mut HashMap::new(),
    );

    let view = techniques_to_view(compressed_techniques, size);

    println!("Part 2: {}", view.get(2020).unwrap());
}

fn main() {
    let args = env::args().collect_vec();
    let filename = args.get(1).map(|s| s.as_ref()).unwrap_or("./res/input.txt");

    let content = fs::read_to_string(filename).unwrap();

    solve_part_1(&content);
    solve_part_2(&content);
}
