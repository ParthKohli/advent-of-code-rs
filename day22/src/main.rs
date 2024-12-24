use std::{
    collections::{HashMap, HashSet},
    io,
};
const MOD: u64 = 16777216;
const NUM_STEPS: usize = 2000;

fn step(mut x: u64) -> u64 {
    x = (x ^ (x * 64)) % MOD;
    x = (x ^ (x / 32)) % MOD;
    x = (x ^ (x * 2048)) % MOD;
    x
}

fn part_one(seeds: &Vec<u64>) -> u64 {
    let mut res: u64 = 0;
    for &seed in seeds {
        let mut final_value = seed;
        for _step in 0..NUM_STEPS {
            final_value = step(final_value);
        }
        res += final_value;
    }
    res
}

fn part_two(seeds: &Vec<u64>) -> u64 {
    let mut delta_slices_profit: HashMap<(i64, i64, i64, i64), u64> = Default::default();
    for &seed in seeds {
        let mut current_value = seed;
        let mut deltas: Vec<i64> = Vec::new();
        let mut prices: Vec<u64> = Vec::new();
        for _step in 0..NUM_STEPS {
            deltas.push(step(current_value) as i64 % 10 - current_value as i64 % 10);
            current_value = step(current_value);
            prices.push(current_value % 10);
        }

        let mut seen_delta_slices: HashSet<(i64, i64, i64, i64)> = Default::default();
        for (idx, price) in prices.iter().enumerate() {
            if idx >= 3 {
                let current_delta_slice = (
                    deltas[idx - 3],
                    deltas[idx - 2],
                    deltas[idx - 1],
                    deltas[idx],
                );
                if seen_delta_slices.contains(&current_delta_slice) {
                    continue;
                }
                seen_delta_slices.insert(current_delta_slice);
                *delta_slices_profit.entry(current_delta_slice).or_insert(0) += price;
            }
        }
    }
    *delta_slices_profit.values().max().unwrap()
}

fn parse_input() -> Vec<u64> {
    io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect()
}

fn main() {
    let seeds = parse_input();
    println!("{} {}", part_one(&seeds), part_two(&seeds));
}
