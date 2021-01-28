mod computer;

#[macro_use]
extern crate num_derive;

use computer::{run_outputs, Int, Program};
use itertools::Itertools;
use std::env;
use std::fs;

fn run(program: &mut Program, request: &str) -> String {
    let command = request.chars().map(|chr| chr as Int);
    program.inputs.extend(command);
    program.input(b'\n' as Int);

    run_outputs(program);

    let response = program
        .outputs
        .iter()
        .map(|output| *output as u8 as char)
        .collect::<String>();
    program.outputs.clear();

    response
}

fn move_checkpoint(program: &mut Program) {
    let path = vec![
        "east",
        "take food ration",
        "south",
        "take prime number",
        "north",
        "east",
        "take manifold",
        "east",
        "north",
        "north",
        "take fuel cell",
        "south",
        "east",
        "take spool of cat6",
        "west",
        "south",
        "east",
        "take jam",
        "west",
        "west",
        "west",
        "west",
        "north",
        "north",
        "north",
        "east",
        "east",
        "take loom",
        "west",
        "west",
        "south",
        "west",
        "take mug",
        "east",
        "south",
        "west",
        "north",
        "west",
        "north",
    ];

    for request in path {
        run(program, request);
    }
}

fn find_code(program: &mut Program) -> Option<usize> {
    let items = [
        "jam",
        "loom",
        "mug",
        "spool of cat6",
        "prime number",
        "food ration",
        "fuel cell",
        "manifold",
    ];

    for item in &items {
        let request = format!("drop {}", item);
        run(program, &request);
    }

    for combination_size in 0..items.len() {
        for combination in items.iter().combinations(combination_size) {
            for item in &combination {
                let request = format!("take {}", item);
                run(program, &request);
            }

            let response = run(program, "north");
            if response.contains("Analysis complete!") {
                let s = "by typing ";
                let start = response.find(s).unwrap() + s.len();
                let end = start + response[start..].find(' ').unwrap();
                let code = response[start..end].parse().unwrap();
                return Some(code);
            }

            for item in &combination {
                let request = format!("drop {}", item);
                run(program, &request);
            }
        }
    }

    None
}

fn solve(content: &str) {
    let mut program: Program = content.parse().unwrap();
    move_checkpoint(&mut program);
    let code = find_code(&mut program).unwrap();
    println!("Airlock Code: {}", code);
}

fn main() {
    let args = env::args().collect_vec();
    let filename = args.get(1).map(|s| s.as_ref()).unwrap_or("./res/input.txt");

    let content = fs::read_to_string(filename).unwrap();

    solve(&content);
}
