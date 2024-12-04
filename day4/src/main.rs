use itertools::Itertools;
use std::io;

fn read_grid() -> Vec<Vec<char>> {
    let lines = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();
    lines
}

fn part_one(grid: Vec<Vec<char>>) -> i32 {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let directions: Vec<(i32, i32)> = vec![
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    let mut res = 0;

    for ((start_row, start_col), &(row_direction, col_direction)) in (0..rows)
        .cartesian_product(0..cols)
        .cartesian_product(directions.iter())
    {
        let word: String = (0..4)
            .map(|step| {
                return (
                    start_row + step * row_direction,
                    start_col + step * col_direction,
                );
            })
            .filter(|&(x, y)| x >= 0 && x < rows && y >= 0 && y < cols)
            .map(&|(x, y)| -> char {
                return grid[x as usize][y as usize];
            })
            .collect();
        if word == "XMAS" {
            res += 1
        }
    }

    res
}

fn main() {
    let grid = read_grid();
    println!("{}", part_one(grid));
}
