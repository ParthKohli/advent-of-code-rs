use itertools::Itertools;
use regex::Regex;
use std::{cmp::min, io};

enum OperandType {
    Literal,
    Combo,
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    opcode: u8,
    operand: u8,
}

#[derive(Default, Debug, Clone)]
struct VM {
    a: u64,
    b: u64,
    c: u64,

    instructions: Vec<Instruction>,
    raw_instructions: Vec<u8>,
    ip: usize,
}

impl VM {
    fn operand_value(&self, instruction: &Instruction) -> u64 {
        let operand_type = match instruction.opcode {
            0 => OperandType::Combo,
            1 => OperandType::Literal,
            2 => OperandType::Combo,
            3 => OperandType::Literal,
            4 => OperandType::Literal,
            5 => OperandType::Combo,
            6 => OperandType::Combo,
            7 => OperandType::Combo,
            _ => panic!(),
        };
        let operand_value: u64 = match operand_type {
            OperandType::Combo => match instruction.operand {
                0..=3 => instruction.operand as u64,
                4 => self.a,
                5 => self.b,
                6 => self.c,
                _ => panic!("{:?}", &instruction),
            },
            OperandType::Literal => instruction.operand.into(),
        };
        operand_value
    }

    fn run_once(&mut self) -> Option<u8> {
        let instruction = self.instructions[self.ip];
        let operand_value = self.operand_value(&instruction);

        let mut output: Option<u8> = None;
        match instruction.opcode {
            0 => {
                self.a = self.a >> operand_value;
            }
            1 => {
                self.b ^= operand_value;
            }
            2 => {
                self.b = operand_value % 8;
            }
            3 => {
                if self.a != 0 {
                    if operand_value % 2 == 1 {
                        panic!();
                    }
                    self.ip = (operand_value as usize) / 2;
                    return None;
                }
            }
            4 => {
                self.b ^= self.c;
            }
            5 => {
                output = Some((operand_value % 8).try_into().unwrap());
            }
            6 => {
                self.b = self.a >> operand_value;
            }
            7 => {
                self.c = self.a >> operand_value;
            }
            _ => panic!(),
        }
        self.ip += 1;
        output
    }

    fn run(&mut self) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::new();
        loop {
            if self.ip >= self.instructions.len() {
                return output;
            }
            if let Some(value) = self.run_once() {
                output.push(value);
            }
        }
    }
}

fn parse() -> VM {
    let register_re = Regex::new(r"Register (A|B|C): (\d+)").unwrap();
    let register_lines: Vec<_> = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    let mut vm = VM::default();
    for (idx, line) in register_lines.into_iter().enumerate() {
        if let Some(captures) = register_re.captures(&line) {
            let register_value: u64 = captures.get(2).unwrap().as_str().parse::<u64>().unwrap();
            match idx {
                0 => vm.a = register_value,
                1 => vm.b = register_value,
                2 => vm.c = register_value,
                _ => panic!(),
            };
        }
    }
    let line = io::stdin().lines().next().unwrap().unwrap();
    let raw_instructions: Vec<_> = line
        .strip_prefix("Program: ")
        .unwrap()
        .split(",")
        .map(|token| token.parse().unwrap())
        .collect();
    vm.raw_instructions = raw_instructions.clone();
    let instructions = raw_instructions
        .into_iter()
        .tuples()
        .map(|(opcode, operand)| Instruction { opcode, operand })
        .collect();
    vm.instructions = instructions;
    vm
}

fn part_one(mut vm: VM) -> String {
    vm.run().into_iter().join(",")
}

fn is_stable(mut vm: VM, a_register: u64) -> bool {
    vm.a = a_register;
    vm.b = 0;
    vm.c = 0;
    vm.ip = 0;
    return vm.run() == vm.raw_instructions;
}

// For this, I had to translate to pseudocode and observed that it operates
// in bit-chunks of length 3:
// while A:
//     B = A % 8
//     B = B ^ 7
//     C = A >> B
//     B = B ^ 7
//     A = A >> 3
//     B = B ^ C
//     out(B % 8)
fn search(idx: u64, so_far: u64, expected: &Vec<u8>) -> Option<u64> {
    if idx == expected.len() as u64 {
        return Some(so_far);
    }
    let mut min_result: Option<u64> = None;
    for next_chunk in 0..8 as u8 {
        let candidate = so_far + ((next_chunk as u64) << (3 * (expected.len() as u64 - idx - 1)));
        let scaled_candidate = candidate >> (3 * (expected.len() as u64 - idx - 1));
        if next_chunk ^ ((scaled_candidate >> (next_chunk ^ 7)) & 7) as u8 != expected[idx as usize]
        {
            continue;
        }
        let next_result = search(idx + 1, candidate, expected);
        if let Some(next_value) = next_result {
            min_result = match min_result {
                None => next_result,
                Some(value) => Some(min(value, next_value)),
            }
        }
    }
    min_result
}

fn part_two(raw_instructions: Vec<u8>) -> u64 {
    search(0, 0, &raw_instructions.iter().rev().cloned().collect()).unwrap()
}

fn main() {
    let initial_vm = parse();
    let part_one = part_one(initial_vm.clone());
    let part_two = part_two(initial_vm.raw_instructions.clone());
    assert!(is_stable(initial_vm, part_two));
    println!("{:?} {:?}", part_one, part_two);
}
