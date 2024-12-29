use regex::Regex;
use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
enum Operator {
    Xor,
    Or,
    And,
}

#[derive(Debug, Clone)]
enum NodeType {
    Simple(i64),
    Operated(Operator, (String, String)),
}

#[derive(Debug)]
struct Network {
    nodes: HashMap<String, NodeType>,
    values: HashMap<String, i64>,
}

impl Network {
    fn resolve(&mut self, node_name: &str) -> i64 {
        if self.values.contains_key(node_name) {
            return self.values[node_name];
        }
        let result = match self.nodes.get(node_name).cloned().unwrap() {
            NodeType::Simple(v) => v,
            NodeType::Operated(op, operands) => {
                let (resolved_operand_1, resolved_operand_2) =
                    (self.resolve(&operands.0), self.resolve(&operands.1));
                match op {
                    Operator::And => resolved_operand_1 & resolved_operand_2,
                    Operator::Or => resolved_operand_1 | resolved_operand_2,
                    Operator::Xor => resolved_operand_1 ^ resolved_operand_2,
                }
            }
        };
        self.values.insert(node_name.to_string(), result);
        result
    }
}

fn parse_input() -> Network {
    let mut nodes = HashMap::new();
    for line in io::stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        let (node_name, node_value) = line.split_once(": ").unwrap();
        let node_value = node_value.parse().unwrap();
        nodes.insert(node_name.to_owned(), NodeType::Simple(node_value));
    }
    let op_re = Regex::new(r"(.*) (XOR|OR|AND) (.*) -> (.*)").unwrap();
    for line in io::stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        let captures = op_re.captures(&line).unwrap();
        let operands = (
            String::from(captures.get(1).unwrap().as_str()),
            String::from(captures.get(3).unwrap().as_str()),
        );
        let operator = captures.get(2).unwrap().as_str();
        let result = captures.get(4).unwrap().as_str();
        let node_type = match operator {
            "XOR" => NodeType::Operated(Operator::Xor, operands),
            "AND" => NodeType::Operated(Operator::And, operands),
            "OR" => NodeType::Operated(Operator::Or, operands),
            _ => panic!(),
        };
        nodes.insert(result.to_string(), node_type);
    }
    Network {
        nodes,
        values: HashMap::new(),
    }
}

fn part_one(network: &mut Network) -> i64 {
    let mut node_names: Vec<_> = network.nodes.keys().cloned().collect();
    node_names.sort();
    let mut res = 0;
    for node_name in node_names.iter().rev() {
        // Opposite because we want to start with the most significant
        if node_name.starts_with("z") {
            let value = network.resolve(node_name);
            res = res * 2 + value;
        }
    }
    res
}

fn main() {
    let mut network = parse_input();
    println!("{}", part_one(&mut network));
}
