extern crate core;

use std::fs;
use std::process::exit;
use ansi_term::Colour::{Yellow};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;

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
    println!("{}",Yellow.bold().paint(line));
}
