use ansi_term::Colour::Yellow;
use crate::read_file;
use itertools::Itertools;

pub fn validate(input: &str) -> bool {
    input.chars().into_iter().unique().collect::<String>().len() == input.len()
}

pub fn parse_line(input: &str) -> Option<usize> {
    for i in 0..input.len() - 4 {
        if validate(&input[i..i + 4]) {
            return Some(i + 4);
        }
    }
    None
}

pub fn parse_line_with_packet_size(input: &str, packetsize: usize) -> Option<usize> {
    if packetsize > input.len() {
        return None;
    }
    for i in 0..input.len() - packetsize {
        if validate(&input[i..i + packetsize]) {
            return Some(i + packetsize);
        }
    }
    None
}

pub fn run() {
    println!("Tuning Trouble");
    let answer = parse_line(&read_file("input/day06.txt")).unwrap();
    println!(
        "Starting packet marker starts at: {}",
        Yellow.bold().paint(format!("{}", answer)));
}

pub fn run2() {
    println!("... and for the messaage: ");
    let answer = parse_line_with_packet_size(&read_file("input/day06.txt"), 14).unwrap();
    println!(
        "Message stream starts at: {}",
        Yellow.bold().paint(format!("{}", answer)));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pattern_validator_works() {
        assert!(validate("jpqm"));
        assert_eq!(false, validate("jpqj"));
    }

    #[test]
    fn parse_line_workd() {
        assert_eq!(7, parse_line("mjqjpqmgbljsphdztnvjfqwrcgsmlb").unwrap());
        assert_eq!(5, parse_line("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap());
        assert_eq!(6, parse_line("nppdvjthqldpwncqszvftbrmjlhg").unwrap());
        assert_eq!(10, parse_line("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap());
        assert_eq!(11, parse_line("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap());
    }

    #[test]
    fn parse_line_with_packetsize_works() {
        assert_eq!(
            19,
            parse_line_with_packet_size("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14).unwrap()
        );
        assert_eq!(
            23,
            parse_line_with_packet_size("bvwbjplbgvbhsrlpgdmjqwftvncz", 14).unwrap()
        );
        assert_eq!(
            23,
            parse_line_with_packet_size("nppdvjthqldpwncqszvftbrmjlhg", 14).unwrap()
        );
        assert_eq!(
            29,
            parse_line_with_packet_size("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14).unwrap()
        );
        assert_eq!(
            26,
            parse_line_with_packet_size("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14).unwrap()
        );
    }
}
