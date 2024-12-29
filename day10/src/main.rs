use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    io,
};

type Grid = Vec<Vec<u32>>;
type Coordinates = (i32, i32);

fn read_grid() -> Grid {
    io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn dims(grid: &Grid) -> (i32, i32) {
    (grid.len() as i32, grid[0].len() as i32)
}

fn inside(grid: &Grid, (row, col): (i32, i32)) -> bool {
    let (rows, cols) = dims(grid);
    row >= 0 && row < rows && col >= 0 && col < cols
}

fn calculate_score_and_rating(grid: &Grid) -> (i32, i32) {
    let mut vals_and_coordinates: Vec<(u32, Coordinates)> = Vec::new();
    let (rows, cols) = dims(grid);
    for (row, col) in (0..rows).cartesian_product(0..cols) {
        vals_and_coordinates.push((grid[row as usize][col as usize], (row, col)));
    }
    vals_and_coordinates.sort();
    vals_and_coordinates.reverse();
    let mut walks: HashMap<Coordinates, (HashSet<Coordinates>, i32)> = HashMap::new();
    let mut sum_zero_scores: i32 = 0;
    let mut sum_zero_ratings: i32 = 0;
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    for (val, (row, col)) in vals_and_coordinates {
        if val == 9 {
            walks.insert((row, col), (HashSet::from([(row, col)]), 1));
        } else {
            walks.insert((row, col), (HashSet::new(), 0));
            for (d_row, d_col) in directions {
                let (neighbour_row, neighbour_col) = (row + d_row, col + d_col);
                if !inside(grid, (neighbour_row, neighbour_col)) {
                    continue;
                }
                if grid[neighbour_row as usize][neighbour_col as usize]
                    == grid[row as usize][col as usize] + 1
                {
                    let neighbour_entry =
                        walks.get(&(neighbour_row, neighbour_col)).unwrap().clone();
                    let entry = walks.entry((row, col)).or_default();
                    entry.0.extend(neighbour_entry.0);
                    entry.1 += neighbour_entry.1;
                }
            }
        }
        if val == 0 {
            let (reachable_nines, rating) = walks.get(&(row, col)).unwrap();
            sum_zero_scores += reachable_nines.len() as i32;
            sum_zero_ratings += rating;
        }
    }
    (sum_zero_scores, sum_zero_ratings)
}

fn main() {
    let grid = read_grid();
    println!("{:?}", calculate_score_and_rating(&grid));
}
