use itertools::Itertools;
use regex::Regex;
use std::cmp::min;
use std::io;

#[derive(Debug, Clone, Copy)]
struct Prize {
    x: i64,
    y: i64,
}

impl Prize {
    fn parse(s: &str) -> Self {
        let re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
        if let Some(captures) = re.captures(s) {
            let x: i64 = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let y: i64 = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
            Prize { x, y }
        } else {
            panic!();
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Button {
    dx: i64,
    dy: i64,
    cost_per_press: i64,
}

impl Button {
    fn parse(s: &str) -> Self {
        let re = Regex::new(r"Button (A|B): X\+(\d+), Y\+(\d+)").unwrap();
        if let Some(captures) = re.captures(s) {
            let button_type = captures.get(1).unwrap().as_str();
            let dx: i64 = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
            let dy: i64 = captures.get(3).unwrap().as_str().parse::<i64>().unwrap();
            Button {
                dx,
                dy,
                cost_per_press: match button_type {
                    "A" => 3,
                    _ => 1,
                },
            }
        } else {
            panic!();
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Machine {
    buttons: (Button, Button),
    prize: Prize,
}

fn read_input() -> Vec<Machine> {
    let chunks = io::stdin()
        .lines()
        .map(|x| x.unwrap())
        .filter(|x| !x.is_empty())
        .chunks(3);
    chunks
        .into_iter()
        .map(|chunk| chunk.collect_tuple().unwrap())
        .map(|(a, b, c)| Machine {
            buttons: (Button::parse(&a), Button::parse(&b)),
            prize: Prize::parse(&c),
        })
        .collect_vec()
}

fn calculate_cost(machines: &Vec<Machine>) -> i64 {
    const NUM_TIMES: i64 = 100;
    let mut res = 0;
    for machine in machines {
        let mut minimum_cost: Option<i64> = None;
        for (a_presses, b_presses) in (0..=NUM_TIMES).cartesian_product(0..=NUM_TIMES) {
            let cost = a_presses * machine.buttons.0.cost_per_press
                + b_presses * machine.buttons.1.cost_per_press;
            let x = machine.buttons.0.dx * a_presses + machine.buttons.1.dx * b_presses;
            let y = machine.buttons.0.dy * a_presses + machine.buttons.1.dy * b_presses;
            if x == machine.prize.x && y == machine.prize.y {
                minimum_cost = match minimum_cost {
                    None => Some(cost),
                    Some(previous_cost) => Some(min(previous_cost, cost)),
                }
            }
        }
        res += match minimum_cost {
            None => 0,
            Some(cost) => cost,
        }
    }
    res
}

fn calculate_cost_smart(machines: &Vec<Machine>) -> i64 {
    let mut res = 0;
    for machine in machines {
        let mut minimum_cost: Option<i64> = None;
        /*
            x * a + y * c == e
            x * b + y * d == f

            x = Dx / D = (ed - cf) / (ad - bc)
            y = Dy / D = (af - be) / (ad - bc)
        */
        let (a, b) = (machine.buttons.0.dx as i64, machine.buttons.0.dy as i64);
        let (c, d) = (machine.buttons.1.dx as i64, machine.buttons.1.dy as i64);
        let (e, f) = (machine.prize.x as i64, machine.prize.y as i64);

        let det = a * d - b * c;
        if det != 0 {
            if (e * d - c * f) % det == 0 && (a * f - b * e) % det == 0 {
                let x = (e * d - c * f) / det;
                let y = (a * f - b * e) / det;
                if x >= 0 && y >= 0 {
                    minimum_cost = Some(3 * x + y);
                }
            }
        } else {
            panic!();
        }
        res += match minimum_cost {
            None => 0,
            Some(cost) => cost,
        }
    }
    res
}

fn part_two(machines: &mut Vec<Machine>) -> i64 {
    for machine in machines.iter_mut() {
        machine.prize.x += 10000000000000;
        machine.prize.y += 10000000000000;
    }
    calculate_cost_smart(&machines)
}

/*
x * (a, b) -> 3x
y * (c, d) -> y
(e, f)

x * a + y * c == e
x * b + y * d == f

minimize 3*x + y
*/

fn main() {
    let mut machines = read_input();
    println!("{} {}", calculate_cost(&machines), part_two(&mut machines));
}
