use std::cmp::Ordering;
use crate::day13::ListItem::{ItemList, Value};
use crate::day13::ParseMode::{FindItems, FindRight};

#[derive(Debug, Eq)]
enum ListItem {
    ItemList(Vec<ListItem>),
    Value(u32),
}

impl PartialEq for ListItem {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value(a), Value(b)) => a == b,
            (Value(a), ItemList(b)) => vec![Value(*a)] == *b,
            (ItemList(a), Value(b)) => vec![Value(*b)] == *a,
            (ItemList(a), ItemList(b)) => a == b
        }
    }
}


impl Ord for ListItem {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value(a), Value(b)) => a.cmp(b),
            (Value(a), ItemList(b)) => vec![Value(*a)].cmp(b),
            (ItemList(a), Value(b)) => a.cmp(&vec![Value(*b)]),
            (ItemList(a), ItemList(b)) => a.cmp(b)
        }
    }
}

impl PartialOrd for ListItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


enum ParseMode {
    FindRight,
    FindItems,
}

fn parse_list(input: &str) -> ListItem {
    if input.is_empty() {
        return ItemList(vec![]);
    }
    let mut cursor = 0;
    let mut capture_start = 0;
    let mut level = 0;
    let stream: Vec<char> = input.chars().collect();
    let mut parse_mode = FindItems;

    let mut items: Vec<ListItem> = vec![];

    while cursor < stream.len() {
        match parse_mode {
            FindRight => {
                match stream[cursor] {
                    '[' => {
                        level += 1;
                    }
                    ']' => {
                        level -= 1;
                        if level == 0 {
                            items.push(parse_list(&input[capture_start..cursor]));
                            parse_mode = FindItems;
                            capture_start = cursor + 1;
                        }
                    }
                    _ => {}
                }
            }
            FindItems => {
                match stream[cursor] {
                    '[' => {
                        parse_mode = FindRight;
                        capture_start = cursor + 1;
                        level += 1;
                    }
                    ',' | ']' => {
                        if cursor > capture_start {
                            items.push(Value(stream[capture_start..cursor].iter().collect::<String>().parse().unwrap()));
                        }
                        capture_start = cursor + 1;
                    }
                    _ => {}
                }
            }
        }

        cursor += 1;
    }
    if cursor > capture_start {
        items.push(Value(stream[capture_start..cursor].iter().collect::<String>().parse().unwrap()));
    }

    ItemList(items)
}


pub fn run() {}

pub fn run2() {}


#[cfg(test)]
mod test {
    use crate::read_file;
    use super::*;

    #[test]
    fn list_parsing() {
        assert_eq!(ItemList(vec![ItemList(vec![])]), parse_list("[]"));
        assert_eq!(ItemList(vec![ItemList(vec![Value(1)])]), parse_list("[1]"));
        assert_eq!(ItemList(vec![ItemList(vec![Value(1), Value(2)])]), parse_list("[1,2]"));
        assert_eq!(ItemList(vec![ItemList(vec![Value(1), ItemList(vec![Value(1), Value(2)])])]), parse_list("[1,[1,2]]"));
        assert_eq!(ItemList(vec![
            ItemList(
                vec![
                    Value(1),
                    ItemList(vec![
                        Value(2),
                        ItemList(vec![
                            Value(3),
                            ItemList(vec![
                                Value(4),
                                ItemList(vec![
                                    Value(5), Value(6), Value(0),
                                ]),
                            ]),
                        ]),
                    ]),
                    Value(8),
                    Value(9),
                ])
        ]), parse_list("[1,[2,[3,[4,[5,6,0]]]],8,9]"));
    }

    #[test]
    fn less_than() {
        assert_eq!(true, parse_list("[1]") < parse_list("[2]"));
    }

    #[test]
    fn correct_answer() {
        let mut count = 0;
        let input = read_file("input/day13.txt");
        let pairs = input.split("\n\n");
        for (i, pair) in pairs.enumerate() {
            let (left, right) = pair.split_once('\n').unwrap();
            let left = parse_list(left);
            let right = parse_list(right);

            if left < right {
                println!("{}", i + 1);
                count += i + 1;
            }
        }
        println!("{}", count);
    }

    #[test]
    fn correct_answer_two() {
        let mut count = 0;
        let input = read_file("input/day13.txt") + "\n\n[[2]]\n[[6]]";

        let mut packet_list: Vec<ListItem> = vec![];

        for line in input.lines() {
            if !line.is_empty() {
                packet_list.push(parse_list(line))
            }
        }
        packet_list.sort();

        let marker1 = packet_list
            .iter()
            .enumerate()
            .find(|(i, item)| ItemList(vec![ItemList(vec![Value(2)])]) == **item)
            .unwrap();
        let marker2 = packet_list
            .iter()
            .enumerate()
            .find(|(i, item)| ItemList(vec![ItemList(vec![Value(6)])]) == **item)
            .unwrap();


        println!("{:?}", { (marker1.0 + 1) * (marker2.0 + 1) });
    }
}