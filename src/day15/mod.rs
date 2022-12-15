use crate::read_file;
use std::cmp::{max, min};
use regex::Regex;

#[derive(Debug, PartialEq)]
struct Pair {
    sensor: (i64, i64),
    beacon: (i64, i64),
}

impl Pair {
    fn manhattan(&self) -> i64 {
        (self.beacon.1 - self.sensor.1).abs() + (self.beacon.0 - self.sensor.0).abs()
    }
    pub fn visible_at_line(&self, line: i64) -> Option<(i64, i64)> {
        let width_at_line = self.manhattan() - (line - self.sensor.1).abs();
        if width_at_line < 1 {
            return None;
        }
        return Some((self.sensor.0 - width_at_line, self.sensor.0 + width_at_line));
    }
}

pub fn interval_union(mut intervals: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut interval_union: Vec<(i64, i64)> = vec![];

    intervals.sort_by(|a, b| a.0.cmp(&b.0));

    for t in intervals.iter() {
        if interval_union.len() == 0 {
            interval_union.push(t.to_owned());
        }
        let mut found = false;
        for i in 0..interval_union.len() {
            let ut = interval_union[i];
            if t.0 >= ut.0 && t.0 <= ut.1 {
                interval_union[i].1 = max(t.1, ut.1);
                found = true;
                break;
            } else if t.1 <= ut.1 && t.1 >= ut.0 {
                interval_union[i].0 = min(t.0, ut.0);
                found = true;
                break;
            }
        }
        if !found {
            interval_union.push(t.to_owned());
        }
    }
    interval_union
}

fn intervals_at_line(pairs: &Vec<Pair>, line: i64) -> Vec<(i64, i64)> {
    let intervals: Vec<Option<(i64, i64)>> = pairs
        .into_iter()
        .map(|pair| pair.visible_at_line(line))
        .collect::<Vec<Option<(i64, i64)>>>();

    let intervals: Vec<(i64, i64)> = intervals
        .into_iter()
        .filter(|i| i.is_some())
        .map(|i| i.unwrap())
        .collect();
    intervals
}

fn count_intervals(interval: Vec<(i64, i64)>) -> i64 {
    interval
        .iter()
        .map(|i| i.1 - i.0 + 1)
        .sum()
}

fn count_intervals_with_bounds(interval: Vec<(i64, i64)>, left: i64, right: i64) -> i64 {
    let clipped = interval
        .iter()
        .filter(|i| i.0 <= right && i.1 >= left)
        .map(|i| (max(i.0, left), min(i.1, right)))
        .collect::<Vec<(i64, i64)>>();
    count_intervals(clipped)
}

fn parse_line(line: &str) -> Pair {
    let regex = Regex::new(r"Sensor at x=([\d-]+), y=([\d-]+): closest beacon is at x=([\d-]+), y=([\d-]+)").unwrap();
    let cap = regex.captures(line).unwrap();

    Pair {
        sensor: (cap[1].parse::<i64>().unwrap(), cap[2].parse::<i64>().unwrap()),
        beacon: (cap[3].parse::<i64>().unwrap(), cap[4].parse::<i64>().unwrap()),
    }
}

fn parse_input(input: &str) -> Vec<Pair> {
    let mut pairs: Vec<Pair> = vec!();
    for line in input.lines() {
        pairs.push(parse_line(line));
    }
    pairs
}


pub fn run() {
    let pairs = parse_input(&read_file("input/day15.txt"));

    let intervals = intervals_at_line(&pairs, 2_000_000);

    println!("{} Positions cannot hold a beacon", count_intervals(interval_union(intervals)));
}

// FIXME: this is impossibly slow
pub fn run2() {
    let pairs = parse_input(&read_file("input/day15.txt"));
    let range = 4_000_000;
    let mut min_count = i64::MAX;
    let mut y = 0;
    for i in 0..range {
        let intervals = intervals_at_line(&pairs, i);
        let vec = interval_union(intervals);
        let count = count_intervals_with_bounds(vec, 0, range);
        if count < min_count {
            min_count = count;
            y = i;
        }
    }
    let intervals = interval_union(intervals_at_line(&pairs, y));
    println!("This is your frequency: {}",y + range * (intervals[0].1+1));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn parses_line() {
        assert_eq!(Pair { sensor: (1, 2), beacon: (-3, 4) }, parse_line("Sensor at x=1, y=2: closest beacon is at x=-3, y=4"));
    }

    #[test]
    pub fn parses_input() {
        assert_eq!(vec![Pair { sensor: (2, 18), beacon: (-2, 15) }, Pair { sensor: (9, 16), beacon: (10, 16) }], parse_input("Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16"))
    }

    #[test]
    pub fn computes_interval() {
        let pair = parse_line("Sensor at x=8, y=7: closest beacon is at x=2, y=10");
        assert_eq!((2, 14), pair.visible_at_line(10).unwrap());
    }

    #[test]
    pub fn computes_intervals() {
        let pairs = parse_input(&read_file("input/day15-test.txt"));

        let intervals = intervals_at_line(&pairs, 11);

        assert_eq!(vec![(-3, 13), (15, 25)], interval_union(intervals));
    }

    #[test]
    pub fn computes_impossible() {
        let pairs = parse_input(&read_file("input/day15-test.txt"));

        let intervals = intervals_at_line(&pairs, 10);

        assert_eq!(26, count_intervals(interval_union(intervals)));
    }

    #[test]
    pub fn scan_lines() {
        let pairs = parse_input(&read_file("input/day15-test.txt"));
        for i in 0..21 {
            let intervals = intervals_at_line(&pairs, i);
            let vec = interval_union(intervals);
            println!("{:?}", vec);
            println!("{}", count_intervals_with_bounds(vec, 0, 20));
        }
    }
}