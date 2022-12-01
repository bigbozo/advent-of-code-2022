use super::read_file;

pub fn sum_per_elf(filename: &str) -> Vec<i32> {
    let mut counts: Vec<i32> = Vec::new();

    let input = read_file(filename);
    let chunks = input.trim().split("\n\n");

    for chunk in chunks {
        let items = chunk.split("\n");

        let mut count = 0;

        for item in items {
            count += match item.parse() {
                Ok(t) => t,
                Err(_) => {
                    println!("No integer found");
                    0
                }
            };
        }

        counts.push(count);
    }


    counts
}

pub fn max_count(counts: Vec<i32>) -> i32 {
    match counts.iter().max() {
        Some(max) => * max,
        None => {
            println!("Should have a maximum value. Did you input values?");
            0
        }
    }
}

pub fn calculate_best(filename:&str)  -> i32 {

    let mut counts = sum_per_elf(filename);
    counts.sort();

    max_count(counts)
}

pub fn calculate_best3(filename: &str) ->i32 {

    let mut counts = sum_per_elf(filename);
    counts.sort();

    counts.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_per_elf_matches() {
        let counts = sum_per_elf("input/day01-test.txt");
        assert_eq!(6000, counts[0]);
        assert_eq!(4000, counts[1]);
        assert_eq!(11000, counts[2]);
        assert_eq!(24000, counts[3]);
        assert_eq!(10000, counts[4]);
    }
    #[test]
    fn max_count_works() {
        assert_eq!(24000,max_count(vec![24000,10000,4000]));
    }
    #[test]
    fn max_count_works_for_empty_vector() {
        assert_eq!(0,max_count(vec![]));
    }
    #[test]
    fn max_sum_gets_returned() {
        let counts = sum_per_elf("input/day01-test.txt");
        assert_eq!(24000,max_count(counts));
    }
    #[test]
    fn calculate_best_works() {
        assert_eq!(24000,calculate_best("input/day01-test.txt"));
    }
    #[test]
    fn calculate_best3_works() {
        assert_eq!(45000,calculate_best3("input/day01-test.txt"));
    }

}