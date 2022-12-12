use crate::read_file;
use ansi_term::Colour::Yellow;

#[derive(PartialEq, Debug)]
struct Range {
    min: i32,
    max: i32,
}

impl Range {
    pub fn new(range: &str) -> Range {
        let mut split = range.split('-');
        let min: i32 = split.next().unwrap().parse().unwrap();
        let max: i32 = split.next().unwrap().parse().unwrap();

        if min > max {
            Range { min: max, max: min }
        } else {
            Range { min, max }
        }
    }

    pub fn contains(&self, b: &Range) -> bool {
        self.min <= b.min && self.max >= b.max
    }
    pub fn overlap(&self, b: &Range) -> bool {
        self.min <= b.max && self.max >= b.min
    }
}

fn get_ranges(ranges: &str) -> (Range, Range) {
    let mut split = ranges.split(',');

    let left = Range::new(split.next().unwrap());
    let right = Range::new(split.next().unwrap());

    (left, right)
}

pub fn calculate(filename: &str) -> i32 {
    let input = read_file(filename);

    input
        .lines()
        .map(|line| {
            let (left, right) = get_ranges(line);
            if left.contains(&right) {
                return 1;
            }
            if right.contains(&left) {
                return 1;
            }
            0
        })
        .sum()
}

pub fn calculate2(filename: &str) -> i32 {
    let input = read_file(filename);

    input
        .split('\n')
        .map(|line| {
            let (left, right) = get_ranges(line);
            if left.overlap(&right) {
                return 1;
            }
            0
        })
        .sum()
}

pub fn run() {
    println!("Camp Cleanup");
    let count = calculate("input/day04.txt");
    println!(
        "There are {} doubled ranges",
        Yellow.bold().paint(format!("{}", count))
    );
}

pub fn run2() {
    println!("This wasn't extensive enough!");
    let count = calculate2("input/day04.txt");
    println!(
        "There are {} overlapping ranges",
        Yellow.bold().paint(format!("{}", count))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_is_correct_parsed() {
        assert_eq!(Range { min: 2, max: 4 }, Range::new("2-4"));
        assert_eq!(Range { min: 6, max: 8 }, Range::new("8-6"));
    }

    #[test]
    fn line_is_correct_parsed() {
        assert_eq!(
            (Range { min: 2, max: 4 }, Range { min: 6, max: 8 }),
            get_ranges("2-4,6-8")
        );
    }

    #[test]
    fn range_is_contained() {
        let a = Range { min: 1, max: 2 };
        let b = Range { min: 1, max: 3 };
        assert!(b.contains(&a));
    }

    #[test]
    fn range_is_not_contained() {
        let a = Range { min: 1, max: 2 };
        let b = Range { min: 1, max: 3 };
        assert_eq!(false, a.contains(&b));
    }

    #[test]
    fn test_data_produced_correct_result() {
        assert_eq!(2, calculate("input/day04-test.txt"));
    }

    #[test]
    fn test_data_produced_correct_result_for_part_two() {
        assert_eq!(4, calculate2("input/day04-test.txt"));
    }
}
