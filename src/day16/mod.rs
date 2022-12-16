use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Node {
    id: String,
    flow: u32,
    neighbours: Vec<String>,
}

fn parse_input(input: String) -> Vec<Node> {
    let mut nodes: Vec<Node> = vec![];
    for line in input.lines() {
        nodes.push(parse_line(line));
    }
    nodes
}

pub fn parse_line(input: &str) -> Node {
    let regex = Regex::new(r"Valve ([A-Z][A-Z]) has flow rate=(\d+); tunnels? leads? to valves? (.*)$").unwrap();

    let cap = regex.captures(input).unwrap();

    Node {
        id: cap[1].to_string(),
        flow: cap[2].parse().unwrap(),
        neighbours: cap[3].split(", ").map(|part| part.to_owned()).collect(),
    }
}

pub fn run() {
    todo!()
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
    fn parses_input() {
        let nodes = parse_input(read_file("input/day16-test.txt"));
        println!("{:?}",nodes);
    }
}