#![allow(dead_code)]

use itertools::Itertools;
use std::num::ParseIntError;
use std::{collections::VecDeque, str::FromStr};

pub type Int = i64;
pub type Memory = Vec<Int>;
pub type Address = usize;
pub type Inputs = VecDeque<Int>;
pub type Outputs = VecDeque<Int>;

#[derive(Clone, Debug)]
pub struct Program {
    instruction_pointer: Address,
    relative_base: Int,
    pub memory: Memory,
    pub inputs: Inputs,
    pub outputs: Outputs,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ProgramState {
    Running,
    Input,
    Output,
    Stop,
}

impl Program {
    pub fn input(&mut self, input: Int) {
        self.inputs.push_back(input)
    }

    pub fn output(&mut self) -> Option<Int> {
        self.outputs.pop_back()
    }
}

impl FromStr for Program {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let memory_result: Result<Vec<_>, _> =
            s.trim().split(',').map(&str::parse::<Int>).collect();

        Ok(Program {
            instruction_pointer: 0,
            relative_base: 0,
            memory: memory_result?,
            inputs: VecDeque::new(),
            outputs: VecDeque::new(),
        })
    }
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
enum ParameterMode {
    Address = 0,
    Value = 1,
    Relative = 2,
}

#[derive(Debug)]
enum Operation {
    BinOp(fn(Int, Int) -> Int),
    Input,
    Output,
    Jump(fn(Int) -> bool),
    Rebase,
    Stop,
}

impl Operation {
    fn from(operation_code: Int) -> Operation {
        match operation_code {
            1 => Operation::BinOp(|l, r| l + r),
            2 => Operation::BinOp(|l, r| l * r),
            3 => Operation::Input,
            4 => Operation::Output,
            5 => Operation::Jump(|v| v != 0),
            6 => Operation::Jump(|v| v == 0),
            7 => Operation::BinOp(|l, r| (l < r) as Int),
            8 => Operation::BinOp(|l, r| (l == r) as Int),
            9 => Operation::Rebase,
            99 => Operation::Stop,
            _ => panic!("Unrecognized opcode {}", operation_code),
        }
    }
    fn number_parameters(&self) -> usize {
        match self {
            Operation::BinOp(_) => 3,
            Operation::Input => 1,
            Operation::Output => 1,
            Operation::Jump(_) => 2,
            Operation::Rebase => 1,
            Operation::Stop => 0,
        }
    }
}

#[derive(Debug)]
struct Instruction {
    parameters_modes: Vec<ParameterMode>,
    operation: Operation,
}

impl Instruction {
    fn parameter_value_address(
        &self,
        program: &Program,
        parameter_index: usize,
    ) -> Address {
        let parameter_address =
            program.instruction_pointer + parameter_index + 1;

        match self.parameters_modes[parameter_index] {
            ParameterMode::Value => parameter_address,
            ParameterMode::Address => {
                program.memory[parameter_address] as Address
            }
            ParameterMode::Relative => {
                (program.relative_base + program.memory[parameter_address])
                    as Address
            }
        }
    }

    fn parameter_value(
        &self,
        program: &Program,
        parameter_index: usize,
    ) -> Int {
        let address = self.parameter_value_address(program, parameter_index);
        program.memory.get(address).copied().unwrap_or_default()
    }

    fn parameter_value_mut<'a>(
        &self,
        program: &'a mut Program,
        parameter_index: usize,
    ) -> &'a mut Int {
        let address = self.parameter_value_address(program, parameter_index);
        if program.memory.len() <= address {
            program.memory.resize(address + 1, 0);
        }
        program.memory.get_mut(address).unwrap()
    }

    fn apply(&self, program: &mut Program) -> ProgramState {
        let instruction_pointer = program.instruction_pointer;
        let number_outputs = program.outputs.len();

        match self.operation {
            Operation::BinOp(op) => {
                let left_value = self.parameter_value(program, 0);
                let right_value = self.parameter_value(program, 1);
                let result_value = self.parameter_value_mut(program, 2);
                *result_value = op(left_value, right_value);
            }

            Operation::Input => match program.inputs.pop_front() {
                None => return ProgramState::Input,
                Some(input) => {
                    let value = self.parameter_value_mut(program, 0);
                    *value = input;
                }
            },

            Operation::Output => {
                let output = self.parameter_value(program, 0);
                program.outputs.push_back(output);
            }

            Operation::Jump(predicate) => {
                let parameter = self.parameter_value(program, 0);
                if predicate(parameter) {
                    let destination = self.parameter_value(program, 1);
                    program.instruction_pointer = destination as Address;
                }
            }

            Operation::Rebase => {
                let parameter = self.parameter_value(program, 0);
                program.relative_base += parameter;
            }

            Operation::Stop => {
                return ProgramState::Stop;
            }
        }

        if instruction_pointer == program.instruction_pointer {
            let pointer_delta = self.operation.number_parameters() + 1;
            program.instruction_pointer += pointer_delta;
        }

        if program.outputs.len() != number_outputs {
            ProgramState::Output
        } else {
            ProgramState::Running
        }
    }
}

fn parse_instruction(program: &Program) -> Instruction {
    let instruction_value = program.memory[program.instruction_pointer];

    let operation_code = instruction_value % 100;
    let operation = Operation::from(operation_code);
    let number_parameters = operation.number_parameters();

    let mut parameters_modes = vec![];
    let mut parameter_mask = 100;
    for _ in 0..number_parameters {
        let parameter_mode_int = instruction_value / parameter_mask % 10;
        let parameter_mode: ParameterMode =
            num::FromPrimitive::from_i64(parameter_mode_int)
                .expect("Unrecognized parameter mode");

        parameters_modes.push(parameter_mode);
        parameter_mask *= 10;
    }

    Instruction {
        parameters_modes,
        operation,
    }
}

pub fn run(program: &mut Program) -> ProgramState {
    let mut state = ProgramState::Running;
    while state == ProgramState::Running {
        let instruction = parse_instruction(&program);
        state = instruction.apply(program);
    }

    state
}
