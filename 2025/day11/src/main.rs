use std::collections::{HashMap, HashSet};
use std::io;

fn parse_graph() -> HashMap<String, Vec<String>> {
    let entries = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (src, dests) = line.split_once(": ").unwrap();
            (
                src.to_string(),
                dests
                    .split_whitespace()
                    .map(|s: &str| s.to_owned())
                    .collect(),
            )
        });
    HashMap::from_iter(entries)
}

fn topological_order(graph: &HashMap<String, Vec<String>>) -> Vec<&str> {
    let mut indeg: HashMap<&str, usize> = HashMap::new();
    let mut universe = HashSet::new();
    for (src, dests) in graph {
        universe.insert(src);
        for dest in dests {
            *indeg.entry(dest).or_default() += 1;
            universe.insert(dest);
        }
    }
    let universe_len = universe.len();
    let mut order = Vec::new();
    let mut added = HashSet::<&str>::new();
    for _ in 0..universe_len {
        for s in &universe {
            if !added.contains(s.as_str()) && *indeg.entry(s.as_str()).or_default() == 0 {
                added.insert(s);
                order.push(s.as_str());
                if let Some(dests) = graph.get(s.as_str()) {
                    for dest in dests {
                        indeg.entry(dest).and_modify(|id| *id -= 1);
                    }
                }
            }
        }
    }
    order
}

fn solve(graph: &HashMap<String, Vec<String>>, order: &[&str], start_node: &str) -> (u64, u64) {
    let mut dp: HashMap<(&str, u8), u64> = HashMap::new();
    dp.insert((start_node, 0), 1);
    for &v in order {
        for vis_cnt in 0..=2_u8 {
            let cnt = *dp.get(&(v, vis_cnt)).unwrap_or(&0u64);
            if let Some(dests) = graph.get(v) {
                for dest in dests {
                    let new_vis_cnt = if ["fft", "dac"].contains(&dest.as_str()) {
                        vis_cnt + 1
                    } else {
                        vis_cnt
                    };
                    *dp.entry((dest, new_vis_cnt)).or_default() += cnt;
                }
            }
        }
    }
    let mut part_one = 0;
    let mut part_two = 0;
    for vis_cnt in 0..=2 {
        if let Some(cnt) = dp.get(&("out", vis_cnt)) {
            part_one += cnt;
            if vis_cnt == 2 {
                part_two += cnt;
            }
        }
    }
    (part_one, part_two)
}

fn main() {
    let graph = parse_graph();
    let order = topological_order(&graph);
    println!("{:?}", solve(&graph, &order, "svr"));
    println!("{:?}", solve(&graph, &order, "you"));
}
