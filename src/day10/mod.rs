use ansi_term::{Colour};
use ansi_term::Colour::{Blue, Yellow};
use crate::day10::Op::{AddX, Noop};
use crate::read_file;

#[derive(Debug, PartialEq, Eq)]
pub enum Op {
    Noop,
    AddX(i32),
}

type Program = Vec<Op>;


type Screen = [[char; 40]; 6];

fn parse_line(input: &str) -> Option<Op> {
    match input[0..4].as_ref() {
        "noop" => Some(Noop),
        _ => {
            let (command, param) = input.split_once(' ').unwrap();
            match command {
                "addx" => Some(AddX(param.parse().unwrap())),
                _ => {
                    println!("No such command: {}", command);
                    None
                }
            }
        }
    }
}

fn parse_program(input: String) -> Program {
    let mut token_stream: Program = vec![];
    for line in input.lines() {
        let op = parse_line(line).unwrap();
        match op {
            Noop => token_stream.push(Noop),
            AddX(_) => {
                token_stream.push(Noop);
                token_stream.push(op);
            }
        }
    }
    token_stream
}

pub fn execute_program(program: Program) -> i32 {
    let mut x = 1;
    let mut signal_strength = 0;

    for (i, item) in program.iter().enumerate() {
        // start of cycle

        // during cycle
        if (i + 21) % 40 == 0 {
            signal_strength += (i + 1) as i32 * x;
        }

        // after cycle
        match item {
            AddX(value) => {
                x += value;
            }
            Noop => {}
        }
    }
    signal_strength
}

pub fn draw_screen(program: Program) -> Screen {
    let mut crt: Screen = [['.'; 40]; 6];

    let mut x: i32 = 1;


    for (i,item) in program.iter().enumerate() {
        let screen_x = i % 40;
        let screen_y = (i - screen_x) / 40;

        if (x - screen_x as i32).abs() < 2 {
            crt[screen_y][screen_x] = '#';
        } else {
            crt[screen_y][screen_x] = '.';
        }

        // after cycle
        match item {
            AddX(value) => {
                x += value;
            }
            Noop => {}
        }
    }
    crt
}

pub fn run() {
    let program = parse_program(read_file("input/day10.txt"));
    println!("Sum of signal strengths is {}", Yellow.bold().paint(execute_program(program).to_string()));
}

pub fn run2() {
    let program = parse_program(read_file("input/day10.txt"));
    let screen = draw_screen(program);


    println!("Screen output of program:");
    for scan_line in screen {
        for pixel in scan_line {
            match pixel {
                '#' => print!("{}", Yellow.bold().on(Colour::Yellow).paint(" ")),
                _ => print!("{}", Blue.bold().on(Colour::Blue).paint(" "))
            }
        }
        println!();
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_input_works() {
        assert_eq!(AddX(3), parse_line("addx 3").unwrap());
        assert_eq!(AddX(-5), parse_line("addx -5").unwrap());
        assert_eq!(Noop, parse_line("noop").unwrap());
    }

    #[test]
    fn parse_program_works() {
        assert_eq!(vec![Noop, Noop, AddX(3), Noop, AddX(-5)], parse_program("noop
addx 3
addx -5".to_string()));
    }

    #[test]
    fn run_program() {
        let program = parse_program("noop
addx 3
addx -5".to_string());

        assert_eq!(0, execute_program(program));
    }

    #[test]
    fn calculation_works() {
        let program = parse_program(read_file("input/day10-test.txt"));
        assert_eq!(13140, execute_program(program));
    }
}