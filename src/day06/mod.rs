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

pub fn run() {
    println!("Tuning Trouble");
    println!("Unique stream starts at: {}", parse_line(&read_file("input/day06.txt")).unwrap());
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
}