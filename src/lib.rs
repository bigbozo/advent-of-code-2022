use std::fs;
use std::process::exit;

pub mod day01;

pub fn read_file(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Ok(data) => data,
        Err(_) => {
            println!("Couln't read file {}",filename);
            exit(1);
        }
    }
}