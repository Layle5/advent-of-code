use itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt");
    let instructions = parse_instructions(input);
    let (sum_signal_strengths, displayed_screen) = run_instructions(instructions);
    println!("Part 1: {}", sum_signal_strengths);
    println!("Part 2: \n{}", displayed_screen);
}

struct Instruction {
    cycles_needed: usize,
    add_register: i64,
}

fn run_instructions(instructions: Vec<Instruction>) -> (i64, String) {
    let mut register: i64 = 1;
    let mut completed_cycles: i64 = 0;
    let mut number_cycles_to_wait: usize = 0;
    let mut signal_strengths: Vec<i64> = vec![];
    let mut instruction_index = 0;
    let mut screen = [['.'; 40]; 6];

    while instruction_index < instructions.len() {
        let current_cycle = completed_cycles + 1;

        if completed_cycles < 220 && current_cycle % 40 == 20 {
            signal_strengths.push((current_cycle as i64) * register);
        }

        let current_row = (current_cycle - 1) / 40;
        let current_pixel = (current_cycle - 1) % 40;
        let show_pixel = register - 1 <= current_pixel && current_pixel <= register + 1;
        screen[current_row as usize][current_pixel as usize] = if show_pixel { '█' } else { ' ' };

        let instruction = &instructions[instruction_index];
        if number_cycles_to_wait == 0 {
            number_cycles_to_wait = instruction.cycles_needed;
        }

        number_cycles_to_wait -= 1;

        if number_cycles_to_wait == 0 {
            register += instruction.add_register;
            instruction_index += 1;
        }

        completed_cycles += 1;
    }

    let sum_signal_strengths = signal_strengths.into_iter().sum::<i64>();
    let displayed_screen = screen.into_iter().map(String::from_iter).join("\n");
    (sum_signal_strengths, displayed_screen)
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| match line.trim() {
            "noop" => Instruction {
                cycles_needed: 1,
                add_register: 0,
            },
            line => match line.split_once(' ') {
                Some(("addx", n)) => {
                    Instruction {
                        cycles_needed: 2,
                        add_register: n.parse().unwrap(),
                    }
                }
                _ => panic!("unrecognized instruction {line}"),
            },
        })
        .collect()
}
