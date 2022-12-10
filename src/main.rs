use adventofcode::{day01, day02, day03, day04, day05, day06, day07, day08, day09, print_headline};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// the day for which to run the code
    day: Option<u32>,
}

fn main() {
    let args = Cli::parse();

    run_day(args.day);
}

fn run_day(day: Option<u32>) {
    match day {
        None => {
            for i in 1..10 {
                run_day(Some(i));
            }
        }
        Some(1) => {
            print_headline("Day 01 -         Calorie Counting");
            day01::run();
            day01::run2();
        }
        Some(2) => {
            print_headline("Day 02 -      Rock Paper Scissors");
            day02::run();
            day02::run2();
        }
        Some(3) => {
            print_headline("Day 03 -  Rucksack Reorganization");
            day03::run();
            day03::run2();
        }
        Some(4) => {
            print_headline("Day 04 -             Camp Cleanup");
            day04::run();
            day04::run2();
        }
        Some(5) => {
            print_headline("Day 05 -            Supply Stacks");
            day05::run();
            day05::run2();
        }
        Some(6) => {
            print_headline("Day 06 -           Tuning Trouble");
            day06::run();
            day06::run2();
        }
        Some(7) => {
            print_headline("Day 07 -  No Space Left On Device");
            day07::run();
            day07::run2();
        }
        Some(8) => {
            print_headline("Day 08 -       Treetop Tree House");
            day08::run();
            day08::run2();
        }
        Some(9) => {
            print_headline("Day 09 -              Rope Bridge");
            day09::run();
            day09::run2();
        }
        _ => println!("Value [{}] for day is invalid", day.unwrap()),
    }
}
