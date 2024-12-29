use gcd::Gcd;
use itertools::Itertools;
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::io;

type Grid = Vec<Vec<char>>;
type Coordinate = (i32, i32);

fn dims(grid: &Grid) -> (usize, usize) {
    (grid.len(), grid[0].len())
}

fn parse_input() -> Grid {
    io::stdin()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .filter(|vec: &Vec<char>| !vec.is_empty())
        .collect()
}

fn coordinates_by_signals(grid: &Grid) -> HashMap<char, Vec<Coordinate>> {
    let mut coordinates_for_signal: HashMap<char, Vec<Coordinate>> = HashMap::new();
    let (rows, cols) = dims(grid);
    for (row, col) in (0..rows).cartesian_product(0..cols) {
        if grid[row][col] == '.' {
            continue;
        }
        (*coordinates_for_signal.entry(grid[row][col]).or_default()).push((row as i32, col as i32));
    }
    coordinates_for_signal
}

fn part_one(grid: &Grid) -> usize {
    let mut unique_antinodes: HashSet<Coordinate> = HashSet::new();
    let (rows, cols) = dims(grid);
    let coordinates_for_signal = coordinates_by_signals(grid);
    let mut considered_pairs = 0;
    for (_, coordinates) in coordinates_for_signal.into_iter() {
        for (c1, c2) in coordinates.iter().cartesian_product(coordinates.iter()) {
            if c1 >= c2 {
                continue;
            }
            considered_pairs += 1;
            let dx = c2.0 - c1.0;
            let dy = c2.1 - c1.1;
            let candidates = [(c1.0 - dx, c1.1 - dy), (c2.0 + dx, c2.1 + dy)];
            for (cx, cy) in candidates {
                if cx >= 0 && cx < rows as i32 && cy >= 0 && cy < cols as i32 {
                    unique_antinodes.insert((cx, cy));
                }
            }
        }
    }
    dbg!(considered_pairs, rows, cols);
    unique_antinodes.len()
}

fn part_two(grid: &Grid) -> usize {
    let mut unique_antinodes: HashSet<Coordinate> = HashSet::new();
    let (rows, cols) = dims(grid);
    let coordinates_for_signal = coordinates_by_signals(grid);
    for (_, coordinates) in coordinates_for_signal.into_iter() {
        for (c1, c2) in coordinates.iter().cartesian_product(coordinates.iter()) {
            if c1 >= c2 {
                continue;
            }
            let dx = c2.0 - c1.0;
            let dy = c2.1 - c1.1;
            let gcd = dx.unsigned_abs().gcd(dy.unsigned_abs()) as i32;
            let dx = dx / gcd;
            let dy = dy / gcd;
            let dim = max(rows, cols) as i32;
            let found_antinodes = (-dim..=dim)
                .map(|step| (c1.0 + step * dx, c1.1 + step * dy))
                .filter(|&(cx, cy)| cx >= 0 && cx < rows as i32 && cy >= 0 && cy < cols as i32);
            for antinode in found_antinodes {
                unique_antinodes.insert(antinode);
            }
        }
    }
    unique_antinodes.len()
}

fn main() {
    let grid = parse_input();
    println!("{} {}", part_one(&grid), part_two(&grid));
}
