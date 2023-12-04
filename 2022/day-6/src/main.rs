fn main() {
    let input = include_bytes!("./input.txt");

    let start_of_packet_index =
        find_marker(input, Marker::Packet).expect("expected start of packet marker to be found");
    println!("Part 1: {start_of_packet_index}");

    let start_of_message_index =
        find_marker(input, Marker::Message).expect("expected start of message marker to be found");
    println!("Part 2: {start_of_message_index}");
}

enum Marker {
    Packet,
    Message,
}

fn find_marker(input: &[u8], marker: Marker) -> Option<usize> {
    let window_size = match marker {
        Marker::Packet => 4,
        Marker::Message => 14,
    };

    let mut counts = [0u8; 26usize];
    for index in 0..input.len() {
        if index >= window_size && counts.iter().copied().all(|c| c <= 1) {
            return Some(index);
        }

        let current_byte = input[index];
        counts[(current_byte - b'a') as usize] += 1;

        if index >= window_size {
            let previous_byte = input[index - window_size];
            counts[(previous_byte - b'a') as usize] -= 1;
        }
    }

    None
}
