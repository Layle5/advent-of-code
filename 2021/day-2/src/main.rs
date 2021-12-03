use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
enum Command {
    Forward(u64),
    Up(u64),
    Down(u64),
}

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (command, number_str) = s.split_once(' ').ok_or("could not split line")?;

        let number = number_str
            .parse::<u64>()
            .map_err(|_| "could not parse number")?;

        match command {
            "up" => Ok(Command::Up(number)),
            "down" => Ok(Command::Down(number)),
            "forward" => Ok(Command::Forward(number)),
            _ => Err("could not recognize command"),
        }
    }
}

#[derive(Debug, Default)]
struct Submarine {
    position: u64,
    depth: u64,
    aim: u64,
}

impl Submarine {
    fn follow_basic(self, command: &Command) -> Submarine {
        match command {
            Command::Up(number) => Submarine {
                depth: self.depth - number,
                ..self
            },
            Command::Down(number) => Submarine {
                depth: self.depth + number,
                ..self
            },
            Command::Forward(number) => Submarine {
                position: self.position + number,
                ..self
            },
        }
    }

    fn follow_complex(self, command: &Command) -> Submarine {
        match command {
            Command::Up(number) => Submarine {
                aim: self.aim - number,
                ..self
            },
            Command::Down(number) => Submarine {
                aim: self.aim + number,
                ..self
            },
            Command::Forward(number) => Submarine {
                position: self.position + number,
                depth: self.depth + self.aim * number,
                ..self
            },
        }
    }
}

fn parse(input: &str) -> Result<Vec<Command>, &'static str> {
    input
        .lines()
        .map(Command::from_str)
        .collect::<Result<Vec<_>, _>>()
}

fn solve<F>(commands: &[Command], follow_command: F) -> u64
where
    F: FnMut(Submarine, &Command) -> Submarine,
{
    let submarine = commands.iter().fold(Submarine::default(), follow_command);
    submarine.position * submarine.depth
}

fn main() -> Result<(), &'static str> {
    let input = include_str!("./input.txt");
    let commands = parse(input)?;
    println!("Part 1: {}", solve(&commands, Submarine::follow_basic));
    println!("Part 2: {}", solve(&commands, Submarine::follow_complex));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() -> Result<(), &'static str> {
        let input = include_str!("./example.txt");
        assert_eq!(solve(&parse(input)?, Submarine::follow_basic), 150);
        Ok(())
    }

    #[test]
    fn example_part_2() -> Result<(), &'static str> {
        let input = include_str!("./example.txt");
        assert_eq!(solve(&parse(input)?, Submarine::follow_complex), 900);
        Ok(())
    }
}
