use std::collections::{BinaryHeap, HashSet};

type Amphipod = u8;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Burrow {
    opens: [Option<Amphipod>; 7],
    rooms: [Vec<Option<Amphipod>>; 4],
}

#[derive(Clone, Debug)]
struct State {
    burrow: Burrow,
    cost: u64,
}

type Paths = [[Vec<usize>; 4]; 7];

fn get_paths() -> Paths {
    [
        [
            vec![0, 1],
            vec![0, 1, 2],
            vec![0, 1, 2, 3],
            vec![0, 1, 2, 3, 4],
        ],
        [vec![1], vec![1, 2], vec![1, 2, 3], vec![1, 2, 3, 4]],
        [vec![2], vec![2], vec![2, 3], vec![2, 3, 4]],
        [vec![3, 2], vec![3], vec![3], vec![3, 4]],
        [vec![4, 3, 2], vec![4, 3], vec![4], vec![4]],
        [vec![5, 4, 3, 2], vec![5, 4, 3], vec![5, 4], vec![5]],
        [
            vec![6, 5, 4, 3, 2],
            vec![6, 5, 4, 3],
            vec![6, 5, 4],
            vec![6, 5],
        ],
    ]
}

fn parse(input: &str) -> Burrow {
    let mut lines = input.trim().lines();
    lines.next().unwrap();
    lines.next().unwrap();
    let line1 = lines.next().unwrap().as_bytes();
    let line2 = lines.next().unwrap().as_bytes();

    let mut burrow = Burrow::default();
    burrow.rooms[0].push(Some(line1[3]));
    burrow.rooms[0].push(Some(line2[3]));
    burrow.rooms[1].push(Some(line1[5]));
    burrow.rooms[1].push(Some(line2[5]));
    burrow.rooms[2].push(Some(line1[7]));
    burrow.rooms[2].push(Some(line2[7]));
    burrow.rooms[3].push(Some(line1[9]));
    burrow.rooms[3].push(Some(line2[9]));

    burrow
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.burrow == other.burrow
    }
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn get_move_cost(
    paths: &Paths,
    open_index: usize,
    room_index: usize,
    room_position: usize,
    amphipod: Amphipod,
) -> u64 {
    let room_distance = 2 * paths[open_index][room_index].len() as u64 + room_position as u64;
    let open_ajusted_distance = room_distance - ([0, 6].contains(&open_index) as u64);
    open_ajusted_distance * 10u64.pow((amphipod - b'A') as u32)
}

fn is_done(state: &State) -> bool {
    state.burrow.opens.iter().all(|amphipod| amphipod.is_none())
        && state
            .burrow
            .rooms
            .iter()
            .enumerate()
            .all(|(room_index, amphipods)| {
                amphipods
                    .iter()
                    .all(|amphipod| amphipod == &Some(room_index as Amphipod + b'A'))
            })
}

fn try_move_into_room(
    paths: &Paths,
    state: &State,
    open_index: usize,
    room_index: usize,
) -> Option<usize> {
    if !is_path_to_room_clear(paths, state, open_index, room_index, true) {
        return None;
    }

    let room_space = &state.burrow.rooms[room_index];
    let amphipod = room_index as u8 + b'A';
    room_space
        .iter()
        .enumerate()
        .take_while(|(_, amphipod)| amphipod.is_none())
        .last()
        .map(|(room_position, _)| room_position)
        .filter(|room_position| {
            room_space[*room_position + 1..]
                .iter()
                .all(|o| *o == Some(amphipod))
        })
}

fn is_path_to_room_clear(
    paths: &Paths,
    state: &State,
    open_index: usize,
    room_index: usize,
    can_open_be_occupied: bool,
) -> bool {
    let path = &paths[open_index][room_index];
    path.iter()
        .skip(can_open_be_occupied as usize)
        .all(|&i| state.burrow.opens[i].is_none())
}

fn should_leave_room(state: &State, room_index: usize, room_position: usize) -> bool {
    let room_space = &state.burrow.rooms[room_index];
    let amphipod = room_index as u8 + b'A';
    let can_leave = room_space[..room_position].iter().all(Option::is_none);
    let must_stay = room_space[room_position..]
        .iter()
        .all(|a| *a == Some(amphipod));

    can_leave && !must_stay
}

fn get_next_states(paths: &Paths, state: &State) -> Vec<State> {
    for (open_index, open_space) in state.burrow.opens.iter().enumerate() {
        if let Some(amphipod) = open_space {
            let room_index = (amphipod - b'A') as usize;
            if let Some(room_position) = try_move_into_room(paths, state, open_index, room_index) {
                let mut next_state = state.clone();
                next_state.burrow.opens[open_index] = None;
                next_state.burrow.rooms[room_index][room_position] = Some(*amphipod);
                let move_cost =
                    get_move_cost(paths, open_index, room_index, room_position, *amphipod);
                next_state.cost += move_cost;
                return vec![next_state];
            }
        }
    }

    let mut next_states = vec![];

    for (room_index, room_space) in state.burrow.rooms.iter().enumerate() {
        for (room_position, amphipod_op) in room_space.iter().enumerate() {
            if let Some(amphipod) = amphipod_op {
                if should_leave_room(state, room_index, room_position) {
                    for open_index in 0..state.burrow.opens.len() {
                        if is_path_to_room_clear(paths, state, open_index, room_index, false) {
                            let mut next_state = state.clone();
                            next_state.burrow.rooms[room_index][room_position] = None;
                            next_state.burrow.opens[open_index] = Some(*amphipod);
                            let move_cost = get_move_cost(
                                paths,
                                open_index,
                                room_index,
                                room_position,
                                *amphipod,
                            );
                            next_state.cost += move_cost;
                            next_states.push(next_state);
                        }
                    }
                }
            }
        }
    }

    next_states
}

impl State {
    fn id(&self) -> u128 {
        let amphipods = self
            .burrow
            .opens
            .iter()
            .chain(self.burrow.rooms.iter().flat_map(|r| r.iter()));

        amphipods.fold(self.cost as u128, |c, a| {
            let v = a.unwrap_or_default();
            c * 5 + v as u128
        })
    }
}

fn solve(burrow: Burrow) -> Option<u64> {
    let paths = get_paths();
    let state = State { burrow, cost: 0 };

    let mut encountered_ids: HashSet<u128> = HashSet::new();
    let mut heap = BinaryHeap::new();
    encountered_ids.insert(state.id());
    heap.push(state);

    while let Some(current_state) = heap.pop() {
        if is_done(&current_state) {
            return Some(current_state.cost);
        }

        let next_states = get_next_states(&paths, &current_state);
        for next_state in next_states {
            let id = next_state.id();
            if encountered_ids.contains(&id) {
                continue;
            }

            heap.push(next_state);
            encountered_ids.insert(id);
        }
    }

    None
}

fn solve_part_1(burrow: Burrow) -> u64 {
    solve(burrow).unwrap()
}

fn solve_part_2(mut burrow: Burrow) -> u64 {
    fn add_lines_to_burrow(burrow: &mut Burrow) {
        burrow.rooms[0].insert(1, Some(b'D'));
        burrow.rooms[0].insert(2, Some(b'D'));
        burrow.rooms[1].insert(1, Some(b'C'));
        burrow.rooms[1].insert(2, Some(b'B'));
        burrow.rooms[2].insert(1, Some(b'B'));
        burrow.rooms[2].insert(2, Some(b'A'));
        burrow.rooms[3].insert(1, Some(b'A'));
        burrow.rooms[3].insert(2, Some(b'C'));
    }

    add_lines_to_burrow(&mut burrow);
    solve(burrow).unwrap()
}

fn main() {
    let input = include_str!("./input.txt");
    let burrow = parse(input);
    println!("Part 1: {}", solve_part_1(burrow.clone()));
    println!("Part 2: {}", solve_part_2(burrow));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("./example.txt");

    #[test]
    fn example_part_1() {
        let burrow = parse(EXAMPLE_INPUT);
        assert_eq!(solve_part_1(burrow), 12521);
    }

    #[test]
    fn example_part_2() {
        let burrow = parse(EXAMPLE_INPUT);
        assert_eq!(solve_part_2(burrow), 44169);
    }
}
