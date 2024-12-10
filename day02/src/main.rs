use itertools::Itertools;
use std::io;

fn read_input() -> Vec<Vec<i32>> {
    return io::stdin()
        .lines()
        .map(|x| x.unwrap())
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.split_whitespace()
                .map(|i| i.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
}

fn is_safe_report(row: Vec<i32>) -> bool {
    if !row.is_sorted() && !row.iter().rev().is_sorted() {
        return false;
    }
    for (left, right) in row.iter().tuple_windows() {
        let diff = (left - right).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    return true;
}

fn part_one(grid: Vec<Vec<i32>>) -> usize {
    return grid
        .iter()
        .filter(|row| is_safe_report(row.to_vec()))
        .count();
}

fn part_two(grid: Vec<Vec<i32>>) -> usize {
    grid.iter()
        .filter(|row| {
            is_safe_report(row.to_vec())
                || row.iter().enumerate().any(|(i, _)| {
                    is_safe_report(
                        row[..i]
                            .iter()
                            .chain(&row[(i + 1)..])
                            .cloned()
                            .collect::<Vec<_>>(),
                    )
                })
        })
        .count()
}

fn main() {
    let input = read_input();
    println!("{} {}", part_one(input.clone()), part_two(input.clone()));
}
