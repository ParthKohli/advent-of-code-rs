use std::io;

type Patterns = Vec<String>;
type Targets = Vec<String>;

fn read_input() -> (Vec<String>, Vec<String>) {
    let patterns = io::stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(", ")
        .map(|v| v.to_string())
        .collect();
    let _empty_line = io::stdin().lines().next();
    let targets = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .collect();

    return (patterns, targets);
}

fn possibilities(patterns: &Vec<String>, target: &String) -> u64 {
    let mut ways = vec![0; target.len() + 1];
    ways[0] = 1;
    for prefix in 0..=target.len() {
        if ways[prefix] == 0 {
            continue;
        }
        for pat in patterns {
            if prefix + pat.len() <= target.len() && target[prefix..prefix + pat.len()] == *pat {
                ways[prefix + pat.len()] += ways[prefix];
            }
        }
    }
    ways[target.len()]
}

fn part_one(patterns: &Patterns, targets: &Targets) -> usize {
    targets
        .iter()
        .filter(|target| possibilities(patterns, target) > 0)
        .count()
}

fn part_two(patterns: &Patterns, targets: &Targets) -> u64 {
    targets
        .iter()
        .map(|target| possibilities(patterns, target))
        .sum()
}

fn main() {
    let (patterns, targets) = read_input();
    println!(
        "{} {}",
        part_one(&patterns, &targets),
        part_two(&patterns, &targets)
    );
}
