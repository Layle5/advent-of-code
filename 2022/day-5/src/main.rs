type Stack = Vec<char>;

#[derive(Debug, Clone)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_stacks_and_moves() -> (Vec<Stack>, Vec<Move>) {
    let input = include_str!("./input.txt");
    let (stacks_lines, move_lines) = input
        .split_once("\n\n")
        .expect("could not separate stacks from moves");

    let mut stacks_lines_iter = stacks_lines.lines().rev();
    let stacks_number = (stacks_lines_iter
        .next()
        .expect("at least one stack lines is required")
        .len()
        + 1)
        / 4;

    let stacks = stacks_lines_iter.fold(vec![vec![]; stacks_number], |mut stacks, line| {
        let mut stack_index = 0usize;
        while let Some(&byte) = line.as_bytes().get(stack_index * 4 + 1) {
            let char = byte as char;
            if char.is_alphabetic() {
                stacks
                    .get_mut(stack_index)
                    .expect("got crate outside of stack number")
                    .push(char);
            }

            stack_index += 1;
        }

        stacks
    });

    let moves = move_lines
        .lines()
        .map(|line| {
            let mut line_split = line.split(|c| c == ' ').skip(1);
            let count_str = line_split.next().expect("expected `count` for move");
            let mut line_split = line_split.skip(1);
            let from_str = line_split.next().expect("expected `from` for move");
            let mut line_split = line_split.skip(1);
            let to_str = line_split.next().expect("expected `to` for move");
            Move {
                count: count_str
                    .parse()
                    .expect("expected move `count` to be a number"),
                from: from_str
                    .parse()
                    .expect("expected move `from` to be a number"),
                to: to_str.parse().expect("expected move `to` to be a number"),
            }
        })
        .collect();

    (stacks, moves)
}

enum CraneBehavior {
    ReverseOrder,
    RetainOrder,
}

fn move_crates(mut stacks: Vec<Stack>, moves: &[Move], crane_behavior: CraneBehavior) -> String {
    for current_move in moves {
        let from_stack = stacks
            .get_mut(current_move.from - 1)
            .expect("expected from stack for move");

        let moved_crate_index = from_stack.len().saturating_sub(current_move.count);
        let mut moved_crates: Vec<_> = from_stack.drain(moved_crate_index..).collect();

        let to_stack = stacks
            .get_mut(current_move.to - 1)
            .expect("expected to stack for move");

        match crane_behavior {
            CraneBehavior::ReverseOrder => moved_crates.reverse(),
            CraneBehavior::RetainOrder => {}
        }

        to_stack.append(&mut moved_crates);
    }

    let first_crates: String = stacks
        .into_iter()
        .filter_map(|stack| stack.last().copied())
        .collect();

    first_crates
}

fn main() {
    let (stacks, moves) = parse_stacks_and_moves();

    let crates_part_one = move_crates(stacks.clone(), &moves, CraneBehavior::ReverseOrder);
    println!("Part 1: {crates_part_one}");

    let crates_part_two = move_crates(stacks, &moves, CraneBehavior::RetainOrder);
    println!("Part 2: {crates_part_two}");
}
