use std::io;

use regex::Regex;

fn extract_result(haystack: &str) -> i64 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut res = 0;
    for (_, [a, b]) in re.captures_iter(haystack).map(|c| c.extract()) {
        res += a.parse::<i64>().unwrap() * b.parse::<i64>().unwrap();
    }
    res
}

fn part_one(lines: &Vec<String>) -> i64 {
    let mut res: i64 = 0;
    for s in lines {
        res += extract_result(s)
    }
    res
}

fn part_two(lines: &Vec<String>) -> i64 {
    let outer_re: Regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").unwrap();
    let mut res: i64 = 0;
    let mut enabled: bool = true;
    for s in lines {
        let matches = outer_re.find_iter(&s).map(|m| m.as_str());
        for found in matches {
            match found {
                "do()" => {
                    enabled = true;
                }
                "don't()" => {
                    enabled = false;
                }
                _ => {
                    if enabled {
                        res += extract_result(found);
                    }
                }
            }
        }
    }
    res
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().map(|line| line.unwrap()).collect();
    println!("{} {}", part_one(&lines), part_two(&lines));
}
