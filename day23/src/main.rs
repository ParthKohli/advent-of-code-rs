use core::time;
use std::{
    cmp::max,
    collections::{BTreeMap, HashSet},
    io,
    time::{Duration, Instant},
};

#[derive(Default)]
struct NetworkBuilder {
    edges: Vec<(String, String)>,
}

#[derive(Debug)]
struct Network {
    edges: Vec<(String, String)>,
    adjacent_vertices: BTreeMap<String, HashSet<String>>,
}

impl NetworkBuilder {
    fn add_edge(&mut self, a: String, b: String) {
        self.edges.push((a, b));
    }

    fn build(&self) -> Network {
        let mut adjacent_vertices: BTreeMap<String, HashSet<String>> = Default::default();
        for (a, b) in self.edges.iter() {
            adjacent_vertices
                .entry(a.clone())
                .or_insert(Default::default())
                .insert(b.clone());
            adjacent_vertices
                .entry(b.clone())
                .or_insert(Default::default())
                .insert(a.clone());
        }
        Network {
            edges: self.edges.clone(),
            adjacent_vertices,
        }
    }
}

fn parse_input() -> Network {
    let mut builder = NetworkBuilder::default();
    for line in io::stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        let mut parts = line.split('-');
        builder.add_edge(
            parts.next().unwrap().to_string(),
            parts.next().unwrap().to_string(),
        );
    }
    builder.build()
}

fn part_one(network: &Network) -> i32 {
    let mut total = 0;
    let mut bad = 0;
    for (a, b) in network.edges.iter() {
        for c in network.adjacent_vertices[a].iter() {
            if network.adjacent_vertices[b].contains(c) {
                total += 1;
                if [a, b, c].iter().all(|s| !s.starts_with('t')) {
                    bad += 1;
                }
            }
        }
    }
    (total - bad) / 3
}

fn max_clique_search<'a>(
    network: &Network,
    current_clique: &mut Vec<&'a str>,
    best_clique: &mut Vec<&'a str>,
    remaining_vertices: &'a [String],
    run_until: Instant,
) -> usize {
    let mut res = current_clique.len();
    if Instant::now() >= run_until {
        return res;
    }
    if remaining_vertices.is_empty() {
        if current_clique.len() >= best_clique.len() {
            best_clique.clear();
            best_clique.extend_from_slice(&current_clique);
        }
        return res;
    }
    for (idx, vertex) in remaining_vertices.iter().enumerate() {
        if current_clique
            .iter()
            .all(|existing| network.adjacent_vertices[vertex].contains(*existing))
        {
            current_clique.push(vertex);
            res = max(
                res,
                max_clique_search(
                    network,
                    current_clique,
                    best_clique,
                    &remaining_vertices[(idx + 1)..remaining_vertices.len()],
                    run_until,
                ),
            );
            current_clique.pop();
        }
    }
    res = max(
        res,
        max_clique_search(
            network,
            current_clique,
            best_clique,
            &remaining_vertices[remaining_vertices.len()..remaining_vertices.len()],
            run_until,
        ),
    );
    res
}

fn part_two(network: &Network) -> String {
    let mut vertices = network
        .adjacent_vertices
        .keys()
        .cloned()
        .collect::<Vec<_>>();
    vertices.sort();
    let mut best_clique = Vec::new();
    let _ = max_clique_search(
        &network,
        &mut Vec::new(),
        &mut best_clique,
        &vertices,
        Instant::now() + Duration::from_secs(10),
    );
    best_clique.join(",")
}

fn main() {
    let network = parse_input();
    println!("{} {:?}", part_one(&network), part_two(&network));
}
