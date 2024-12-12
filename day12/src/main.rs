use core::num;
use itertools::Itertools;
use std::{
    collections::{BTreeSet, HashSet, VecDeque},
    hash::Hash,
    io,
};

struct Grid {
    raw_grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

type Cell = (i32, i32);

#[derive(Default)]
struct RegionSpec {
    area: u64,
    perimeter: u64,
    fence_cells: BTreeSet<(i32, i32)>,
    region_cells: BTreeSet<Cell>,
}

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

fn neighbours(coordinates: (i32, i32), scaled: bool) -> Vec<(i32, i32)> {
    match scaled {
        true => DIRECTIONS
            .iter()
            .map(|(x, y)| (2 * x, 2 * y))
            .map(|(dx, dy)| (coordinates.0 + dx, coordinates.1 + dy))
            .collect(),
        false => DIRECTIONS
            .iter()
            .map(|(dx, dy)| (coordinates.0 + dx, coordinates.1 + dy))
            .collect(),
    }
}

impl RegionSpec {
    fn part_one_price(&self) -> u64 {
        self.area * self.perimeter
    }

    fn num_sides(&self) -> u64 {
        let mut visited_fences: BTreeSet<(i32, i32)> = BTreeSet::new();
        let mut num_sides = 0;
        for fence_cell in self.fence_cells.iter() {
            if visited_fences.contains(fence_cell) {
                continue;
            }
            num_sides += 1;
            let mut side_fences: Vec<(i32, i32)> = Vec::new();
            let mut fence_queue: VecDeque<(i32, i32)> = VecDeque::from([*fence_cell]);
            while !fence_queue.is_empty() {
                let fence = fence_queue.pop_front().unwrap();
                if visited_fences.contains(&fence) {
                    continue;
                }
                visited_fences.insert(fence);
                side_fences.push(fence);
                let (fence_row, fence_col) = fence;
                for (d_row, d_col) in DIRECTIONS {
                    let (d_row, d_col) = (d_row * 2, d_col * 2);
                    let parity_compatible = (d_col == 0) == (fence_row % 2 == 0);
                    if !parity_compatible {
                        continue;
                    }
                    let next_row: i32 = fence_row + d_row;
                    let next_col: i32 = fence_col + d_col;

                    let current_region_neighbours: HashSet<(i32, i32)> =
                        neighbours((fence_row, fence_col), false)
                            .into_iter()
                            .filter(|neighbour| self.region_cells.contains(&neighbour))
                            .collect::<HashSet<_>>();

                    let next_region_neighbours: HashSet<(i32, i32)> =
                        neighbours((next_row, next_col), false)
                            .into_iter()
                            .filter(|neighbour| self.region_cells.contains(&neighbour))
                            .collect::<HashSet<_>>();

                    let mut region_compatible = false;
                    for current_region_neighbour in current_region_neighbours {
                        let second_degree_neighbours = neighbours(current_region_neighbour, true);
                        let second_degree_neighbours: HashSet<(i32, i32)> =
                            HashSet::from_iter(second_degree_neighbours);
                        if !next_region_neighbours.is_disjoint(&second_degree_neighbours) {
                            region_compatible = true;
                        }
                    }
                    if !region_compatible {
                        continue;
                    }

                    if self.fence_cells.contains(&(next_row, next_col)) {
                        fence_queue.push_back((next_row, next_col));
                    }
                }
            }
        }
        num_sides
    }

    fn part_two_price(&self) -> u64 {
        self.num_sides() * self.area
    }
}

fn read_input() -> Grid {
    let raw_grid: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.chars().collect())
        .collect();

    let (rows, cols) = (raw_grid.len(), raw_grid[0].len());
    Grid {
        raw_grid,
        rows,
        cols,
    }
}

fn calculate_prices(grid: &Grid) -> (u64, u64) {
    let mut visited: BTreeSet<Cell> = BTreeSet::new();
    let (rows, cols) = (grid.rows, grid.cols);
    let mut part_one_price = 0;
    let mut part_two_price = 0;
    for (row, col) in (0..rows).cartesian_product(0..cols) {
        let (row, col) = (row as i32, col as i32);
        if visited.contains(&(row, col)) {
            continue;
        }
        let region_char = grid.raw_grid[row as usize][col as usize];
        let mut region_queue: VecDeque<Cell> = VecDeque::from([(row, col)]);
        let mut region_spec: RegionSpec = Default::default();
        while !region_queue.is_empty() {
            let cell = region_queue.pop_front().unwrap();
            if visited.contains(&cell) {
                continue;
            }
            visited.insert(cell);
            region_spec.area += 1;
            let (cell_row, cell_col) = cell;
            region_spec
                .region_cells
                .insert((2 * cell_row, 2 * cell_col));
            for (d_row, d_col) in DIRECTIONS {
                let next_row: i32 = cell_row as i32 + d_row;
                let next_col: i32 = cell_col as i32 + d_col;
                if next_row >= 0
                    && next_row < rows as i32
                    && next_col >= 0
                    && next_col < cols as i32
                    && grid.raw_grid[next_row as usize][next_col as usize] == region_char
                {
                    region_queue.push_back((next_row, next_col));
                } else {
                    region_spec.perimeter += 1;
                    let fence_cell: (i32, i32) =
                        (cell_row as i32 * 2 + d_row, cell_col as i32 * 2 + d_col);
                    region_spec.fence_cells.insert(fence_cell);
                }
            }
        }
        part_one_price += region_spec.part_one_price();
        part_two_price += region_spec.part_two_price();
    }
    (part_one_price, part_two_price)
}

fn main() {
    let grid = read_input();
    println!("{:?}", calculate_prices(&grid));
}
