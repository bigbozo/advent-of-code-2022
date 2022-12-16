use std::collections::HashMap;
use regex::{Captures, Regex};
use crate::read_file;

#[derive(Debug, PartialEq,Clone)]
pub struct Node {
    id: String,
    flow: u32,
    neighbours: Vec<String>,
}

impl Node {
    pub fn swap_neighbours(&mut self, a: &str, b: String) {
        for i in 0..self.neighbours.len() {
            if self.neighbours[i] == a {
                self.neighbours[i] = b.to_string();
            }
        }
    }
}

fn parse_input(input: &str) -> HashMap<String,Node> {
    let mut nodes: HashMap<String,Node> = HashMap::new();
    let mut node_ids = vec![];

    for line in input.lines() {
        let node = parse_line(line);
        node_ids.push(node.id.to_string());
        nodes.insert(node.id.to_string(),node);
    }

    for id in node_ids {
        let mut node = nodes.get(&id).unwrap();
        if node.flow == 0 && id != "AA" {
            nodes.get(&node.neighbours[0]).unwrap().swap_neighbours(&id, node.neighbours[1].to_string());
            nodes.get(&node.neighbours[1]).unwrap().swap_neighbours(&id, node.neighbours[0].to_string());
            nodes.remove(&id);
        }
    }

    nodes
}

pub fn parse_line(input: &str) -> Node {
    let regex = Regex::new(r"Valve ([A-Z][A-Z]) has flow rate=(\d+); tunnels? leads? to valves? (.*)$").unwrap();

    let cap: Captures = regex.captures(input)
        .unwrap();

    Node {
        id: cap[1].to_string(),
        flow: cap[2].parse().unwrap(),
        neighbours: cap[3].split(", ").map(|c| c.to_string()).collect(),
    }
}

pub fn run() {
    let mut nodes = parse_input(&read_file("input/day16.txt"));
    println!("digraph {}", "{");




    for (id, node) in &nodes {
        println!("{} [label =\"{} {}\"]", id, id, node.flow);


        for neighbour in &node.neighbours {
            println!("{} -> {}", id, neighbour);
        }
    }
    println!("{}", "}");
}

pub fn run2() {}

#[cfg(test)]
mod test {
    use crate::read_file;
    use super::*;

    #[test]
    fn parses_line() {
        assert_eq!(Node { id: "AA".to_string(), flow: 0, neighbours: vec!["DD".to_string(), "II".to_string(), "BB".to_string()] }, parse_line("Valve AA has flow rate=0; tunnels lead to valves DD, II, BB"))
    }

    #[test]
    fn parses_input() {}
}