use itertools::Itertools;
use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    io,
};

type Point = (i32, i32);

#[derive(Debug)]
struct Dims {
    rows: i32,
    cols: i32,
}

struct Grid {
    raw_grid: Vec<Vec<char>>,
    start: Point,
    end: Point,
    dims: Dims,
}

fn parse() -> Grid {
    let mut raw_grid: Vec<Vec<char>> = Vec::new();
    for line in io::stdin().lines() {
        let line = line.unwrap();
        if !line.is_empty() {
            raw_grid.push(line.chars().collect());
        }
    }
    let (rows, cols) = (raw_grid[0].len(), raw_grid[1].len());
    let mut start: Option<Point> = None;
    let mut end: Option<Point> = None;
    for (row, col) in (0..rows as i32).cartesian_product(0..cols as i32) {
        match raw_grid[row as usize][col as usize] {
            'S' => start = Some((row, col)),
            'E' => end = Some((row, col)),
            _ => {}
        }
    }
    let start = start.unwrap();
    let end = end.unwrap();
    Grid {
        raw_grid,
        start,
        end,
        dims: Dims {
            rows: rows as i32,
            cols: cols as i32,
        },
    }
}

fn bfs(grid: &Grid, source: Point) -> HashMap<Point, i32> {
    let mut queue: VecDeque<(Point, i32)> = VecDeque::new();
    let mut distances: HashMap<Point, i32> = HashMap::new();

    queue.push_back((source, 0));
    let directions = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    while let Some(((row, col), d)) = queue.pop_front() {
        if distances.contains_key(&(row, col)) {
            continue;
        }
        distances.insert((row, col), d);
        for (d_row, d_col) in directions {
            let (next_row, next_col) = (row + d_row, col + d_col);
            if !(next_row >= 0
                && next_row < grid.dims.rows
                && next_col >= 0
                && next_col < grid.dims.cols)
            {
                continue;
            }
            let next_cell = grid.raw_grid[next_row as usize][next_col as usize];
            if next_cell != '#' {
                queue.push_back(((next_row, next_col), d + 1));
            }
        }
    }
    distances
}

fn find_shortcuts(grid: &Grid, middle_path_len: i32) -> i32 {
    let start_distances = bfs(grid, grid.start);
    let end_distances = bfs(grid, grid.end);
    let old_distance = start_distances[&grid.end];
    let mut saved_distances_count: BTreeMap<i32, i32> = BTreeMap::new();
    for (start_row, start_col) in (0..grid.dims.rows).cartesian_product(0..grid.dims.cols) {
        for (end_row, end_col) in (start_row - middle_path_len..=start_row + middle_path_len)
            .cartesian_product(start_col - middle_path_len..=start_col + middle_path_len)
        {
            if start_distances.contains_key(&(start_row, start_col))
                && end_distances.contains_key(&(end_row, end_col))
            {
                let manhattan = (end_row - start_row).abs() + (end_col - start_col).abs();
                if manhattan <= middle_path_len {
                    let new_distance = manhattan
                        + start_distances[&(start_row, start_col)]
                        + end_distances[&(end_row, end_col)];
                    *saved_distances_count
                        .entry(old_distance - new_distance)
                        .or_insert(0) += 1;
                }
            }
        }
    }
    let mut res = 0;
    for (saved_distance, count) in saved_distances_count.into_iter() {
        if saved_distance >= 100 {
            res += count;
        }
    }
    res
}

fn main() {
    let grid = parse();
    println!("{} {}", find_shortcuts(&grid, 2), find_shortcuts(&grid, 20));
}
