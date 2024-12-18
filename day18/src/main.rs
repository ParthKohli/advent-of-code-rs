use itertools::Itertools;
use std::{
    collections::{HashSet, VecDeque},
    io,
};

type Point = (i32, i32);

fn read_input() -> Vec<Point> {
    io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            line.split(',')
                .map(|token| token.parse().unwrap())
                .collect_tuple::<Point>()
                .unwrap()
        })
        .collect()
}

fn distance(banned: &[Point], rows: i32, cols: i32) -> Option<i32> {
    let mut banned_set: HashSet<Point> = HashSet::new();
    for &banned_point in banned {
        banned_set.insert(banned_point);
    }

    let directions = [(-1, 0), (0, 1), (0, -1), (1, 0)];
    let mut queue: VecDeque<((i32, i32), i32)> = VecDeque::from([((0, 0), 0)]);
    let mut visited: HashSet<Point> = HashSet::new();

    let mut bottom_right_dist = None;
    while let Some(((x, y), d)) = queue.pop_front() {
        if visited.contains(&(x, y)) {
            continue;
        }
        if (x, y) == (rows - 1, cols - 1) {
            bottom_right_dist = Some(d);
        }
        visited.insert((x, y));
        for (dx, dy) in directions {
            let (nx, ny) = (x + dx, y + dy);
            if nx >= 0 && nx < rows && ny >= 0 && ny < cols && !banned_set.contains(&(nx, ny)) {
                queue.push_back(((nx, ny), d + 1));
            }
        }
    }
    bottom_right_dist
}

fn part_two(whole_banned: &[Point], rows: i32, cols: i32) -> Point {
    let mut lo = 0;
    let mut hi = whole_banned.len() - 1;
    let mut point: Option<Point> = None;
    while lo <= hi {
        let mid = (lo + hi) / 2;
        if let Some(_d) = distance(&whole_banned[0..=mid], rows, cols) {
            lo = mid + 1;
        } else {
            point = Some(whole_banned[mid]);
            hi = mid - 1;
        }
    }
    point.unwrap()
}

fn main() {
    let coordinates = read_input();
    let (rows, cols) = (71, 71);
    let prefix_size = 1024;
    println!(
        "{:?}",
        distance(&coordinates[..prefix_size], rows, cols).unwrap()
    );
    println!("{:?}", part_two(&coordinates[..], rows, cols));
}
