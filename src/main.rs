use adventofcode::{day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13, day14, day15, day16, day17, day18, day19, print_headline};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// the day for which to run the code
    day: Option<u32>,
}

fn main() {
    let args = Cli::parse();

    run_day(args.day, false);
}

fn run_day(day: Option<u32>, run_all: bool) {
    match day {
        None => {
            for i in 1..20 {
                if i!=16 {
                    run_day(Some(i),true);
                } else {
                    println!("Day 16 only solo, too long");
                }
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
        Some(10) => {
            print_headline("Day 10 -         Cathode-Ray Tube");
            day10::run();
            day10::run2();
        }
        Some(11) => {
            print_headline("Day 11 -     Monkey in the Middle");
            day11::run();
            day11::run2();
        }
        Some(12) => {
            print_headline("Day 12 -  Hill Climbing Algorithm");
            day12::run();
            day12::run2();
        }
        Some(13) => {
            print_headline("Day 13 -          Distress Signal");
            day13::run();
            day13::run2();
        }
        Some(14) => {
            print_headline("Day 14 -       Regolith Reservoir");
            day14::run();
            day14::run2();
        }
        Some(15) => {
            print_headline("Day 15 -    Beacon Exclusion Zone");
            day15::run();
            if !run_all {
                day15::run2();
            } else {
                println!("Takes too long! Start day 15");
            }
        }
        Some(16) => {
            print_headline("Day 16 -    Proboscidea Volcanium");
            day16::run();
            day16::run2();
        }
        Some(17) => {
            print_headline("Day 17 -         Pyroclastic Flow");
            day17::run();
            if !run_all {
                day17::run2();
            } else {
                println!("Takes too long! Start day 17");
            }
        }
        Some(18) => {
            print_headline("Day 18 -        Boiling Boulders");
            day18::run();
            day18::run2();
        }
        Some(19) => {
            print_headline("Day 19 -     Not Enough Minerals");
            day19::run();
            day19::run2();
        }
        _ => println!("Value [{}] for day is invalid", day.unwrap()),
    }
}
