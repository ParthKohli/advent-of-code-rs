use std::{collections::HashMap, io};

fn read_input() -> Vec<u64> {
    let line = io::stdin().lines().next().unwrap().unwrap();
    line.split_whitespace()
        .map(|chunk| chunk.parse::<u64>().unwrap())
        .collect()
}

fn evolve_number(num: u64) -> Vec<u64> {
    if num == 0 {
        return vec![1];
    }
    let num_str = num.to_string();
    if num_str.len() % 2 == 0 {
        let half_length = num_str.len() / 2;
        let first_half = num_str[..half_length].parse::<u64>().unwrap();
        let second_half = num_str[half_length..].parse::<u64>().unwrap();
        vec![first_half, second_half]
    } else {
        vec![num * 2024]
    }
}

/* Written for part 1. */
fn evolve_sequence_naive(seq: Vec<u64>) -> Vec<u64> {
    seq.into_iter()
        .map(evolve_number)
        .collect::<Vec<_>>()
        .concat()
}

/* Written for part 1. */
fn evolve_sequence_multiple_times_naive(mut seq: Vec<u64>, num_evolutions: u64) -> usize {
    for _evolution in 0..num_evolutions {
        seq = evolve_sequence_naive(seq);
    }
    seq.len()
}

/* Written for part 2. */
fn evolve_sequence_using_counts(counts: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_counts: HashMap<u64, u64> = HashMap::new();
    for (&key, &key_count) in counts.iter() {
        for new_key in evolve_number(key) {
            *new_counts.entry(new_key).or_insert(0) += key_count;
        }
    }
    new_counts
}

/* Written for part 2. */
fn evolve_sequence_multiple_times_using_counts(seq: Vec<u64>, num_evolutions: u64) -> u64 {
    let mut counts: HashMap<u64, u64> = HashMap::new();
    for elem in seq {
        *counts.entry(elem).or_insert(0) += 1;
    }
    for _evolution in 0..num_evolutions {
        counts = evolve_sequence_using_counts(counts);
    }
    counts.values().sum()
}

fn main() {
    let seq = read_input();
    println!(
        "{} {}",
        evolve_sequence_multiple_times_naive(seq.clone(), 25), // part 1
        evolve_sequence_multiple_times_using_counts(seq.clone(), 75)  // part 2
    );
}
