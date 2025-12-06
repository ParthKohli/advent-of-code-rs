use std::io;

#[derive(Copy, Clone, Debug)]
enum Op {
    Plus,
    Mul,
}

fn parse_input() -> Vec<String> {
    io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect()
}

fn part_one(lines: &[String]) -> u64 {
    let nums: Vec<Vec<u64>> = lines[..lines.len() - 1]
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|part| part.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    let ops: Vec<Op> = lines
        .last()
        .and_then(|line| {
            Some(
                line.split_whitespace()
                    .map(|part| match part {
                        "*" => Op::Mul,
                        "+" => Op::Plus,
                        _ => panic!(),
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .unwrap();

    let mut res = 0;
    for (idx, &op) in ops.iter().enumerate() {
        let col_nums: Vec<u64> = (0..nums.len()).map(|i| nums[i][idx]).collect();
        res += col_nums
            .into_iter()
            .reduce(|acc, x| match op {
                Op::Plus => acc + x,
                Op::Mul => acc * x,
            })
            .unwrap();
    }
    res
}

fn part_two(lines: &[String]) -> u64 {
    if lines.is_empty() {
        return 0;
    }
    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let mut total = 0;
    let mut cols: Vec<Vec<char>> = vec![vec![' '; lines.len()]; max_len];
    for (row_idx, line) in lines.iter().enumerate() {
        for (col_idx, ch) in line.chars().enumerate() {
            cols[col_idx][row_idx] = ch;
        }
    }
    let mut current_block: Vec<Vec<char>> = Vec::new();
    for col_idx in (0..max_len).rev() {
        let col = &cols[col_idx];
        let is_empty = col.iter().all(|&c| c == ' ');
        if is_empty {
            if !current_block.is_empty() {
                total += calc_block(&current_block);
                current_block.clear();
            }
        } else {
            current_block.push(col.clone());
        }
    }
    if !current_block.is_empty() {
        total += calc_block(&current_block);
    }
    total
}

fn calc_block(cols: &[Vec<char>]) -> u64 {
    let rows = cols[0].len();
    let mut op = None;
    for col in cols {
        if let Some(o) = match col[rows - 1] {
            '*' => Some(Op::Mul),
            '+' => Some(Op::Plus),
            ' ' => None,
            _ => None,
        } {
            op = Some(o);
        }
    }
    let op = op.unwrap();
    let mut nums = Vec::new();
    for col in cols {
        let s: String = col[..rows - 1].iter().filter(|&&c| c != ' ').collect();
        if !s.is_empty() {
            nums.push(s.parse::<u64>().unwrap());
        }
    }
    nums.into_iter()
        .reduce(|acc, x| match op {
            Op::Plus => acc + x,
            Op::Mul => acc * x,
        })
        .unwrap()
}

fn main() {
    let lines = parse_input();
    println!("{} {}", part_one(&lines), part_two(&lines));
}
