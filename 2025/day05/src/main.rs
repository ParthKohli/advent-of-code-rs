use std::cmp::{Eq, Ord};
use std::io;

type Ranges = Vec<(u64, u64)>;
type Queries = Vec<u64>;

#[derive(Eq, PartialEq, PartialOrd, Ord, Copy, Clone)]
enum OpenClose {
    Open,
    Close,
}

fn parse_input() -> (Ranges, Queries) {
    let ranges = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (l, r) = line.split_once('-').unwrap();
            (l.parse::<u64>().unwrap(), r.parse::<u64>().unwrap())
        })
        .collect();

    let queries = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .map(|line| line.parse::<u64>().unwrap())
        .collect();

    (ranges, queries)
}

fn part_one(ranges: &Ranges, queries: &Queries) -> usize {
    let mut res = 0;
    for &query in queries {
        if ranges.iter().any(|&(l, r)| l <= query && query <= r) {
            res += 1;
        }
    }
    res
}

fn part_two(ranges: &Ranges) -> i64 {
    let mut events: Vec<(i64, OpenClose)> = Vec::new();
    for &(l, r) in ranges {
        events.push((l as i64, OpenClose::Open));
        events.push((r as i64, OpenClose::Close));
    }
    events.sort();
    let mut res: i64 = 0;
    let mut last: i64 = events[0].0;
    let mut balance = 0;
    for &(v, kind) in events.iter() {
        if kind == OpenClose::Open {
            if balance == 0 {
                last = v - 1;
            }
            balance += 1;
        }
        res += v - last;
        last = v;
        if kind == OpenClose::Close {
            balance -= 1;
        }
    }
    res
}

fn main() {
    let (ranges, queries) = parse_input();
    println!("{} {}", part_one(&ranges, &queries), part_two(&ranges));
}
