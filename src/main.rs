use clap::Parser;
use adventofcode::print_headline;

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
        None => for i in 1..6 {
            run_day(Some(i));
        },
        Some(1) => {
            print_headline("Day 01");
            adventofcode::day01::run();
            adventofcode::day01::run2();
        }
        Some(2) => {
            print_headline("Day 02");
            adventofcode::day02::run();
            adventofcode::day02::run2();
        }
        Some(3) => {
            print_headline("Day 03");
            adventofcode::day03::run();
            adventofcode::day03::run2();
        }
        Some(4) => {
            print_headline("Day 04");
            adventofcode::day04::run();
            adventofcode::day04::run2();
        }
        Some(5)=> {
            print_headline("Day 05");
            adventofcode::day05::run();
            adventofcode::day05::run2();
        }
        Some(6)=> {
            print_headline("Day 06");
            adventofcode::day06::run();
            adventofcode::day06::run2();
        }
        _ => println!("Value [{}] for day is invalid", day.unwrap()),
    }
}

