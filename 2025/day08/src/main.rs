use std::{cmp::min, io};

type Point = (i64, i64, i64);
const NUM_MERGES: usize = 1000;

fn parse_points() -> Vec<Point> {
    io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split(',').map(|part| part.parse::<i64>().unwrap());
            (
                parts.next().unwrap(),
                parts.next().unwrap(),
                parts.next().unwrap(),
            )
        })
        .collect()
}

fn solve(points: Vec<Point>) -> (u64, i64) {
    let mut comp: Vec<usize> = (0..points.len()).collect();
    let mut dists = Vec::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let (x, y, z) = (
                points[i].0 - points[j].0,
                points[i].1 - points[j].1,
                points[i].2 - points[j].2,
            );
            dists.push((x * x + y * y + z * z, i, j));
        }
    }
    dists.sort();
    let mut part_one_res = 0;
    let mut part_two_res = 0;
    for (idx, &(_, i, j)) in dists.iter().enumerate() {
        let (old_i, old_j) = (comp[i], comp[j]);
        if old_i != old_j {
            for cv in comp.iter_mut() {
                if *cv == old_i || *cv == old_j {
                    *cv = min(old_i, old_j);
                }
            }
        }
        if idx == NUM_MERGES - 1 {
            let mut sizes: Vec<u64> = Vec::new();
            for v in 0..points.len() {
                if comp[v] == v {
                    let comp_size = comp.iter().filter(|&c| *c == v).count() as u64;
                    sizes.push(comp_size);
                }
            }
            sizes.sort();
            part_one_res = sizes[sizes.len() - 1] * sizes[sizes.len() - 2] * sizes[sizes.len() - 3]
        }
        if part_two_res == 0 && comp.iter().all(|v| *v == 0) {
            part_two_res = points[i].0 * points[j].0;
        }
    }
    (part_one_res, part_two_res)
}

fn main() {
    let points = parse_points();
    println!("{:?}", solve(points));
}
