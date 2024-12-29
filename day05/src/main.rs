use std::collections::{HashMap, HashSet};
use std::io;

type Page = i32;

#[derive(Default)]
pub struct PageOrderBuilder {
    ordered_edges: Vec<(Page, Page)>,
}

impl PageOrderBuilder {
    pub fn add_ordered_edge(
        &mut self,
        (independent_page, dependent_page): (Page, Page),
    ) -> &mut Self {
        self.ordered_edges.push((independent_page, dependent_page));
        self
    }

    pub fn build_topological_order(&self) -> Vec<Page> {
        let mut indegree: HashMap<Page, i32> = HashMap::new();

        for &(independent_page, dependent_page) in self.ordered_edges.iter() {
            indegree.entry(independent_page).or_insert(0);
            *indegree.entry(dependent_page).or_insert(0) += 1;
        }

        let mut topological_order: Vec<Page> = Vec::new();

        loop {
            let next_page = match indegree
                .keys()
                .find(|page| *indegree.get(page).unwrap() == 0)
            {
                Some(v) => *v,
                _ => {
                    break;
                }
            };

            topological_order.push(next_page);
            indegree.remove(&next_page);

            for dependent_page in self
                .ordered_edges
                .iter()
                .filter(|(out_page, _)| *out_page == next_page)
                .map(|(_, in_page)| *in_page)
            {
                indegree
                    .entry(dependent_page)
                    .and_modify(|indeg| *indeg -= 1);
            }
        }

        topological_order
    }

    pub fn build(self) -> PageOrder {
        let topological_order = self.build_topological_order();
        PageOrder {
            ordered_edges: self.ordered_edges,
            topological_order,
        }
    }
}

pub struct PageOrder {
    ordered_edges: Vec<(Page, Page)>,
    topological_order: Vec<Page>,
}

impl PageOrder {
    pub fn validate_order(&self, order: &[Page]) -> bool {
        let mut idx_in_order: HashMap<Page, usize> = HashMap::new();
        for (idx, &page) in order.iter().enumerate() {
            idx_in_order.insert(page, idx);
        }

        self.ordered_edges
            .iter()
            .all(|(independent_page, dependent_page)| {
                match (
                    idx_in_order.get(independent_page),
                    idx_in_order.get(dependent_page),
                ) {
                    (Some(u), Some(v)) => u < v,
                    _ => true,
                }
            })
    }

    pub fn topological_reorder(&self, pages: &[Page]) -> Vec<Page> {
        let pages: HashSet<&i32> = HashSet::from_iter(pages);
        let mut subgraph_order_builder = PageOrderBuilder::default();
        let filtered_edges = self
            .ordered_edges
            .iter()
            .filter(|(a, b)| pages.contains(a) && pages.contains(b));
        for filtered_edge in filtered_edges {
            subgraph_order_builder.add_ordered_edge(*filtered_edge);
        }
        let subgraph_order = subgraph_order_builder.build();
        subgraph_order.topological_order
    }
}

fn parse_input() -> (PageOrder, Vec<Vec<Page>>) {
    let mut page_order_builder = PageOrderBuilder::default();
    for line in io::stdin().lines().map(|line| line.unwrap()) {
        if line.is_empty() {
            break;
        }
        let mut pages = line.split('|');
        page_order_builder.add_ordered_edge((
            pages.next().unwrap().parse::<Page>().unwrap(),
            pages.next().unwrap().parse::<Page>().unwrap(),
        ));
    }
    let mut orderings: Vec<Vec<Page>> = Vec::new();
    for line in io::stdin().lines().map(|line| line.unwrap()) {
        if line.is_empty() {
            break;
        }
        let ordering = line
            .split(',')
            .map(|token| token.parse::<Page>().unwrap())
            .collect();
        orderings.push(ordering);
    }
    (page_order_builder.build(), orderings)
}

fn part_one(page_order: &PageOrder, orderings: &[Vec<Page>]) -> i64 {
    orderings
        .iter()
        .filter(|ordering| page_order.validate_order(ordering))
        .map(|ordering| ordering[ordering.len() / 2] as i64)
        .sum()
}

fn part_two(page_order: &PageOrder, orderings: &[Vec<Page>]) -> i64 {
    orderings
        .iter()
        .filter(|ordering| !page_order.validate_order(ordering))
        .map(|ordering| page_order.topological_reorder(ordering))
        .map(|ordering| ordering[ordering.len() / 2] as i64)
        .sum()
}

fn main() {
    let (page_order, orderings) = parse_input();
    println!(
        "{} {}",
        part_one(&page_order, &orderings),
        part_two(&page_order, &orderings)
    );
}
