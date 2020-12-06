use std::env;
use std::fs;

type SeatId = u16;

fn compute_seat_id(seat_path: &[u8]) -> SeatId {
    match seat_path.last() {
        None => 0,
        Some(&last_byte) => {
            let last_index = seat_path.len() - 1;
            let partial_seat_id = compute_seat_id(&seat_path[..last_index]);
            let least_significant_bit =
                (last_byte == b'B' || last_byte == b'R') as SeatId;
            (partial_seat_id << 1) + least_significant_bit
        }
    }
}

fn find_missing_seat(mut seats_id: Vec<SeatId>) -> SeatId {
    seats_id.sort();

    let next_seats = seats_id.iter().enumerate().skip(1);
    for (next_seat_index, &next_seat_id) in next_seats {
        let prev_seat_index = next_seat_index - 1;
        let prev_seat_id: SeatId = seats_id[prev_seat_index];
        if prev_seat_id + 1 < next_seat_id {
            return prev_seat_id + 1;
        }
    }

    panic!("The missing seat could not be found")
}

fn solve(content: &str) {
    let seats_id: Vec<SeatId> = content
        .lines()
        .map(|line_str: &str| line_str.as_bytes())
        .map(|line: &[u8]| compute_seat_id(line))
        .collect();

    let seat_id = find_missing_seat(seats_id);
    println!("Seat id: {}", seat_id)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let default_filename: &str = "./res/input.txt";
    let filename: &str =
        args.get(1).map(|s| s.as_ref()).unwrap_or(default_filename);

    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    solve(&content);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn compute_seat_id_test() {
        let test = |s: &str, e| assert_eq!(compute_seat_id(s.as_bytes()), e);
        test("FBFBBFFRLR", 357);
    }
}
