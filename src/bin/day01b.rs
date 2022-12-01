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
    counts.reverse();
    let mut sum = 0;

    let chunk = match counts.chunks(3).nth(0) {
        Some(t) => t,
        None => {
            println!("Should have a chunk of 3");
            exit(1);
        }
    };
    for item in chunk {
        sum += item;
    }

    println!("{}", sum);
}