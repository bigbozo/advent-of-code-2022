use std::fs;
use std::process::exit;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;

pub fn read_file(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Ok(data) => data,
        Err(_) => {
            println!("Couln't read file {}",filename);
            exit(1);
        }
    }
}

pub fn print_headline(title: &str) {
    println!();
    println!("{}", title);
    println!("======");
}
