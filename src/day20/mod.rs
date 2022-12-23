use std::fmt::{Debug, Formatter};
use crate::read_file;

struct VisitNumber {
    number: i64,
    visited: bool,
    number_in_line: usize,
}

impl Debug for VisitNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.number)
    }
}

fn parse_input(input: &str) -> Vec<VisitNumber> {
    let input = read_file(input);
    let mut counter: usize = 0;

    input.lines().map(|c| {
        counter += 1;
        VisitNumber {
            number: c.parse().unwrap(),
            visited: false,
            number_in_line: counter,
        }
    }).collect::<Vec<VisitNumber>>()
}

pub fn run() {
    let mut numbers = parse_input("input/day20.txt");
    let mut cursor: i64 = 0;
    let len = (numbers.len() - 1) as i64;
    let mut to_go = len + 1;

    while to_go > 0 {
        if numbers[cursor as usize].visited {
            cursor += 1;
            continue;
        }
        if numbers[cursor as usize].number != 0 {
            let mut number = numbers.remove(cursor as usize);
            let mut index = cursor + number.number;
            if index < 0 {
                index = (len + index % len) % len;
            } else if index >= len {
                index = index % len;
            }
            number.visited = true;
            to_go -= 1;
            if index == len {
                numbers.push(number);
            } else {
                numbers.insert(index as usize, number);
            }
            if index < cursor {
                cursor += 1;
            }
        } else {
            numbers[cursor as usize].visited = true;
            to_go -= 1;
            cursor += 1;
        }
    }

    let zero_position = numbers.iter().position(|c| c.number == 0).unwrap();
    println!("Result: {}",
             numbers[(zero_position + 1000) % numbers.len()].number +
                 numbers[(zero_position + 2000) % numbers.len()].number +
                 numbers[(zero_position + 3000) % numbers.len()].number
    );
}

pub fn run2() {
    let mut numbers = parse_input("input/day20.txt");
    let len: i64 = (numbers.len() - 1) as i64;
    for number in numbers.iter_mut() {
        number.number *= 811589153;
        //number.number = number.number % len;
    }
    println!("{:?}", numbers);
    for _ in 0..10 {
        for number in numbers.iter_mut() {
            number.visited = false;
        }
        for i in 1..=numbers.len() {
            let cursor = numbers.iter().position(|c| c.number_in_line == i).unwrap();

            numbers[cursor].visited = true;

            if numbers[cursor].number != 0 {
                let number = numbers.remove(cursor);
                let mut index: i64 = cursor as i64 + number.number;
                if index < 0 {
                    index = (len + index % len) % len;
                } else if index >= len {
                    index = index % len;
                }
                if index == len {
                    numbers.push(number);
                } else {
                    numbers.insert(index as usize, number);
                }
            }
        }
        println!("{:?}", numbers);
    }
    println!("{:?}", numbers);

    let zero_position = numbers.iter().position(|c| c.number == 0).unwrap();
    println!("Result: {}",
             numbers[(zero_position + 1000) % numbers.len()].number +
                 numbers[(zero_position + 2000) % numbers.len()].number +
                 numbers[(zero_position + 3000) % numbers.len()].number
    );
}
