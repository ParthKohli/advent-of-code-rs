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

fn part_one(grid: &[Vec<char>]) -> i32 {
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
                (
                    start_row + step * row_direction,
                    start_col + step * col_direction,
                )
            })
            .filter(|&(x, y)| x >= 0 && x < rows && y >= 0 && y < cols)
            .map(&|(x, y)| -> char { grid[x as usize][y as usize] })
            .collect();
        if word == "XMAS" {
            res += 1
        }
    }

    res
}

fn is_direction_valid(grid: &[Vec<char>], (row, col): (i32, i32), direction: (i32, i32)) -> bool {
    let (row_direction, col_direction) = direction;

    let word: String = (-1..=1)
        .map(|step| (row + step * row_direction, col + step * col_direction))
        .map(|(cell_x, cell_y)| grid[cell_x as usize][cell_y as usize])
        .collect();

    word == "MAS" || word == "SAM"
}

fn part_two(grid: &[Vec<char>]) -> i32 {
    let rows = grid.len() as i32;
    let cols = grid.len() as i32;

    let directions = [(-1, 1), (1, 1)]; // The two perpendicular directions for the X
    let mut res = 0;
    for (middle_row, middle_col) in (1..(rows - 1)).cartesian_product(1..(cols - 1)) {
        if directions
            .iter()
            .all(|&direction| is_direction_valid(grid, (middle_row, middle_col), direction))
        {
            res += 1
        }
    }

    res
}

fn main() {
    let grid = read_grid();
    println!("{} {}", part_one(&grid), part_two(&grid));
}
