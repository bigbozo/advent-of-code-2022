use adventofcode::read_file;
use std::process::exit;

fn main() {
    println!("Hello Elves!");

    let mut counts: Vec<i32> = Vec::new();

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

        counts.push(count);
    }

    counts.sort();

    let sum: i32 = counts.iter().rev().take(3).sum();

    println!("{}", sum);
}