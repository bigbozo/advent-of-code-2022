extern crate core;

use ansi_term::Colour::Yellow;
use std::fmt::{Debug, Display, Formatter};
use std::process::exit;
use std::{fmt, fs};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;

pub fn read_file(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Ok(data) => data,
        Err(_) => {
            println!("Couln't read file {}", filename);
            exit(1);
        }
    }
}

pub fn print_headline(title: &str) {
    println!();
    println!("{}", Yellow.bold().paint(title));
    let line = title.chars().into_iter().map(|_| "=").collect::<String>();
    println!("{}", Yellow.bold().paint(line));
}

#[derive(PartialEq, Clone, Copy, Eq)]
pub struct Point<T> {
    x: T,
    y: T,
}
impl Display for Point<i32> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.x, self.y)
    }
}
impl Debug for Point<usize> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}|{})", self.x, self.y)
    }
}
