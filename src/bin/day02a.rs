use adventofcode::day02::*;

fn main() {
    println!("Hello Elves!");


    let score = adventofcode::day02::calculate_score(
        get_parsed_input("input/day02.txt"),
        score_matrix('X', 'Y', 'Z'),
    );

    println!("{}", score);
}

