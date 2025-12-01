use itertools::Itertools;
use std::{io, iter::zip};

type Seq = Vec<usize>;

struct LocksAndKeys {
    locks: Vec<Seq>,
    keys: Vec<Seq>,
}

fn parse_input() -> LocksAndKeys {
    let mut locks: Vec<Seq> = Vec::new();
    let mut keys: Vec<Seq> = Vec::new();
    loop {
        let grid: Vec<Vec<char>> = io::stdin()
            .lines()
            .map(|line| line.unwrap())
            .take_while(|line| !line.is_empty())
            .map(|line| line.chars().collect())
            .collect();
        if grid.is_empty() {
            break;
        }
        let mut seq: Seq = Vec::new();
        for column in 0..5_usize {
            let num_pins = (0..6_usize)
                .map(|row| grid[row][column])
                .filter(|&c| c == '#')
                .count();
            seq.push(num_pins);
        }
        match grid[0][0] {
            '.' => {
                keys.push(seq);
            }
            '#' => {
                locks.push(seq);
            }
            _ => panic!(),
        }
    }
    LocksAndKeys { locks, keys }
}

fn compatible(lock: &Seq, key: &Seq) -> bool {
    for (lock_elem, key_elem) in zip(lock, key) {
        if lock_elem + key_elem > 6 {
            return false;
        }
    }
    true
}

fn part_one(locks_and_keys: &LocksAndKeys) -> i32 {
    let mut res = 0;
    for (lock, key) in locks_and_keys
        .locks
        .iter()
        .cartesian_product(locks_and_keys.keys.iter())
    {
        if compatible(lock, key) {
            res += 1;
        }
    }
    res
}

fn main() {
    let locks_and_keys = parse_input();
    println!("{}", part_one(&locks_and_keys));
}
