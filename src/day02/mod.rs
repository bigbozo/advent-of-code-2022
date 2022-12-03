use std::collections::HashMap;
use super::read_file;

pub fn get_parsed_input(filename: &str) -> HashMap<String, u32> {
    let mut hash_map = HashMap::new();
    let input: String = read_file(filename);
    for line in input.lines() {
        if !line.is_empty() {
            hash_map.entry(line.to_owned()).and_modify(|count| *count += 1).or_insert(1);
        }
    }
    hash_map
}

fn build_pattern(a: char, b: char) -> String {
    format!("{a} {b}")
}

pub fn score_matrix(rock_b: char, paper_b: char, scissors_b: char) -> HashMap<String, u32> {
    let mut hash_map = HashMap::new();

    let rock_a = 'A';
    let paper_a = 'B';
    let scissors_a = 'C';

    let rock_base = 1;
    let paper_base = 2;
    let scissors_base = 3;

    let win = 6;
    let loss = 0;
    let draw = 3;

    hash_map.insert(build_pattern(rock_a, rock_b), rock_base + draw);
    hash_map.insert(build_pattern(rock_a, paper_b), paper_base + win);
    hash_map.insert(build_pattern(rock_a, scissors_b), scissors_base + loss);

    hash_map.insert(build_pattern(paper_a, rock_b), rock_base + loss);
    hash_map.insert(build_pattern(paper_a, paper_b), paper_base + draw);
    hash_map.insert(build_pattern(paper_a, scissors_b), scissors_base + win);

    hash_map.insert(build_pattern(scissors_a, rock_b), rock_base + win);
    hash_map.insert(build_pattern(scissors_a, paper_b), paper_base + loss);
    hash_map.insert(build_pattern(scissors_a, scissors_b), scissors_base + draw);

    hash_map
}

pub fn score_matrix2(win_b: char, draw_b: char, lose_b: char) -> HashMap<String, u32> {
    let mut hash_map = HashMap::new();

    let rock_a = 'A';
    let paper_a = 'B';
    let scissors_a = 'C';

    let rock_base = 1;
    let paper_base = 2;
    let scissors_base = 3;

    let win = 6;
    let loss = 0;
    let draw = 3;

    hash_map.insert(build_pattern(rock_a, win_b), paper_base + win);
    hash_map.insert(build_pattern(rock_a, draw_b), rock_base + draw);
    hash_map.insert(build_pattern(rock_a, lose_b), scissors_base + loss);

    hash_map.insert(build_pattern(paper_a, win_b), scissors_base + win);
    hash_map.insert(build_pattern(paper_a, draw_b), paper_base + draw);
    hash_map.insert(build_pattern(paper_a, lose_b), rock_base + loss);

    hash_map.insert(build_pattern(scissors_a, win_b), rock_base + win);
    hash_map.insert(build_pattern(scissors_a, draw_b), scissors_base + draw);
    hash_map.insert(build_pattern(scissors_a, lose_b), paper_base + loss);

    hash_map
}

pub fn calculate_score(result_map: HashMap<String, u32>, score_map: HashMap<String, u32>) -> u32 {
    let mut score = 0;

    for key in result_map.keys() {
        score += *result_map.get(key).unwrap() * *score_map.get(key).unwrap();
    }

    score
}

pub fn run() {
    println!("Rock Paper Scissors!");

    let score = calculate_score(
        get_parsed_input("input/day02.txt"),
        score_matrix('X', 'Y', 'Z'),
    );

    println!("Your totaled score: {}", score);
}

pub fn run2() {
    println!("Let's be unconspicuous!");

    let score = calculate_score(
        get_parsed_input("input/day02.txt"),
        score_matrix2('Z', 'Y', 'X'),
    );

    println!("You're adapted score is {}.", score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_parsing_works() {
        let hash_map = get_parsed_input("input/day02-test.txt");
        assert_eq!(3, hash_map.len());
    }

    #[test]
    fn score_matrix_is_correct() {
        let score_matrix = score_matrix('X', 'Y', 'Z');
        assert_eq!(8, *score_matrix.get("A Y").unwrap());
        assert_eq!(1, *score_matrix.get("B X").unwrap());
        assert_eq!(6, *score_matrix.get("C Z").unwrap());
    }

    #[test]
    fn total_score_is_correct() {
        assert_eq!(15, calculate_score(
            get_parsed_input("input/day02-test.txt"),
            score_matrix('X', 'Y', 'Z'),
        ));
    }

    #[test]
    fn total_score_is_correct_for_b() {
        assert_eq!(12, calculate_score(
            get_parsed_input("input/day02-test.txt"),
            score_matrix2('Z', 'Y', 'X'),
        ));
    }
}

