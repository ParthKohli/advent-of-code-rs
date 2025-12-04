use itertools::Itertools;
use std::{
    collections::{HashSet, VecDeque},
    io,
};

fn parse_grid() -> Vec<Vec<char>> {
    io::stdin()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .filter(|line: &Vec<char>| !line.is_empty())
        .collect()
}

fn neighbours(i: i32, j: i32, grid: &Vec<Vec<char>>) -> Vec<(i32, i32)> {
    let (rows, cols): (i32, i32) = (grid.len() as i32, grid[0].len() as i32);
    (-1..=1)
        .cartesian_product(-1..=1)
        .map(|(dx, dy)| (i + dx, j + dy))
        .filter(|&(i2, j2)| i2 >= 0 && i2 < rows && j2 >= 0 && j2 < cols && (i2, j2) != (i, j))
        .collect()
}

fn is_removable(i: i32, j: i32, grid: &Vec<Vec<char>>) -> bool {
    if grid[i as usize][j as usize] != '@' {
        return false;
    }
    neighbours(i, j, grid)
        .into_iter()
        .filter(|&(i, j)| grid[i as usize][j as usize] == '@')
        .count()
        < 4
}

fn part_one(grid: &Vec<Vec<char>>) -> usize {
    let (rows, cols): (i32, i32) = (grid.len() as i32, grid[0].len() as i32);
    let mut res: usize = 0;
    for (i, j) in (0..rows).cartesian_product(0..cols) {
        if grid[i as usize][j as usize] != '@' {
            continue;
        }
        if is_removable(i, j, grid) {
            res += 1;
        }
    }
    res
}

fn part_two(grid: &mut Vec<Vec<char>>) -> usize {
    let (rows, cols): (i32, i32) = (grid.len() as i32, grid[0].len() as i32);
    let mut q: VecDeque<(i32, i32)> = VecDeque::new();
    let mut added: HashSet<(i32, i32)> = HashSet::new();
    for (i, j) in (0..rows).cartesian_product(0..cols) {
        if is_removable(i, j, grid) {
            q.push_back((i, j));
            added.insert((i, j));
        }
    }
    while let Some((i, j)) = q.pop_front() {
        grid[i as usize][j as usize] = '.';
        for (i2, j2) in neighbours(i, j, grid) {
            if is_removable(i2, j2, grid) && !added.contains(&(i2, j2)) {
                q.push_back((i2, j2));
                added.insert((i2, j2));
            }
        }
    }
    added.len()
}

fn main() {
    let mut grid = parse_grid();
    println!("{} {}", part_one(&grid), part_two(&mut grid));
}
