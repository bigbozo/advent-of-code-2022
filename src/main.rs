use adventofcode::print_headline;
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
            adventofcode::day01::run();
            adventofcode::day01::run2();
        }
        Some(2) => {
            print_headline("Day 02 -      Rock Paper Scissors");
            adventofcode::day02::run();
            adventofcode::day02::run2();
        }
        Some(3) => {
            print_headline("Day 03 -  Rucksack Reorganization");
            adventofcode::day03::run();
            adventofcode::day03::run2();
        }
        Some(4) => {
            print_headline("Day 04 -             Camp Cleanup");
            adventofcode::day04::run();
            adventofcode::day04::run2();
        }
        Some(5) => {
            print_headline("Day 05 -            Supply Stacks");
            adventofcode::day05::run();
            adventofcode::day05::run2();
        }
        Some(6) => {
            print_headline("Day 06 -           Tuning Trouble");
            adventofcode::day06::run();
            adventofcode::day06::run2();
        }
        Some(7) => {
            print_headline("Day 07 -  No Space Left On Device");
            adventofcode::day07::run();
            adventofcode::day07::run2();
        }
        Some(8) => {
            print_headline("Day 08 -       Treetop Tree House");
            adventofcode::day08::run();
            adventofcode::day08::run2();
        }
        Some(9) => {
            print_headline("Day 09 -              Rope Bridge");
            adventofcode::day09::run();
            adventofcode::day09::run2();
        }
        _ => println!("Value [{}] for day is invalid", day.unwrap()),
    }
}
