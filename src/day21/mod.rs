use std::collections::HashMap;
use std::process::exit;
use regex::Regex;
use crate::day21::Operand::{Value, Variable};
use crate::read_file;

#[derive(Debug, Clone)]
enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    Player,
}

#[derive(Debug, Clone)]
enum Operand {
    Value(i64),
    Variable(String),
}


#[derive(Debug, Clone)]
struct Monkey {
    id: String,
    left: Operand,
    right: Operand,
    operator: Operator,
    remove: bool,
}


fn parse_input(input: &str) -> Vec<Monkey> {
    let input = read_file(input);
    let mut monkeys = vec![];

    let regex = Regex::new(r"(\w+): ((\d+)|(\w+) (.) (\w+))").unwrap();

    for line in input.lines() {
        let cap = regex.captures(line).unwrap();
        match cap.get(6) {
            None => {
                monkeys.push(Monkey {
                    id: cap[1].to_string(),
                    left: Value(cap[2].parse().unwrap()),
                    right: Value(0),
                    operator: Operator::Plus,
                    remove: false,
                });
            }
            Some(_) => {
                monkeys.push(Monkey {
                    id: cap[1].to_string(),
                    left: match cap[4].parse() {
                        Err(_) => Variable(cap[4].to_string()),
                        Ok(v) => Operand::Value(v)
                    },
                    right: (match cap[6].parse() {
                        Err(_) => Variable(cap[6].to_string()),
                        Ok(v) => Operand::Value(v)
                    }),
                    operator: match &cap[5] {
                        "+" => Operator::Plus,
                        "-" => Operator::Minus,
                        "*" => Operator::Multiply,
                        "/" => Operator::Divide,
                        _ => {
                            println!("Unknown Operaotr: {}", &cap[5]);
                            exit(1);
                        }
                    },
                    remove: false,
                });
            }
        }
    }

    monkeys
}



pub fn run() {
    let mut monkeys = parse_input("input/day21.txt");
    let mut value_map: HashMap<String, i64> = HashMap::new();

    loop {
        monkeys.retain(|monkey| !monkey.remove);

        for monkey in &mut monkeys {
            match &monkey.left {
                Variable(id) => match value_map.get(id) {
                    Some(v) => {
                        monkey.left = Operand::Value(*v);
                    }
                    _ => {}
                },
                Operand::Value(_) => {}
            }
            match &monkey.right {
                Variable(id) => match value_map.get(id) {
                    Some(v) => {
                        monkey.right = Operand::Value(*v);
                    }
                    _ => {}
                },
                Operand::Value(_) => {}
            }
            match (&monkey.left, &monkey.right) {
                (Operand::Value(a), Operand::Value(b)) => {
                    monkey.remove = true;
                    match &monkey.operator {
                        Operator::Plus => {
                            value_map.insert(monkey.id.clone(), a + b);
                        }
                        Operator::Minus => {
                            value_map.insert(monkey.id.clone(), a - b);
                        }
                        Operator::Multiply => {
                            value_map.insert(monkey.id.clone(), a * b);
                        }
                        Operator::Divide => {
                            value_map.insert(monkey.id.clone(), a / b);
                        }
                        _ => {}
                    }
                }
                (_, _) => {}
            }
        }

        match value_map.get("root") {
            None => {}
            Some(v) => {
                println!("Root Value is: {}", v);
                break;
            }
        };
    }
}

pub fn run2() {
    let mut monkeys = parse_input("input/day21.txt");
    let mut value_map: HashMap<String, i64> = HashMap::new();

    let root_position = monkeys.iter().position(|monkey| monkey.id == "root".to_string()).unwrap();
    monkeys[root_position].operator = Operator::Equal;
    let humn_position = monkeys.iter().position(|monkey| monkey.id == "humn".to_string()).unwrap();
    monkeys[humn_position].operator = Operator::Player;

    loop {
        for monkey in &mut monkeys {
            match &monkey.left {
                Variable(id) => match value_map.get(id) {
                    Some(v) => {
                        monkey.left = Operand::Value(*v);
                    }
                    _ => {}
                },
                Operand::Value(_) => {}
            }
            match &monkey.right {
                Variable(id) => match value_map.get(id) {
                    Some(v) => {
                        monkey.right = Operand::Value(*v);
                    }
                    _ => {}
                },
                Operand::Value(_) => {}
            }
            match (&monkey.left, &monkey.right) {
                (Operand::Value(a), Operand::Value(b)) => {
                    match &monkey.operator {
                        Operator::Plus => {
                            monkey.remove = true;
                            value_map.insert(monkey.id.clone(), a + b);
                        }
                        Operator::Minus => {
                            monkey.remove = true;
                            value_map.insert(monkey.id.clone(), a - b);
                        }
                        Operator::Multiply => {
                            monkey.remove = true;
                            value_map.insert(monkey.id.clone(), a * b);
                        }
                        Operator::Divide => {
                            monkey.remove = true;
                            value_map.insert(monkey.id.clone(), a / b);
                        }
                        Operator::Equal => {
                            break;
                        }
                        Operator::Player => {}
                    }
                }
                (_, _) => {}
            }
        }

        let count = monkeys.len();
        monkeys.retain(|monkey| !monkey.remove);
        if count == monkeys.len() {
            break;
        }
    }

    let current_position = monkeys.iter().position(|monkey| monkey.id == "root".to_string()).unwrap();
    let mut target;
    let mut next_pos;
    match &monkeys[current_position].left {
        Value(v) => {
            target = *v;
            match &monkeys[current_position].right {
                Value(_) => {
                    unreachable!();
                }
                Variable(v) => {
                    next_pos = (*v).clone();
                }
            }
        }
        Variable(v) => {
            next_pos = (*v).clone();
            match &monkeys[current_position].right {
                Value(v) => {
                    target = *v;
                }

                Variable(_) => {
                    unreachable!();
                }
            }
        }
    }

    loop {
        if next_pos == "humn".to_string() {
            println!("You have to yell: {}",target);
            exit(1);
        }
        let current_position = monkeys.iter().position(|monkey| monkey.id == next_pos).unwrap();

        let monkey: &Monkey = &monkeys[current_position];
        match &monkey.left {
            Value(v) => {
                match &monkey.operator {
                    Operator::Plus => {
                        target -= v;
                    }
                    Operator::Minus => {
                        target = v - target;
                    }
                    Operator::Multiply => {
                        target /= v;
                    }
                    Operator::Divide => {
                        target = v / target;
                    }
                    _ => {}
                }
                match &monkey.right {
                    Value(_) => {
                        unreachable!();
                    }
                    Variable(v) => {
                        next_pos = (*v).clone();
                    }
                }
            }
            Variable(v) => {
                next_pos = (*v).clone();
                match &monkey.right {
                    Value(v) => {
                        match &monkey.operator {
                            Operator::Plus => {
                                target -= v;
                            }
                            Operator::Minus => {
                                target += v;
                            }
                            Operator::Multiply => {
                                target /= v;
                            }
                            Operator::Divide => {
                                target *= v;
                            }
                            _ => {}
                        }
                    }
                    Variable(_) => {
                        unreachable!();
                    }
                }

            }
        }
    }
}