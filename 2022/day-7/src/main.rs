use std::collections::HashMap;

#[derive(Debug)]
enum Entry<'a> {
    Dir(&'a str),
    File(&'a str, u64),
}

impl<'a> TryFrom<&'a str> for Entry<'a> {
    type Error = &'static str;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let (dir_or_size_str, name) = s.trim().split_once(' ').ok_or("could not split entry")?;
        match dir_or_size_str {
            "dir" => Ok(Entry::Dir(name)),
            size_str => Ok(Entry::File(
                name,
                size_str.parse().map_err(|_| "could not parse size")?,
            )),
        }
    }
}

#[derive(Debug)]
enum Command<'a> {
    Cd(&'a str),
    Ls(Vec<Entry<'a>>),
}

impl<'a> TryFrom<&'a str> for Command<'a> {
    type Error = &'static str;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut lines = s.lines();
        let command_line = lines.next().ok_or("parsing command without lines")?;
        let splitted_command_line = command_line
            .strip_prefix("$")
            .unwrap_or(command_line)
            .trim()
            .split_once(' ');
        match splitted_command_line {
            Some(("cd", name)) => Ok(Command::Cd(name)),
            _ => {
                let entries = lines
                    .map(|line| Entry::try_from(line))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Command::Ls(entries))
            }
        }
    }
}

fn iter_paths<'a>(current_path: &'a [&'a str]) -> impl Iterator<Item = String> + 'a {
    let path = "/".to_owned();
    std::iter::once(path.clone()).chain(current_path.iter().scan(path, |path, part| {
        path.push_str(part);
        path.push_str("/");
        Some(path.clone())
    }))
}

fn build_directory_sizes(commands: Vec<Command<'_>>) -> HashMap<String, u64> {
    let mut current_path: Vec<&str> = vec![];
    let mut directory_sizes: HashMap<String, u64> = HashMap::default();

    for command in commands {
        match command {
            Command::Cd("/") => {
                current_path.clear();
            }
            Command::Cd("..") => {
                current_path.pop();
            }
            Command::Cd(name) => {
                current_path.push(name);
            }
            Command::Ls(entries) => {
                let entries_size: u64 = entries
                    .iter()
                    .map(|entry| match entry {
                        Entry::Dir(_) => 0,
                        Entry::File(_, size) => *size,
                    })
                    .sum();

                for path in iter_paths(&current_path) {
                    let directory_size = directory_sizes.entry(path).or_default();
                    *directory_size += entries_size;
                }
            }
        }
    }
    directory_sizes
}

fn main() -> Result<(), &'static str> {
    let input = include_str!("./input.txt");
    let commands = input
        .split("$")
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|command| Command::try_from(command))
        .collect::<Result<Vec<_>, _>>()?;

    let directory_sizes = build_directory_sizes(commands);

    let total_small_folders_size = directory_sizes
        .values()
        .copied()
        .filter(|&size| size < 100000)
        .sum::<u64>();

    println!("Part 1: {total_small_folders_size}");

    let size_used = *directory_sizes
        .get("/")
        .ok_or("root folder size not calculated")?;

    let system_size = 70000000;
    let needed_size = 30000000;
    let size_to_free = needed_size - (system_size - size_used);

    let minimum_directory_size_to_delete = directory_sizes
        .values()
        .copied()
        .filter(|&size| size >= size_to_free)
        .min()
        .ok_or("minimum folder size to delete not found")?;

    println!("Part 2: {minimum_directory_size_to_delete}");

    Ok(())
}
