use std::{cmp::max, io};

fn solve(banks: &[Vec<u64>], num_digits: usize) -> u64 {
    let mut res = 0;
    for bank in banks {
        let mut bests: Vec<u64> = vec![0; num_digits + 1];
        for &element in bank.iter().rev() {
            for i in (1..=num_digits).rev() {
                if bests[i - 1] > 0 || i == 1 {
                    bests[i] = max(
                        bests[i],
                        element * (10_u64).pow((i - 1).try_into().unwrap()) + bests[i - 1],
                    );
                }
            }
        }
        res += bests[num_digits]
    }
    res
}

fn parse_banks() -> Vec<Vec<u64>> {
    io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as u64).collect())
        .collect()
}

fn main() {
    let banks = parse_banks();
    println!("{} {}", solve(&banks, 2), solve(&banks, 12),);
}
