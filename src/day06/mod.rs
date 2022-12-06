use itertools::Itertools;
use crate::read_file;


pub fn validate(input: &str) -> bool {
    let vec: Vec<char> = input.chars().into_iter().unique().collect();

    vec.len() == input.len()

}

pub fn parse_line(input: &str) -> Option<usize> {
    for i in 0..input.len()-4 {
        if validate(&input[i..i+4]) {
            return Some(i+4);
        }
    }
    None
}
pub fn parse_line_with_packet_size(input: &str, packetsize: usize) -> Option<usize> {
    if packetsize > input.len() {
        return None;
    }
    for i in 0..input.len()-packetsize {
        if validate(&input[i..i+packetsize]) {
            return Some(i+packetsize);
        }
    }
    None
}

pub fn run() {
    println!("Tuning Trouble");
    println!("Starting packet marker starts at: {}", parse_line(&read_file("input/day06.txt")).unwrap());
}
pub fn run2() {
    println!("... and for the messaage: ");
    println!("Message stream starts at: {}", parse_line_with_packet_size(&read_file("input/day06.txt"),14).unwrap());
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
        assert_eq!(19, parse_line_with_packet_size("mjqjpqmgbljsphdztnvjfqwrcgsmlb",14).unwrap());
        assert_eq!(23, parse_line_with_packet_size("bvwbjplbgvbhsrlpgdmjqwftvncz",14).unwrap());
        assert_eq!(23, parse_line_with_packet_size("nppdvjthqldpwncqszvftbrmjlhg",14).unwrap());
        assert_eq!(29, parse_line_with_packet_size("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",14).unwrap());
        assert_eq!(26, parse_line_with_packet_size("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",14).unwrap());
    }
}