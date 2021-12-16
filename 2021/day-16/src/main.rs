use itertools::Itertools;
use std::collections::HashMap;

type Bit = u8;

#[derive(Debug)]
enum PacketValue {
    Literal(u64),
    Operator(u64, Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: u64,
    value: PacketValue,
}

#[derive(Debug)]
struct ParseResult<T> {
    length: usize,
    result: T,
}

fn parse_bool<T: Iterator<Item = Bit>>(iter: &mut T) -> ParseResult<bool> {
    let bit = iter.next().unwrap();
    let result = bit != 0;
    ParseResult { length: 1, result }
}

fn parse_bits<T: Iterator<Item = Bit>>(iter: &mut T, length: usize) -> ParseResult<u64> {
    let mut result = 0;
    for _ in 0..length {
        let bit = iter.next().unwrap();
        result = result * 2 + bit as u64;
    }
    ParseResult { length, result }
}

fn parse_literal_value<T: Iterator<Item = Bit>>(iter: &mut T) -> ParseResult<u64> {
    let mut length = 0;
    let mut result = 0;

    loop {
        let continue_after = parse_bool(iter);
        let part = parse_bits(iter, 4);
        length += continue_after.length + part.length;
        result = result * 16 + part.result;
        if !continue_after.result {
            break;
        }
    }

    ParseResult { length, result }
}

fn parse_packets<T, F>(iter: &mut T, should_continue_parsing: F) -> ParseResult<Vec<Packet>>
where
    T: Iterator<Item = Bit>,
    F: Fn(&ParseResult<Vec<Packet>>) -> bool,
{
    let mut packets = ParseResult {
        length: 0,
        result: vec![],
    };

    while should_continue_parsing(&packets) {
        let packet = parse_packet(iter);
        packets.length += packet.length;
        packets.result.push(packet.result);
    }

    packets
}

fn parse_packets_with_number<T: Iterator<Item = Bit>>(iter: &mut T) -> ParseResult<Vec<Packet>> {
    let target_count = parse_bits(iter, 11);
    let packets = parse_packets(iter, |r| r.result.len() < target_count.result as usize);

    ParseResult {
        length: target_count.length + packets.length,
        result: packets.result,
    }
}

fn parse_packets_with_length<T: Iterator<Item = Bit>>(iter: &mut T) -> ParseResult<Vec<Packet>> {
    let target_length = parse_bits(iter, 15);
    let packets = parse_packets(iter, |r| r.length < target_length.result as usize);

    ParseResult {
        length: target_length.length + packets.length,
        result: packets.result,
    }
}

fn parse_sub_packets<T: Iterator<Item = Bit>>(iter: &mut T) -> ParseResult<Vec<Packet>> {
    let length_type_id = parse_bool(iter);
    let sub_packets = if length_type_id.result {
        parse_packets_with_number(iter)
    } else {
        parse_packets_with_length(iter)
    };

    ParseResult {
        length: length_type_id.length + sub_packets.length,
        result: sub_packets.result,
    }
}

fn parse_packet_value<T: Iterator<Item = Bit>>(iter: &mut T) -> ParseResult<PacketValue> {
    let type_id = parse_bits(iter, 3);
    match type_id.result {
        4 => {
            let literal_value = parse_literal_value(iter);
            ParseResult {
                length: type_id.length + literal_value.length,
                result: PacketValue::Literal(literal_value.result),
            }
        }
        _ => {
            let sub_packets_value = parse_sub_packets(iter);
            ParseResult {
                length: type_id.length + sub_packets_value.length,
                result: PacketValue::Operator(type_id.result, sub_packets_value.result),
            }
        }
    }
}

fn parse_packet<T: Iterator<Item = Bit>>(iter: &mut T) -> ParseResult<Packet> {
    let version = parse_bits(iter, 3);
    let packet_value = parse_packet_value(iter);

    ParseResult {
        length: version.length + packet_value.length,
        result: Packet {
            version: version.result,
            value: packet_value.result,
        },
    }
}

fn parse(input: &str) -> Packet {
    let bits_per_byte: HashMap<u8, [u8; 4]> = [
        (b'0', [0, 0, 0, 0]),
        (b'1', [0, 0, 0, 1]),
        (b'2', [0, 0, 1, 0]),
        (b'3', [0, 0, 1, 1]),
        (b'4', [0, 1, 0, 0]),
        (b'5', [0, 1, 0, 1]),
        (b'6', [0, 1, 1, 0]),
        (b'7', [0, 1, 1, 1]),
        (b'8', [1, 0, 0, 0]),
        (b'9', [1, 0, 0, 1]),
        (b'A', [1, 0, 1, 0]),
        (b'B', [1, 0, 1, 1]),
        (b'C', [1, 1, 0, 0]),
        (b'D', [1, 1, 0, 1]),
        (b'E', [1, 1, 1, 0]),
        (b'F', [1, 1, 1, 1]),
    ]
    .into_iter()
    .collect();

    let mut iter = input
        .trim()
        .as_bytes()
        .iter()
        .flat_map(|byte| bits_per_byte.get(byte))
        .flatten()
        .copied();

    let parse_result = parse_packet(&mut iter);
    parse_result.result
}

fn solve_part_1(packet: &Packet) -> u64 {
    packet.version
        + match &packet.value {
            PacketValue::Literal(_) => 0,
            PacketValue::Operator(_, sub_packets) => sub_packets.iter().map(solve_part_1).sum(),
        }
}

fn solve_part_2(packet: &Packet) -> u64 {
    match &packet.value {
        PacketValue::Literal(value) => *value,
        PacketValue::Operator(type_id, sub_packets) => {
            let sub_values = sub_packets.iter().map(solve_part_2);
            match type_id {
                0 => sub_values.sum(),
                1 => sub_values.product(),
                2 => sub_values.min().unwrap(),
                3 => sub_values.max().unwrap(),
                5 => sub_values.tuple_windows().all(|(a, b)| a > b) as u64,
                6 => sub_values.tuple_windows().all(|(a, b)| a < b) as u64,
                7 => sub_values.tuple_windows().all(|(a, b)| a == b) as u64,
                _ => panic!("unrecognized operator type id"),
            }
        }
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let packet = parse(input);
    println!("Part 1: {}", solve_part_1(&packet));
    println!("Part 2: {}", solve_part_2(&packet));
}
