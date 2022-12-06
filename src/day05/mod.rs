use crate::read_file;
use regex::Regex;

#[derive(PartialEq, Debug)]
struct Stack {
    items: Vec<char>,
}

#[derive(Debug)]
struct Stacks {
    items: Vec<Stack>,
}

#[derive(PartialEq, Debug)]
struct Instruction {
    from: usize,
    to: usize,
    count: usize,
}


impl Stacks {
    pub fn add(&mut self, stacks: Stacks) {
        for i in 0..stacks.items.len() {
            for char in &stacks.items[i].items {
                self.items[i].items.push(*char);
            }
        }
    }


    pub fn mv(&mut self, from: usize, to: usize) {
        let from = self.items[from].items.pop().unwrap().to_owned();
        let to = &mut self.items[to];
        to.items.push(from);
    }

    pub fn mv_stack(&mut self, from: usize, to: usize, stack_size: usize) {
        let mut swap: Vec<char> = Vec::new();

        for _ in 0..stack_size {
            swap.push(self.items[from].items.pop().unwrap().to_owned());
        }

        for _ in 0..stack_size {
            self.items[to].items.push(swap.pop().unwrap());
        }
    }


    pub fn perform_instructions(&mut self, instructions: String) {
        for instruction in instructions.lines() {
            let instruction = parse_instruction(instruction);
            for _i in 0..instruction.count {
                self.mv(instruction.from - 1, instruction.to - 1);
            }
        }
    }

    pub fn perform_instructions9001(&mut self, instructions: String) {
        for instruction in instructions.lines() {
            let instruction = parse_instruction(instruction);
            self.mv_stack(instruction.from - 1, instruction.to - 1, instruction.count);
        }
    }

    pub fn get_tops(&self) -> Vec<char> {
        let mut tops: Vec<char> = Vec::new();
        for stack in &self.items {
            tops.push(stack.items[stack.items.len() - 1].to_owned());
        }
        tops
    }
}


fn parse_to_stacks(input: &str) -> Stacks {
    let mut stacks: Vec<Stack> = Vec::new();

    for cols in input.chars().collect::<Vec<char>>().chunks(4) {
        if cols[1] != ' ' {
            stacks.push(Stack {
                items: vec![cols[1]]
            });
        } else {
            stacks.push(Stack {
                items: vec![]
            })
        }
    }

    Stacks {
        items: stacks
    }
}

fn parse_input(filename: &str) -> (Stacks, String) {
    let input = read_file(filename);

    let mut split = input.split("\n\n");

    let stack_input = split.next().unwrap();
    let instructions = split.next().unwrap().to_owned();


    let mut stack_input = stack_input.lines().rev();

    let _counter_line = stack_input.next().to_owned();
    let first_line = stack_input.next().to_owned();


    let mut stacks = parse_to_stacks(first_line.unwrap());
    for line in stack_input {
        stacks.add(parse_to_stacks(line));
    }

    (stacks, instructions)
}

fn parse_instruction(instruction: &str) -> Instruction {
    let regex = Regex::new(r"move (\d+) from (.*) to (.*)").unwrap();

    let cap = regex.captures(instruction).unwrap();

    Instruction {
        count: cap[1].parse().unwrap(),
        from: cap[2].parse().unwrap(),
        to: cap[3].parse().unwrap(),
    }
}

pub fn run() {
    println!("Supply Stacks");
    let (mut stacks, instructions) = parse_input("input/day05.txt");
    stacks.perform_instructions(instructions);
    let answer: String = stacks.get_tops().iter().collect();
    println!("Final Supply Stack top crates: {}", answer);
}

pub fn run2() {
    println!("Oh, it's a 9001");
    let (mut stacks, instructions) = parse_input("input/day05.txt");
    stacks.perform_instructions9001(instructions);
    let answer: String = stacks.get_tops().iter().collect();
    println!("Then the answer is {}", answer);
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_to_stacks_work() {
        let stacks = parse_to_stacks("    [D]    ");
        assert_eq!(3, stacks.items.len());
        assert_eq!(Stack { items: vec![] }, stacks.items[2]);
        assert_eq!(Stack { items: vec!['D'] }, stacks.items[1]);
        assert_eq!(Stack { items: vec![] }, stacks.items[0]);
    }

    #[test]
    fn pushing_to_stacks_work() {
        let mut stacks1 = parse_to_stacks("    [D]     ");
        let stacks2 = parse_to_stacks("[N] [C]     ");
        let stacks3 = parse_to_stacks("[Z] [M] [P] ");
        stacks1.add(stacks2);
        stacks1.add(stacks3);
        assert_eq!(Stack { items: vec!['D', 'C', 'M'] }, stacks1.items[1]);
    }

    #[test]
    fn parsing_input() {
        let (stacks, _instructions) = parse_input("input/day05-test.txt");
        assert_eq!(Stack { items: vec!['M', 'C', 'D'] }, stacks.items[1]);
    }

    #[test]
    fn move_from_stack_to_stack() {
        let (mut stacks, _instructions) = parse_input("input/day05-test.txt");
        stacks.mv(1, 0);
        assert_eq!(Stack { items: vec!['Z', 'N', 'D'] }, stacks.items[0]);
        assert_eq!(Stack { items: vec!['M', 'C'] }, stacks.items[1]);
    }

    #[test]
    fn instruction_is_correctly_parsed() {
        let instruction = "move 3 from 2 to 1";
        assert_eq!(Instruction { from: 2, to: 1, count: 3 }, parse_instruction(instruction));
    }

    #[test]
    fn moving_instructions_are_fulfilled() {
        let (mut stacks, instructions) = parse_input("input/day05-test.txt");

        stacks.perform_instructions(instructions);

        assert_eq!(Stack { items: vec!['C'] }, stacks.items[0]);
        assert_eq!(Stack { items: vec!['M'] }, stacks.items[1]);
        assert_eq!(Stack { items: vec!['P', 'D', 'N', 'Z'] }, stacks.items[2]);
    }

    #[test]
    fn top_crates_are_returned() {
        let (mut stacks, instructions) = parse_input("input/day05-test.txt");
        stacks.perform_instructions(instructions);
        assert_eq!(vec!['C', 'M', 'Z'], stacks.get_tops());
    }

    #[test]
    fn moving_stacks() {
        let (mut stacks, _instructions) = parse_input("input/day05-test.txt");
        stacks.mv_stack(1, 0, 1);
        assert_eq!(Stack { items: vec!['Z', 'N', 'D'] }, stacks.items[0]);
        assert_eq!(Stack { items: vec!['M', 'C'] }, stacks.items[1]);
        assert_eq!(Stack { items: vec!['P'] }, stacks.items[2]);
        stacks.mv_stack(0, 2, 3);
        assert_eq!(Stack { items: vec![] }, stacks.items[0]);
        assert_eq!(Stack { items: vec!['M', 'C'] }, stacks.items[1]);
        assert_eq!(Stack { items: vec!['P', 'Z', 'N', 'D'] }, stacks.items[2]);
    }
}