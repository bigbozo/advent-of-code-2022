use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::process::exit;
use ansi_term::Colour::Yellow;
use crate::read_file;
use crate::day09::Direction::{D, L, R, U};

#[derive(Debug, PartialEq)]
enum Direction {
    U,
    R,
    D,
    L,
}

#[derive(Debug, PartialEq)]
struct Command {
    dir: Direction,
    amount: u32,
}


#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn chase(&mut self, target: Point) {
        if (target.x - self.x).abs() > 1 || (target.y - self.y).abs() > 1 {
            if target.x < self.x {
                self.x -= 1;
            }
            if target.x > self.x {
                self.x += 1;
            }
            if target.y > self.y {
                self.y += 1;
            }
            if target.y < self.y {
                self.y -= 1;
            }
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.x, self.y)
    }
}


fn parse_input(input: String) -> Vec<Command> {
    let mut commands: Vec<Command> = vec![];

    for line in input.lines() {
        let (command, amount) = line.split_once(' ').unwrap();
        commands.push(Command {
            dir: match command {
                "U" => U,
                "D" => D,
                "L" => L,
                "R" => R,
                _ => {
                    println!("Unknown Command");
                    exit(1);
                }
            },
            amount: amount.parse().unwrap(),
        })
    }

    commands
}

fn walk(input: String) -> usize {
    let commands: Vec<Command> = parse_input(input);

    let mut positions: HashMap<String, bool> = HashMap::new();

    let mut head: Point = Point { x: 0, y: 0 };
    let mut tail: Point = Point { x: 0, y: 0 };

    positions.entry(tail.to_string()).or_insert(true);

    for command in commands {
        for _step in 0..command.amount {
            match command.dir {
                U => head.y -= 1,
                R => head.x += 1,
                D => head.y += 1,
                L => head.x -= 1
            }
            tail.chase(head);
            positions.entry(tail.to_string()).or_insert(true);
        }
    }

    positions.len()
}

fn walk10(input: String) -> usize {
    let commands: Vec<Command> = parse_input(input);

    let mut positions: HashMap<String, bool> = HashMap::new();
    let mut rope = [Point { x: 0, y: 0 }; 10];


    positions.entry(rope[9].to_string()).or_insert(true);

    for command in commands {
        for _step in 0..command.amount {
            match command.dir {
                U => rope[0].y -= 1,
                R => rope[0].x += 1,
                D => rope[0].y += 1,
                L => rope[0].x -= 1
            }
            for i in 1..10 {
                rope[i].chase(rope[i - 1]);
            }
            positions.entry(rope[9].to_string()).or_insert(true);
        }
    }

    positions.len()
}

pub fn run() {
    let input = read_file("input/day09.txt");
    let count = walk(input);

    println!("The Tail visits {} positions.", Yellow.bold().paint(format!("{}", count)));
}

pub fn run2() {
    let input = read_file("input/day09.txt");
    let count = walk10(input);

    println!("The 10-Foot-Rope-Tail visits {} positions.", Yellow.bold().paint(format!("{}", count)));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn parse_test() {
        let input = read_file("input/day09-test.txt");
        let commands: Vec<Command> = parse_input(input);
        assert_eq!(Command { dir: R, amount: 4 }, commands[0]);
        assert_eq!(Command { dir: U, amount: 4 }, commands[1]);
        assert_eq!(Command { dir: L, amount: 3 }, commands[2]);
        assert_eq!(Command { dir: D, amount: 1 }, commands[3]);
    }

    #[test]
    pub fn walker_test() {
        let input = read_file("input/day09-test.txt");
        assert_eq!(13, walk(input));
    }

    #[test]
    pub fn walker2_test() {
        let input = read_file("input/day09-test.txt");
        assert_eq!(1, walk10(input));

        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(36, walk10(input.to_string()));
    }
}