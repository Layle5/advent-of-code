mod computer;

#[macro_use]
extern crate num_derive;

use computer::{run_outputs, Int, Program};
use itertools::{Itertools};
use std::env;
use std::fs;

type Message = (Int, Int, Int);

fn parse(content: &str) -> Vec<Program> {
    let program: Program = content.parse().unwrap();
    let mut computers = vec![program; 50];

    for (index, computer) in computers.iter_mut().enumerate() {
        computer.input(index as Int);
    }

    computers
}

fn network(mut computers: Vec<Program>, nat: bool) -> Int {
    let mut nat_message_op: Option<Message> = None;
    let mut nat_sent_message_op: Option<Message> = None;

    while nat || nat_message_op.is_none() {
        let mut messages: Vec<Message> = vec![];

        for computer in &mut computers {
            computer.input(-1);
            run_outputs(computer);

            let outputs_iter = computer.outputs.iter().copied();
            messages.extend(outputs_iter.tuple_windows::<Message>());
            computer.outputs.clear();
        }

        if nat && messages.is_empty() {
            let (_, x, y) = nat_message_op.unwrap();
            let sent_y_op = nat_sent_message_op.map(|t| t.2);
            if let Some(sent_y) = sent_y_op {
                if sent_y == y {
                    break;
                }
            }

            computers[0].input(x);
            computers[0].input(y);
            nat_sent_message_op = nat_message_op;
        }

        for (address, x, y) in messages {
            if let Some(computer) = computers.get_mut(address as usize) {
                computer.input(x);
                computer.input(y);
            } else if address == 255 {
                nat_message_op = Some((address, x, y));
            }
        }
    }

    nat_message_op.unwrap().2
}

fn solve_part_1(content: &str) {
    let computers = parse(content);
    let y = network(computers, false);
    println!("Part 1: {}", y);
}

fn solve_part_2(content: &str) {
    let computers = parse(content);
    let y = network(computers, true);
    println!("Part 2: {}", y);
}

fn main() {
    let args = env::args().collect_vec();
    let filename = args.get(1).map(|s| s.as_ref()).unwrap_or("./res/input.txt");

    let content = fs::read_to_string(filename).unwrap();

    solve_part_1(&content);
    solve_part_2(&content);
}
