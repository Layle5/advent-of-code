use std::collections::HashSet;

type Position = (isize, isize);
type Move = (usize, Position);

fn main() {
    let input = include_str!("./input.txt");
    let head_moves = parse_moves(input);
    println!("Part 1: {}", move_rope::<2>(&head_moves));
    println!("Part 2: {}", move_rope::<10>(&head_moves));
}

fn parse_moves(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| {
            let (direction_str, distance_str) = line.trim().split_once(' ').unwrap();
            let distance = distance_str.parse().unwrap();
            let direction = match direction_str {
                "U" => (-1, 0),
                "D" => (1, 0),
                "L" => (0, -1),
                "R" => (0, 1),
                d => panic!("unreconized direction {d}"),
            };
            
            (distance, direction)
        })
        .collect()
}

fn move_rope<const N: usize>(head_moves: &[Move]) -> usize {
    let mut rope = [(0isize, 0isize); N];
    let mut tail_positions: HashSet<Position> = HashSet::from_iter([rope[N - 1]]);

    for (distance, (delta_row, delta_col)) in head_moves {
        for _ in 0..*distance {
            let head = &mut rope[0];
            *head = (head.0 + delta_row, head.1 + delta_col);

            for next_index in 1..rope.len() {
                let prev = rope[next_index - 1];
                let next = &mut rope[next_index];

                let diff_row = prev.0 - next.0;
                let diff_col = prev.1 - next.1;
                let need_to_move_tail = diff_row.abs() >= 2 || diff_col.abs() >= 2;
                if !need_to_move_tail {
                    continue;
                }

                *next = (
                    next.0 + diff_row.clamp(-1, 1),
                    next.1 + diff_col.clamp(-1, 1),
                );
            }

            tail_positions.insert(*rope.last().unwrap());
        }
    }

    tail_positions.len()
}
