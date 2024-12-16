use itertools::Itertools;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    hash::Hash,
    io,
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

type Cell = (usize, usize);

struct ShortestPaths {
    grid: Vec<Vec<char>>,
    start: Cell,
    end: Cell,
}

impl ShortestPaths {
    fn calculate(
        &self,
        source: Cell,
        initial_direction: Direction,
    ) -> HashMap<(Cell, Direction), u64> {
        let mut distance_heap = BinaryHeap::new();
        let mut cell_direction_distance: HashMap<(Cell, Direction), u64> = HashMap::new();

        distance_heap.push(Reverse((0 as u64, source, initial_direction)));

        while let Some(Reverse(cheapest)) = distance_heap.pop() {
            let (distance, (row, col), direction) = cheapest;
            if cell_direction_distance.contains_key(&((row, col), direction)) {
                continue;
            }
            cell_direction_distance.insert(((row, col), direction), distance);
            for next_direction in Direction::iter() {
                if direction.opp() == next_direction {
                    continue;
                }
                let ((next_row, next_col), cost) = if next_direction != direction {
                    ((row, col), 1000)
                } else {
                    (
                        match next_direction {
                            Direction::Up => (row - 1, col),
                            Direction::Right => (row, col + 1),
                            Direction::Left => (row, col - 1),
                            Direction::Down => (row + 1, col),
                        },
                        1,
                    )
                };
                if self.grid[next_row][next_col] == '#' {
                    continue;
                }
                distance_heap.push(Reverse((
                    distance + cost,
                    (next_row, next_col),
                    next_direction,
                )));
            }
        }

        cell_direction_distance
    }
}

#[derive(Eq, PartialEq, Hash, PartialOrd, Ord, EnumIter, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn opp(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
        }
    }
}

fn parse_input() -> ShortestPaths {
    let grid: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(io::Result::unwrap)
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let (rows, cols) = (grid.len(), grid[0].len());

    let mut start: Option<Cell> = None;
    let mut end: Option<Cell> = None;
    for (row, col) in (0..rows).cartesian_product(0..cols) {
        if grid[row][col] == 'S' {
            start = Some((row, col));
        } else if grid[row][col] == 'E' {
            end = Some((row, col));
        }
    }

    ShortestPaths {
        grid,
        start: start.unwrap(),
        end: end.unwrap(),
    }
}

fn part_one(grid: &ShortestPaths, source: Cell, destination: Cell) -> Option<(u64, Direction)> {
    let distances = grid.calculate(source, Direction::Right);
    Direction::iter()
        .map(|direction| (distances.get(&(destination, direction)), direction))
        .filter_map(|(dist, dir)| match dist {
            None => None,
            Some(dist) => Some((*dist, dir)),
        })
        .min()
}

fn part_two(grid: &ShortestPaths) -> usize {
    let (start_to_end, final_direction) = part_one(&grid, grid.start, grid.end).unwrap();
    let start_distances = grid.calculate(grid.start, Direction::Right);
    let end_distances = grid.calculate(grid.end, final_direction.opp());
    let mut res = 0;
    let (rows, cols) = (grid.grid.len(), grid.grid[0].len());
    for (row, col) in (0..rows as usize).cartesian_product(0..cols as usize) {
        if grid.grid[row][col] != '#' {
            let mut is_good_tile = false;
            for direction in Direction::iter() {
                if let Some(start_to_cell) = start_distances.get(&((row, col), direction)) {
                    if let Some(cell_to_end) = end_distances.get(&((row, col), direction.opp())) {
                        if start_to_cell + cell_to_end == start_to_end {
                            is_good_tile = true;
                        }
                    }
                }
            }
            if is_good_tile {
                res += 1;
            }
        }
    }
    res
}

fn main() {
    let grid = parse_input();
    // Part 1:
    println!("{}", part_one(&grid, grid.start, grid.end).unwrap().0);
    // Part 2:
    println!("{}", part_two(&grid)); // Part 2
}
