use std::cmp::max;
use adventofcode::read_file;

fn main() {
    println!("Hello Elves!");

    let mut max_count = 0;

    let input = read_file("input/day01-01.txt");
    let chunks = input.trim().split("\n\n");


    for chunk in chunks {

        let items = chunk.split("\n");

        let mut count = 0;

        for item in items {
            count += match item.parse() {
                Ok(t) => t,
                Err(_) => {
                    println!("No integer found");
                    0
                }
            };
        }

        max_count = max(max_count, count);
    }

    println!("{}", max_count);
}