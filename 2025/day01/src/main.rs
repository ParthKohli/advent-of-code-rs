use std::io;

fn parse_delta(line: String) -> i32 {
    let sign = if line.starts_with('L') { -1 } else { 1 };
    let rest = line[1..].parse::<i32>().unwrap();
    sign * rest
}

fn part_1(deltas: Vec<i32>) -> i32 {
    let mut position: i32 = 50;
    let mut res = 0;
    for delta in deltas {
        position += delta;
        if position % 100 == 0 {
            res += 1;
        }
    }
    res
}

fn part_2(deltas: Vec<i32>) -> i32 {
    let mut position: i32 = 50;
    let mut res = 0;
    for delta in deltas {
        let (a, ar) = (position.div_euclid(100), position.rem_euclid(100));
        position += delta;
        let (b, br) = (position.div_euclid(100), position.rem_euclid(100));
        let mut d = (a - b).abs();
        if ar == 0 && delta < 0 && d > 0 {
            d -= 1;
        }
        if br == 0 && delta < 0 {
            d += 1;
        }
        res += d;
    }
    res
}

fn main() {
    let deltas: Vec<_> = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .map(parse_delta)
        .collect();
    println!("{} {}", part_1(deltas.clone()), part_2(deltas));
}
