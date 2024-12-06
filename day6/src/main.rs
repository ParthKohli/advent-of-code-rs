use itertools::Itertools;
use std::{collections::HashSet, io};

type Coordinates = (i32, i32);
type Grid = Vec<Vec<char>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GuardPosition {
    coordinates: Coordinates,
    direction: Direction,
}

enum StepResult {
    Success(GuardPosition),
    FoundObstacle,
    GridEscaped,
}

#[derive(Clone)]
struct GridWalk {
    pos: GuardPosition,
    grid: Grid,
}

fn dims(grid: &Vec<Vec<char>>) -> (usize, usize) {
    (grid.len(), grid[0].len())
}

fn parse_input() -> Option<GridWalk> {
    let grid: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let (rows, cols) = dims(&grid);
    for (row, col) in (0..rows).cartesian_product(0..cols) {
        if let Some(direction) = match grid[row][col] {
            '^' => Some(Direction::Up),
            '>' => Some(Direction::Right),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            _ => None,
        } {
            return Some(GridWalk {
                grid,
                pos: GuardPosition {
                    direction,
                    coordinates: (row as i32, col as i32),
                },
            });
        }
    }

    None
}

impl GridWalk {
    fn step_forward(&self) -> StepResult {
        let pos = self.pos;
        let (coordinates, direction) = (pos.coordinates, pos.direction);
        let coordinates: Coordinates = match direction {
            Direction::Up => (coordinates.0 - 1, coordinates.1),
            Direction::Right => (coordinates.0, coordinates.1 + 1),
            Direction::Down => (coordinates.0 + 1, coordinates.1),
            Direction::Left => (coordinates.0, coordinates.1 - 1),
        };
        let (rows, cols): (i32, i32) = (
            self.grid.len().try_into().unwrap(),
            self.grid[0].len().try_into().unwrap(),
        );
        if coordinates.0 < 0 || coordinates.0 >= rows || coordinates.1 < 0 || coordinates.1 >= cols
        {
            return StepResult::GridEscaped;
        }
        let configuration = GuardPosition {
            direction: pos.direction,
            coordinates,
        };
        match self.grid[coordinates.0 as usize][coordinates.1 as usize] {
            '#' => StepResult::FoundObstacle,
            _ => StepResult::Success(configuration),
        }
    }

    fn turn(&mut self) {
        self.pos.direction = match self.pos.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn part_one(mut grid_walk: GridWalk) -> usize {
    let mut visited_coordinates: HashSet<Coordinates> = HashSet::new();
    loop {
        visited_coordinates.insert(grid_walk.pos.coordinates);
        match grid_walk.step_forward() {
            StepResult::GridEscaped => {
                break;
            }
            StepResult::FoundObstacle => grid_walk.turn(),
            StepResult::Success(new_position) => {
                grid_walk.pos = new_position;
            }
        }
    }
    visited_coordinates.len()
}

fn causes_infinite_loop(mut grid_walk: GridWalk) -> bool {
    let mut visited_positions: HashSet<GuardPosition> = HashSet::new();
    loop {
        if visited_positions.contains(&grid_walk.pos) {
            return true;
        }
        visited_positions.insert(grid_walk.pos);
        match grid_walk.step_forward() {
            StepResult::GridEscaped => {
                return false;
            }
            StepResult::FoundObstacle => grid_walk.turn(),
            StepResult::Success(new_position) => {
                grid_walk.pos = new_position;
            }
        }
    }
}

fn part_two(grid_walk: GridWalk) -> usize {
    let (rows, cols) = dims(&grid_walk.grid);
    let mut jhs = Vec::new();
    for (candidate_row, candidate_col) in (0..rows).cartesian_product(0..cols) {
        if (candidate_row as i32, candidate_col as i32) == grid_walk.pos.coordinates {
            continue;
        }
        let mut grid_walk = grid_walk.clone();
        jhs.push(std::thread::spawn(move || {
            grid_walk.grid[candidate_row][candidate_col] = '#';
            causes_infinite_loop(grid_walk)
        }));
    }
    let mut res = 0;
    for jh in jhs {
        if jh.join().unwrap() {
            res += 1
        }
    }
    res
}

fn main() {
    let grid_walk = parse_input().unwrap();
    println!(
        "{} {}",
        part_one(grid_walk.clone()),
        part_two(grid_walk.clone())
    );
}
