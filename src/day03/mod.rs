use super::read_file;
use std::collections::HashMap;

fn priority(a: char) -> u32 {
    let priority = a as u32;

    if priority < 97 {
        priority - 38
    } else {
        priority - 96
    }
}

fn get_common_char(items: &str) -> Option<char> {

    let count = items.chars().count() / 2;
    let mut letters: HashMap<char, u32> = HashMap::new();
    // Build a hashmap from the first half of letter
    for letter in items[..count].chars() {
        letters.entry(letter).and_modify(|count| *count += 1).or_insert(1);
    }
    // check for letters already used
    for letter in items[count..].chars() {
        if letters.contains_key(&letter) {
            return Some(letter);
        }
    }

    None
}

pub fn priority_sum(input: String) -> u32 {
    input.lines().map(|line| priority(get_common_char(line).unwrap())).sum()
}

pub fn priority_for_file(filename: &str) -> u32 {
    priority_sum(read_file(filename))
}

fn unique_items(items: &str) -> HashMap<char, u32> {
    let mut letters: HashMap<char, u32> = HashMap::new();

    for letter in items.chars() {
        letters.entry(letter).or_insert(1);
    }
    letters
}

fn add_items(mut map1: HashMap<char, u32>, map2: HashMap<char, u32>) -> HashMap<char, u32> {
    for letter in map2.keys() {
        map1.entry(*letter).and_modify(|count| *count += 1).or_insert(1);
    }

    map1
}

pub fn find_valid_badge(backpacks: &[&str]) -> Option<char> {
    let mut items = unique_items(backpacks[0]);
    let items2 = unique_items(backpacks[1]);
    let items3 = unique_items(backpacks[2]);

    items = add_items(items, items2);
    items = add_items(items, items3);


    items.iter().find_map(|(key, &val)| if val == 3 { Some(*key) } else { None })
}

pub fn calculate_group_sums_for_file(filename: &str) -> u32 {
    let input = read_file(filename);
    let mut sum = 0;

    for item in input.lines().collect::<Vec<&str>>().chunks(3) {
        sum += priority(find_valid_badge(item).unwrap())
    };

    sum
}

pub fn run() {
    println!("The Rucksacks are wrong Packed!");

    let score = priority_for_file("input/day03.txt");

    println!("This is the priority score: {}", score);
}

pub fn run2() {
    println!("Oh no! What's wrong with the badges???");

    let score = calculate_group_sums_for_file("input/day03.txt");

    println!("Corrected Badge-score: {}", score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn priority_is_correct() {
        assert_eq!(1, priority('a'));
        assert_eq!(26, priority('z'));
        assert_eq!(27, priority('A'));
        assert_eq!(52, priority('Z'));
    }

    #[test]
    fn finds_common_items() {
        assert_eq!(Some('p'), get_common_char("vJrwpWtwJgWrhcsFMMfFFhFp"));
        assert_eq!(Some('L'), get_common_char("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"));
        assert_eq!(Some('P'), get_common_char("PmmdzqPrVvPwwTWBwg"));
        assert_eq!(Some('v'), get_common_char("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"));
        assert_eq!(Some('t'), get_common_char("ttgJtRGJQctTZtZT"));
        assert_eq!(Some('s'), get_common_char("CrZsJsPPZsGzwwsLwLmpwMDw"));
        assert_eq!(Some('D'), get_common_char("FVzJtDDJDqTMlmlM"));
    }

    #[test]
    fn calculates_correct_priority_sum() {
        assert_eq!(157, priority_sum(read_file("input/day03-test.txt")));
    }

    #[test]
    fn calculates_correct_priority_sum_for_file() {
        assert_eq!(157, priority_for_file("input/day03-test.txt"));
    }

    #[test]
    fn finds_valid_badges() {
        assert_eq!(Some('c'), find_valid_badge(&["abc", "bcd", "cde"]));
        assert_eq!(Some('r'), find_valid_badge(&["vJrwpWtwJgWrhcsFMMfFFhFp", "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", "PmmdzqPrVvPwwTWBwg"]));
        assert_eq!(Some('Z'), find_valid_badge(&["wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", "ttgJtRGJQctTZtZT", "CrZsJsPPZsGzwwsLwLmpwMDw"]));
    }

    #[test]
    fn calculates_correct_sum() {
        assert_eq!(70, calculate_group_sums_for_file("input/day03-test.txt"));
    }
}
