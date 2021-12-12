use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    str::FromStr,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Cave {
    name: [u8; 5],
}

impl FromStr for Cave {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut name = [0; 5];
        name.iter_mut()
            .zip(s.trim().as_bytes().iter().copied())
            .for_each(|(name_byte, s_byte)| {
                *name_byte = s_byte;
            });

        Ok(Self { name })
    }
}

impl Cave {
    fn is_big(&self) -> bool {
        self.name.first().unwrap().is_ascii_uppercase()
    }

    fn is_small(&self) -> bool {
        !self.is_big()
    }

    fn is_start(&self) -> bool {
        self.name.iter().eq("start".as_bytes().iter())
    }

    fn is_end(&self) -> bool {
        self.name.iter().eq("end\0\0".as_bytes().iter())
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in self.name.iter().copied().take_while(|&byte| byte != 0) {
            write!(f, "{}", byte as char)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
struct CaveSystem {
    connections_per_cave: HashMap<Cave, HashSet<Cave>>,
}

impl FromStr for CaveSystem {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let connection_results = s.trim().lines().map(|line| {
            let (lhs_cave_str, rhs_cave_str) =
                line.split_once('-').ok_or("could not split line")?;
            let lhs_cave: Cave = lhs_cave_str.parse()?;
            let rhs_cave: Cave = rhs_cave_str.parse()?;
            Ok((lhs_cave, rhs_cave))
        });

        let mut connections_per_cave: HashMap<Cave, HashSet<Cave>> = HashMap::new();

        for connection_result in connection_results {
            let (lhs_cave, rhs_cave) = connection_result?;
            connections_per_cave
                .entry(lhs_cave)
                .or_default()
                .insert(rhs_cave);
            connections_per_cave
                .entry(rhs_cave)
                .or_default()
                .insert(lhs_cave);
        }

        Ok(Self {
            connections_per_cave,
        })
    }
}

fn solve_rec(
    cave_system: &CaveSystem,
    current_cave: Cave,
    visited_caves: &mut HashSet<Cave>,
    allow_second_visits: bool,
    cave_visited_twice: Option<Cave>,
) -> u64 {
    if current_cave.is_end() {
        return 1;
    }

    let is_small = current_cave.is_small();
    let visited = visited_caves.contains(&current_cave);

    if current_cave.is_start() && visited {
        return 0;
    }
    if is_small && visited && !allow_second_visits {
        return 0;
    }
    if is_small && visited && allow_second_visits && cave_visited_twice.is_some() {
        return 0;
    }

    if !visited {
        visited_caves.insert(current_cave);
    }

    let connections = cave_system.connections_per_cave.get(&current_cave).unwrap();
    let mut total = 0;
    let connection_cave_visited_twice = if visited && is_small && allow_second_visits {
        Some(current_cave)
    } else {
        cave_visited_twice
    };

    for &connection in connections {
        total += solve_rec(
            cave_system,
            connection,
            visited_caves,
            allow_second_visits,
            connection_cave_visited_twice,
        );
    }

    if !visited {
        visited_caves.remove(&current_cave);
    }

    total
}

fn solve_part_1(cave_system: &CaveSystem) -> u64 {
    solve_rec(
        cave_system,
        Cave::from_str("start").unwrap(),
        &mut HashSet::new(),
        false,
        None,
    )
}

fn solve_part_2(cave_system: &CaveSystem) -> u64 {
    solve_rec(
        cave_system,
        Cave::from_str("start").unwrap(),
        &mut HashSet::new(),
        true,
        None,
    )
}

fn main() {
    let input = include_str!("./input.txt");
    let cave_system: CaveSystem = input.parse().unwrap();
    println!("Part 1: {}", solve_part_1(&cave_system));
    println!("Part 2: {}", solve_part_2(&cave_system));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_example_part_1() {
        let input = include_str!("./small-example.txt");
        let cave_system: CaveSystem = input.parse().unwrap();
        assert_eq!(solve_part_1(&cave_system), 10);
    }

    #[test]
    fn slightly_larger_example_part_1() {
        let input = include_str!("./slightly-larger-example.txt");
        let cave_system: CaveSystem = input.parse().unwrap();
        assert_eq!(solve_part_1(&cave_system), 19);
    }

    #[test]
    fn even_larger_example_part_1() {
        let input = include_str!("./even-larger-example.txt");
        let cave_system: CaveSystem = input.parse().unwrap();
        assert_eq!(solve_part_1(&cave_system), 226);
    }

    #[test]
    fn small_example_part_2() {
        let input = include_str!("./small-example.txt");
        let cave_system: CaveSystem = input.parse().unwrap();
        assert_eq!(solve_part_2(&cave_system), 36);
    }

    #[test]
    fn slightly_larger_example_part_2() {
        let input = include_str!("./slightly-larger-example.txt");
        let cave_system: CaveSystem = input.parse().unwrap();
        assert_eq!(solve_part_2(&cave_system), 103);
    }

    #[test]
    fn even_larger_example_part_2() {
        let input = include_str!("./even-larger-example.txt");
        let cave_system: CaveSystem = input.parse().unwrap();
        assert_eq!(solve_part_2(&cave_system), 3509);
    }
}
