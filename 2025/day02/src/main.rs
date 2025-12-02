use std::io;

fn parse_ranges() -> Vec<(u64, u64)> {
    let line = io::stdin().lines().next().unwrap().unwrap();
    let ranges = line
        .split(',')
        .map(|part| {
            let mut bounds = part.split('-').map(|n| n.parse::<u64>().unwrap());
            let start = bounds.next().unwrap();
            let end = bounds.next().unwrap();
            (start, end)
        })
        .collect::<Vec<(u64, u64)>>();
    ranges
}

fn is_periodic(v: &str, d: usize) -> bool {
    let l = v.len();
    if d == 0 || l % d != 0 {
        return false;
    }
    for start in (d..v.len()).step_by(d) {
        if v[start..start + d] != v[0..d] {
            return false;
        }
    }
    true
}

fn solve<F>(ranges: &[(u64, u64)], predicate: F) -> u64
where
    F: Fn(u64) -> bool,
{
    let mut res = 0;
    for &(l, r) in ranges {
        res += (l..=r).filter(|&x| predicate(x)).sum::<u64>();
    }
    res
}

fn part_one(ranges: &[(u64, u64)]) -> u64 {
    solve(ranges, |x| {
        let s = x.to_string();
        s.len() % 2 == 0 && is_periodic(&s, s.len() / 2)
    })
}

fn part_two(ranges: &[(u64, u64)]) -> u64 {
    solve(ranges, |x| {
        let s = x.to_string();
        (1..s.len()).any(|d| is_periodic(&s, d))
    })
}

fn main() {
    let ranges = parse_ranges();
    println!("{} {}", part_one(&ranges), part_two(&ranges));
}
