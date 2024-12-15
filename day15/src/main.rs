use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
    io,
};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn deltas(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
}

#[derive(Debug)]
struct GridWalk {
    grid: Vec<Vec<char>>,
    walk: Vec<Direction>,
    position: (i32, i32),
}

fn parse_input(scaled: bool) -> GridWalk {
    let grid: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| {
                    if !scaled {
                        c.to_string()
                    } else {
                        String::from(match c {
                            '@' => "@.",
                            '.' => "..",
                            'O' => "[]",
                            '#' => "##",
                            _ => panic!(),
                        })
                    }
                })
                .join("")
                .chars()
                .collect()
        })
        .take_while(|line: &Vec<char>| !line.is_empty())
        .collect();

    let (rows, cols) = (grid.len(), grid[0].len());

    let mut position: Option<(i32, i32)> = None;
    for (row, col) in (0..rows).cartesian_product(0..cols) {
        if grid[row][col] == '@' {
            position = Some((row as i32, col as i32));
        }
    }

    let walk: Vec<String> = io::stdin().lines().map(|line| line.unwrap()).collect();
    let walk: String = walk.concat();
    let walk: Vec<Direction> = walk
        .chars()
        .map(|c| match c {
            '>' => Direction::Right,
            '^' => Direction::Up,
            '<' => Direction::Left,
            'v' => Direction::Down,
            _ => panic!(),
        })
        .collect();

    GridWalk {
        walk,
        grid,
        position: position.unwrap(),
    }
}

fn part_one(mut grid_walk: GridWalk) -> i64 {
    for step in grid_walk.walk.iter() {
        let (x, y) = grid_walk.position;
        let (dx, dy) = step.deltas();
        let (new_x, new_y): (i32, i32) = (grid_walk.position.0 + dx, grid_walk.position.1 + dy);
        let mut moved = false;
        match grid_walk.grid[new_x as usize][new_y as usize] {
            '.' => {
                grid_walk.grid[new_x as usize][new_y as usize] = '@';
                grid_walk.grid[x as usize][y as usize] = '.';
                moved = true;
            }
            '#' => {
                moved = false;
            }
            'O' => {
                let (far_x, far_y) = (0..)
                    .map(|step| (x + step * dx, y + step * dy))
                    .find_or_first(|&(new_x, new_y)| {
                        let c = grid_walk.grid[new_x as usize][new_y as usize];
                        c == '#' || c == '.'
                    })
                    .unwrap();
                match grid_walk.grid[far_x as usize][far_y as usize] {
                    '#' => {}
                    '.' => {
                        grid_walk.grid[x as usize][y as usize] = '.';
                        grid_walk.grid[new_x as usize][new_y as usize] = '@';
                        grid_walk.grid[far_x as usize][far_y as usize] = 'O';
                        moved = true;
                    }
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
        grid_walk.position = match moved {
            true => (new_x, new_y),
            false => (x, y),
        };
    }

    let mut res: i64 = 0;
    let (rows, cols) = (grid_walk.grid.len(), grid_walk.grid[0].len());
    for (row, col) in (0..rows).cartesian_product(0..cols) {
        if grid_walk.grid[row][col] == 'O' {
            let score = row * 100 + col;
            let score = score as i64;
            res += score;
        }
    }
    res
}

fn part_two(mut grid_walk: GridWalk) -> i64 {
    for step in grid_walk.walk.iter() {
        let (x, y) = grid_walk.position;
        let (dx, dy) = step.deltas();
        let (new_x, new_y): (i32, i32) = (grid_walk.position.0 + dx, grid_walk.position.1 + dy);
        match grid_walk.grid[new_x as usize][new_y as usize] {
            '.' => {
                grid_walk.position = (new_x, new_y);
                grid_walk.grid[new_x as usize][new_y as usize] = '@';
                grid_walk.grid[x as usize][y as usize] = '.';
            }
            '#' => {}
            '[' | ']' => {
                let mut queue: VecDeque<(i32, i32)> = Default::default();
                let mut added: HashSet<(i32, i32)> = Default::default();

                queue.push_back((x, y));
                added.insert((x, y));
                let mut is_blocked = false;
                while !queue.is_empty() {
                    let (x, y) = queue.pop_front().unwrap();
                    let (next_x, next_y) = (x + dx, y + dy);
                    let c = grid_walk.grid[x as usize][y as usize];
                    if c == '[' || c == ']' {
                        if let Some((neigh_x, neigh_y)) = match c {
                            '[' => Some((x, y + 1)),
                            ']' => Some((x, y - 1)),
                            _ => None,
                        } {
                            if !added.contains(&(neigh_x, neigh_y)) {
                                queue.push_front((neigh_x, neigh_y));
                                added.insert((neigh_x, neigh_y));
                            }
                        }
                    }
                    let next_c = grid_walk.grid[next_x as usize][next_y as usize];
                    match next_c {
                        '#' => {
                            is_blocked = true;
                            break;
                        }
                        '.' => {
                            continue;
                        }
                        _ => {}
                    }
                    if !added.contains(&(next_x, next_y)) {
                        queue.push_back((next_x, next_y));
                        added.insert((next_x, next_y));
                    }
                }
                if is_blocked {
                    continue;
                }
                let mut value_cache: HashMap<(i32, i32), char> = Default::default();
                for &(x, y) in added.iter() {
                    value_cache.insert((x, y), grid_walk.grid[x as usize][y as usize]);
                    grid_walk.grid[x as usize][y as usize] = '.';
                }
                for &(x, y) in added.iter() {
                    let c = *value_cache.get(&(x, y)).unwrap();
                    let (x, y) = (x + dx, y + dy);
                    grid_walk.grid[x as usize][y as usize] = c;
                }
                grid_walk.position = (grid_walk.position.0 + dx, grid_walk.position.1 + dy);
            }
            _ => panic!(),
        }
    }
    let mut res: i64 = 0;
    let (rows, cols) = (grid_walk.grid.len(), grid_walk.grid[0].len());
    for (row, col) in (0..rows).cartesian_product(0..cols) {
        if grid_walk.grid[row][col] == '[' {
            let score = row * 100 + col;
            let score = score as i64;
            res += score;
        }
    }
    res
}

fn main() {
    let grid_walk = parse_input(true);
    let scaled = true;
    if !scaled {
        println!("{}", part_one(grid_walk));
    } else {
        println!("{}", part_two(grid_walk));
    }
}
