use std::collections::BTreeSet;
use std::io;

struct Equation {
    operands: Vec<u64>,
    result: u64,
}

fn parse_input() -> Vec<Equation> {
    let lines = io::stdin()
        .lines()
        .map(|x| x.unwrap())
        .filter(|x| !x.is_empty());

    let mut equations = Vec::new();
    for line in lines {
        let (result, operands) = line.split_once(':').unwrap();
        let result: u64 = result.parse().unwrap();
        let operands: Vec<u64> = operands
            .split_whitespace()
            .map(|str| str.parse().unwrap())
            .collect();
        equations.push(Equation { result, operands });
    }

    equations
}

fn part_one_next_candidates(value_so_far: u64, operand: u64) -> Vec<u64> {
    vec![value_so_far * operand, value_so_far + operand]
}

fn part_two_next_candidates(value_so_far: u64, operand: u64) -> Vec<u64> {
    vec![
        value_so_far * operand,
        value_so_far + operand,
        (value_so_far.to_string() + &operand.to_string())
            .parse()
            .unwrap(),
    ]
}

fn check_possible(
    equation: &Equation,
    generate_next_candidate: impl Fn(u64, u64) -> Vec<u64>,
) -> bool {
    let mut possible_values: BTreeSet<u64> = BTreeSet::new();
    possible_values.insert(equation.operands[0]);
    for operand in &equation.operands[1..] {
        let mut new_possible_values: BTreeSet<u64> = BTreeSet::new();
        for possible_value in possible_values.iter() {
            for next_candidate in generate_next_candidate(*possible_value, *operand) {
                if next_candidate <= equation.result {
                    new_possible_values.insert(next_candidate);
                }
            }
        }
        possible_values = new_possible_values;
    }
    possible_values.contains(&equation.result)
}

fn part_one(equations: &[Equation]) -> u64 {
    equations
        .iter()
        .filter(|equation| check_possible(equation, part_one_next_candidates))
        .map(|equation| equation.result)
        .sum()
}

fn part_two(equations: &[Equation]) -> u64 {
    equations
        .iter()
        .filter(|equation| check_possible(equation, part_two_next_candidates))
        .map(|equation| equation.result)
        .sum()
}

fn main() {
    let equations = parse_input();
    println!("{} {}", part_one(&equations), part_two(&equations));
}
