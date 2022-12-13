use crate::day13::ListItem::{ItemList, Value, Empty};
use crate::day13::ParseMode::{FindItems, FindRight};

enum ListItem {
    ItemList(Vec<ListItem>),
    Value(u32),
    Empty,
}

impl ListItem {
    pub fn less_than(&self, item: ListItem) -> bool {
        match self {
            ItemList(a) =>
                match item {
                    ItemList(b) => { a.less_than(b)}
                    Value(b) => { a.less_than}
                    Empty => { false }
                }
            Value(a) => {
                match item {
                    ItemList(_) => {}
                    Value(b) => { b > *a }
                    Empty => {}
                }
            }
            Empty => {
                true
            }
        }
        false
    }
}

enum ParseMode {
    FindRight,
    FindItems,
}

fn parse_list(input: &str) -> ListItem {
    if input.len() == 0 {
        return ItemList(vec![Empty]);
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
                        level = level + 1;
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
    use super::*;

    #[test]
    fn list_parsing() {
        assert_eq!(ItemList(vec![ItemList(vec![Empty])]), parse_list("[]"));
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
}