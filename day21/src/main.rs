use itertools::Itertools;
use std::collections::HashMap;
use std::io;

type Point = (i32, i32);

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum LayoutType {
    Arrows,
    Numpad,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Key {
    c: char,
    point: Point,
}

#[derive(Clone)]
struct Layout {
    point_to_key: HashMap<Point, Key>,
    char_to_key: HashMap<char, Key>,
}

impl Layout {
    fn new(keys: Vec<Key>) -> Self {
        let mut point_to_key = HashMap::new();
        let mut char_to_key = HashMap::new();
        for &key in keys.iter() {
            point_to_key.insert(key.point, key);
            char_to_key.insert(key.c, key);
        }
        Layout {
            point_to_key,
            char_to_key,
        }
    }
}

struct ShortestSequences {
    numpad: Layout,
    arrows: Layout,
    saved: HashMap<(LayoutType, Key, Key, u32), u64>,
}

impl ShortestSequences {
    fn new(numpad: Layout, arrows: Layout) -> Self {
        Self {
            numpad,
            arrows,
            saved: HashMap::new(),
        }
    }

    fn sequence_length(
        &mut self,
        layout_type: LayoutType,
        source: Key,
        dest: Key,
        level: u32,
    ) -> u64 {
        if let Some(&saved_entry) = self.saved.get(&(layout_type, source, dest, level)) {
            return saved_entry;
        }

        let layout = match layout_type {
            LayoutType::Arrows => &self.arrows,
            LayoutType::Numpad => &self.numpad,
        };

        let (dx, dy) = (dest.point.0 - source.point.0, dest.point.1 - source.point.1);

        if level == 0 {
            return 1;
        }

        let horizontal_key = if dy >= 0 {
            self.arrows.char_to_key[&'>']
        } else {
            self.arrows.char_to_key[&'<']
        };

        let vertical_key = if dx >= 0 {
            self.arrows.char_to_key[&'v']
        } else {
            self.arrows.char_to_key[&'^']
        };

        let horizontal_path: String = (0..dy.abs()).map(|_c| horizontal_key.c).collect();
        let vertical_path: String = (0..dx.abs()).map(|_c| vertical_key.c).collect();

        let mut paths: Vec<String> = Vec::new();

        if layout
            .point_to_key
            .contains_key(&(source.point.0 + dx, source.point.1))
        {
            paths.push("A".to_string() + &vertical_path + &horizontal_path + "A");
        }

        if layout
            .point_to_key
            .contains_key(&(source.point.0, source.point.1 + dy))
        {
            paths.push("A".to_string() + &horizontal_path + &vertical_path + "A");
        }

        let mut possible_lengths = Vec::new();

        for path in paths {
            let mut path_seq_len = 0;
            for (c1, c2) in path.chars().into_iter().tuple_windows() {
                let source = self.arrows.char_to_key[&c1];
                let dest = self.arrows.char_to_key[&c2];
                let seq_len = self.sequence_length(LayoutType::Arrows, source, dest, level - 1);
                path_seq_len += seq_len;
            }
            possible_lengths.push(path_seq_len);
        }

        let res = *possible_lengths.iter().min().unwrap();

        self.saved.insert((layout_type, source, dest, level), res);

        res
    }
}

fn parse_input() -> Vec<String> {
    io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .collect()
}

fn main() {
    let arrows = Layout::new(vec![
        Key {
            c: '^',
            point: (0, 0),
        },
        Key {
            c: 'A',
            point: (0, 1),
        },
        Key {
            c: '<',
            point: (1, -1),
        },
        Key {
            c: 'v',
            point: (1, 0),
        },
        Key {
            c: '>',
            point: (1, 1),
        },
    ]);
    let numpad = Layout::new(vec![
        Key {
            c: '7',
            point: (0, 0),
        },
        Key {
            c: '8',
            point: (0, 1),
        },
        Key {
            c: '9',
            point: (0, 2),
        },
        Key {
            c: '4',
            point: (1, 0),
        },
        Key {
            c: '5',
            point: (1, 1),
        },
        Key {
            c: '6',
            point: (1, 2),
        },
        Key {
            c: '1',
            point: (2, 0),
        },
        Key {
            c: '2',
            point: (2, 1),
        },
        Key {
            c: '3',
            point: (2, 2),
        },
        Key {
            c: '0',
            point: (3, 1),
        },
        Key {
            c: 'A',
            point: (3, 2),
        },
    ]);
    let mut shortest_sequences = ShortestSequences::new(numpad.clone(), arrows.clone());
    let mut part_1 = 0;
    let mut part_2 = 0;
    let seqs = parse_input();
    for seq in seqs {
        let mut part_1_contribution: u64 = 0;
        let mut part_2_contribution: u64 = 0;
        let val: u64 = seq[..seq.len() - 1].parse().unwrap();
        let seq = "A".to_string() + &seq;
        for (c1, c2) in seq.chars().tuple_windows() {
            let source = numpad.char_to_key[&c1];
            let dest = numpad.char_to_key[&c2];
            part_1_contribution +=
                shortest_sequences.sequence_length(LayoutType::Numpad, source, dest, 1 + 2);
            part_2_contribution +=
                shortest_sequences.sequence_length(LayoutType::Numpad, source, dest, 1 + 25);
        }
        part_1 += part_1_contribution * val;
        part_2 += part_2_contribution * val;
    }
    println!("{} {}", part_1, part_2);
}
