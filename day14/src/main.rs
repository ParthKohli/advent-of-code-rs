use std::{
    collections::{BTreeSet, HashMap, VecDeque},
    io,
};

use regex::Regex;

const DIMS: (i64, i64) = (101, 103);
const ELAPSED_TIME: i64 = 100;

#[derive(Default)]
struct Grid {
    dims: (i64, i64),
    robots: Vec<Robot>,
}

struct Robot {
    position: (i64, i64),
    velocity: (i64, i64),
    dims: (i64, i64),
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Robot {
    fn future_position(&self, elapsed_time: i64) -> (i64, i64) {
        let x = self.position.0 + self.velocity.0 * elapsed_time;
        let y = self.position.1 + self.velocity.1 * elapsed_time;

        let (x, y) = (x % self.dims.0, y % self.dims.1);
        let (x, y) = (
            match x < 0 {
                true => x + self.dims.0,
                false => x,
            },
            match y < 0 {
                true => y + self.dims.1,
                false => y,
            },
        );
        (x, y)
    }
}

fn read_input(dims: (i64, i64)) -> Grid {
    let robots: Vec<Robot> = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
            if let Some(captures) = re.captures(&line) {
                let (x, y): (i64, i64) =
                    (captures[1].parse().unwrap(), captures[2].parse().unwrap());
                let (dx, dy): (i64, i64) =
                    (captures[3].parse().unwrap(), captures[4].parse().unwrap());
                Robot {
                    position: (x, y),
                    velocity: (dx, dy),
                    dims,
                }
            } else {
                panic!()
            }
        })
        .collect();
    Grid { robots, dims }
}

fn draw(grid: &Grid, elapsed_time: i64) {
    let dims = grid.dims;
    let mut raw_grid: Vec<Vec<char>> = vec![vec!['.'; dims.0 as usize]; dims.1 as usize];
    println!("{elapsed_time}");
    for robot in grid.robots.iter() {
        let (x, y) = robot.future_position(elapsed_time);
        raw_grid[y as usize][x as usize] = '#';
    }
    for row in raw_grid {
        let row_str: String = row.iter().collect();
        println!("{}", row_str);
    }
}

fn check(grid: &Grid, elapsed_time: i64) -> bool {
    let mut positions: BTreeSet<(i64, i64)> = BTreeSet::new();
    for robot in grid.robots.iter() {
        let (x, y) = robot.future_position(elapsed_time);
        positions.insert((x, y));
    }
    let directions = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    while !positions.is_empty() {
        let mut component_size = 0;
        let mut component_queue: VecDeque<(i64, i64)> = VecDeque::new();
        component_queue.push_back(*positions.last().unwrap());
        positions.remove(component_queue.back().unwrap());
        loop {
            if component_queue.is_empty() {
                break;
            }
            let (x, y) = component_queue.pop_back().unwrap();
            component_size += 1;
            for (dx, dy) in directions {
                let next_cell = (x + dx, y + dy);
                if !positions.contains(&next_cell) {
                    continue;
                }
                component_queue.push_back(next_cell);
                positions.remove(&next_cell);
            }
        }
        if component_size >= 30 {
            return true;
        }
    }
    false
}

fn part_one(grid: &Grid) -> i64 {
    let mut quadrant_counts: HashMap<Quadrant, i64> = HashMap::from([
        (Quadrant::TopLeft, 0),
        (Quadrant::TopRight, 0),
        (Quadrant::BottomLeft, 0),
        (Quadrant::BottomRight, 0),
    ]);
    for robot in grid.robots.iter() {
        let (rx, ry) = robot.future_position(ELAPSED_TIME);
        if 2 * rx == (DIMS.0 - 1) || 2 * ry == (DIMS.1 - 1) {
            continue;
        }
        let quadrant = match (rx < (DIMS.0 - 1) / 2, ry < (DIMS.1 - 1) / 2) {
            (true, true) => Quadrant::TopLeft,
            (false, true) => Quadrant::TopRight,
            (true, false) => Quadrant::BottomLeft,
            (false, false) => Quadrant::BottomRight,
        };
        *quadrant_counts.entry(quadrant).or_insert(0) += 1;
    }
    quadrant_counts.values().product()
}

fn part_two(grid: &Grid) {
    let mut elapsed_time = 0;
    loop {
        elapsed_time += 1;
        if check(grid, elapsed_time) {
            draw(grid, elapsed_time);
            break;
        }
    }
}

fn main() {
    let grid = read_input(DIMS);

    println!("{}", part_one(&grid));
    part_two(&grid);
}
