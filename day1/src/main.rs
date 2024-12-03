use std::collections::HashMap;
use std::io;
use std::iter::zip;

fn read_input() -> (Vec<i64>, Vec<i64>) {
    let lines = io::stdin().lines();

    let mut a: Vec<i64> = Vec::new();
    let mut b: Vec<i64> = Vec::new();

    for line in lines {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        let mut ints = line.split_whitespace().map(|x| {
            return x.parse::<i64>().unwrap();
        });
        a.push(ints.next().unwrap());
        b.push(ints.next().unwrap());
    }

    return (a, b);
}

fn part_one(a: &Vec<i64>, b: &Vec<i64>) -> i64 {
    let mut a: Vec<i64> = a.clone();
    a.sort();
    let mut b: Vec<i64> = b.clone();
    b.sort();
    let res: i64 = zip(a, b).map(|(a, b)| (a - b).abs()).sum();
    return res;
}

fn part_two(a: &Vec<i64>, b: &Vec<i64>) -> i64 {
    let mut counts: HashMap<i64, i64> = HashMap::new();
    for element in b {
        *counts.entry(*element).or_insert(0) += 1
    }
    return a
        .iter()
        .map(|x| counts.get(x).cloned().unwrap_or(0) * x)
        .sum();
}

fn main() {
    let (a, b) = read_input();

    println!("{} {}", part_one(&a, &b), part_two(&a, &b));
}
