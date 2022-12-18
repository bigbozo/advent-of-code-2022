use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use regex::{Captures, Regex};
use crate::day16::Action::{Valve, Walk};
use crate::read_file;

#[derive(PartialEq, Copy, Clone)]
pub struct Edge {
    to: NodeId,
    weight: u32,
}

pub enum Action {
    Valve(NodeId),
    Walk(NodeId, NodeId, u32),
}

#[derive(PartialEq)]
pub struct Node {
    id: NodeId,
    flow: u32,
    valve_open: bool,
    neighbours: Vec<Edge>,
}


type NodeId = u32;

pub fn make_id(id_string: &str) -> u32 {
    let mut val: u32 = 0;
    for char in id_string.chars() {
        val *= 256;
        val += char as u32;
    }
    val
}

impl Node {
    pub fn new(id: NodeId, flow: u32, neighbours: Vec<Edge>) -> Node {
        Node {
            id,
            flow,
            valve_open: flow < 1,
            neighbours,
        }
    }
    pub fn swap_neighbours(&mut self, a: NodeId, b: NodeId, weight: u32) {
        for i in 0..self.neighbours.len() {
            if self.neighbours[i].to == a {
                self.neighbours[i].to = b;
                self.neighbours[i].weight = weight;
            }
        }
    }
}
impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, F:{}, {:?}]", make_node_string(self.id),self.flow,self.neighbours)
    }
}
impl Debug for Edge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", make_node_string(self.to),self.weight)
    }
}

fn parse_input(input: &str) -> HashMap<NodeId, Node> {
    let mut nodes: HashMap<NodeId, Node> = HashMap::new();
    let mut node_ids: Vec<NodeId> = vec![];

    for line in input.lines() {
        let node = parse_line(line);
        node_ids.push(node.id);
        nodes.insert(node.id, node);
    }

    for id in node_ids {
        let node = nodes.get(&id).unwrap();
        if node.flow == 0 && id != make_id("AA") {
            let x = node.neighbours[0].to;
            let y = node.neighbours[1].to;
            let weight = node.neighbours[0].weight + node.neighbours[1].weight;
            nodes.remove(&id);
            nodes.entry(x).and_modify(|node| node.swap_neighbours(id, y, weight));
            nodes.entry(y).and_modify(|node| node.swap_neighbours(id, x, weight));
        }
    }

    nodes
}

pub fn parse_line(input: &str) -> Node {
    let regex = Regex::new(r"Valve ([A-Z][A-Z]) has flow rate=(\d+); tunnels? leads? to valves? (.*)$").unwrap();

    let cap: Captures = regex.captures(input)
        .unwrap();

    Node::new(
        make_id(&cap[1]),
        cap[2].parse().unwrap(),
        cap[3].split(", ").map(|c| Edge { to: make_id(c), weight: 1 }).collect(),
    )
}

fn walk(nodes: &mut HashMap<NodeId, Node>, current_node: NodeId, previous_node: NodeId, steps: u32, open_valves: u32) -> (u32, Vec<Action>) {
    if steps < 1 {
        return (0, vec![]);
    }

    if open_valves as usize == nodes.len() {
        println!("All valves open!");
        return (0, vec![]);
    }

    let mut max_score: u32 = 0;
    let mut best_steps: Vec<Action> = vec![];

    if !nodes.get(&current_node).unwrap().valve_open {
        nodes.entry(current_node).and_modify(|node| node.valve_open = true);
        let node = nodes.get(&current_node).unwrap();
        let score = node.flow * (steps - 1);
        let neighbours = node.neighbours.clone();
        for neighbour in neighbours {
            // makes only sense if we can walk AND open the valve AND there's still time after
            if steps > 1 + neighbour.weight {
                let (score, done_steps) = walk(nodes, neighbour.to, 0, steps - 1 - neighbour.weight, open_valves + 1);
                if score > max_score {
                    best_steps = done_steps;
                    best_steps.push(Valve(current_node));
                    best_steps.push(Walk(current_node, neighbour.to, neighbour.weight));
                    max_score = score;
                }
            }
        }
        nodes.entry(current_node).and_modify(|node| node.valve_open = false);
        max_score += score;
    }

    let neighbours = nodes.get(&current_node).unwrap().neighbours.clone();

    for neighbour in neighbours {
        if neighbour.to != previous_node {
            // makes only sense if we can walk AND open the valve AND there's still time after
            if steps > 1 + neighbour.weight {
                let (score, done_steps) = walk(nodes, neighbour.to, current_node, steps - neighbour.weight, open_valves);
                if score > max_score {
                    best_steps = done_steps;
                    best_steps.push(Walk(current_node, neighbour.to, neighbour.weight));
                    max_score = score;
                }
            }
        }
    }

    (max_score, best_steps)
}

pub fn run() {
    println!("You must be kidding, how can this be solved? ... Wait ...");
    let mut nodes = parse_input(&read_file("input/day16.txt"));

    //    output_dotfile(&mut nodes);
    let (score, steps) = walk(&mut nodes, make_id("AA"), 0, 30, 0);
    println!("Here, take that score: {}\n", score);
    for s in steps {
        match s {
            Valve(node_id) => { println!("Open valve: {}", make_node_string(node_id)) }
            Walk(from_id, node_id, weight) => { println!("Walk from {} to {} (Costs: {})", make_node_string(from_id), make_node_string(node_id), weight) }
        }
    }
}

fn make_node_string(p0: NodeId) -> String {
    [char::from_u32(p0 / 256), char::from_u32(p0 % 256)].into_iter().flatten().collect::<String>()
}

pub fn run2() {
    println!("Sorry, won't let animals work!")
}

#[cfg(test)]
mod test {
    use crate::read_file;
    use super::*;

    #[test]
    fn parses_line() {
        //assert_eq!(Node { id: "AA".to_string(), flow: 0, neighbours: vec!["DD".to_string(), "II".to_string(), "BB".to_string()] }, parse_line("Valve AA has flow rate=0; tunnels lead to valves DD, II, BB"))
    }

    #[test]
    fn parses_input() {
        let mut nodes = parse_input(&read_file("input/day16-test.txt"));

        let score = walk(&mut nodes, make_id("AA"), 0, 30,0);

    }
}