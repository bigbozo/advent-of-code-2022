use regex::Regex;
use crate::read_file;
use crate::day11::Operation::{
    Mul,
    Add,
};
use crate::day11::Parameter::{
    Old,
    Constant,
};

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum Parameter {
    Constant(usize),
    Old,
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum Operation {
    Add(Parameter, Parameter),
    Mul(Parameter, Parameter),
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Test {
    divisor: usize,
    true_target: usize,
    false_target: usize,
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Monkey {
    id: usize,
    items: Vec<usize>,
    op: Operation,
    test: Test,
    inspected: usize
}

type MonkeyHorde = Vec<Monkey>;

pub fn parse_op(input: &str) -> Operation {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let left = match parts[0] {
        "old" => Old,
        _ => Constant(parts[0].parse().unwrap())
    };
    let right = match parts[2] {
        "old" => Old,
        _ => Constant(parts[2].parse().unwrap())
    };
    match parts[1] {
        "*" => Mul(left, right),
        "+" => Add(left, right),
        _ => {
            panic!("Unknown operation");
        }
    }
}

pub fn parse_monkey(input: &str) -> Monkey {
    let regex = Regex::new(r"^Monkey (\d+):\n\s+Starting items: ([^\n]+)\n\s+Operation: new = ([^\n]+)\n\s+Test: divisible by ([^\n]+)\n\s+If true: throw to monkey ([^\n]+)\n\s+If false: throw to monkey ([^\n]+)$").unwrap();

    let cap = regex.captures(input).unwrap();


    Monkey {
        id: cap[1].parse().unwrap(),
        items: cap[2].split(", ").map(|c| c.parse().unwrap()).collect(),
        op: parse_op(&cap[3]),
        test: Test {
            divisor: cap[4].parse().unwrap(),
            true_target: cap[5].parse().unwrap(),
            false_target: cap[6].parse().unwrap(),
        },
        inspected: 0
    }
}

fn parse_monkeys(input: String) -> MonkeyHorde {
    let mut monkey_horde: MonkeyHorde = vec![];
    for line in input.split("\n\n") {
        monkey_horde.push(parse_monkey(line));
    }
    monkey_horde
}

pub fn run_turns(monkeys: &mut MonkeyHorde, count: usize, worry_mul: usize) {


    let mut kgv = 1;
    for monkey in monkeys.iter_mut() {
        kgv *= monkey.test.divisor;
    }

    for _rounds in 0..count {
        for i in 0..monkeys.len() {
            let monkey = monkeys[i].clone();
            let false_target = monkey.test.false_target;
            let true_target = monkey.test.true_target;
            monkeys[monkey.id].items = vec![];
            monkeys[monkey.id].inspected += monkey.items.len() as usize;
            let op = &monkey.op;
            for item in monkey.items {
                let score = perform_op(item, op, kgv) / worry_mul;
                if score % monkey.test.divisor > 0 {
                    monkeys[false_target].items.push(score);
                } else {
                    monkeys[true_target].items.push(score);
                }
            }
        }
    }
}

pub fn monkey_business_level(monkeys: &mut MonkeyHorde) -> usize {

    monkeys.sort_by(|a,b| a.inspected.cmp(&b.inspected));

    monkeys.pop().unwrap().inspected * monkeys.pop().unwrap().inspected

}

fn perform_op(value: usize, op: &Operation, kgv: usize) -> usize {
    match op {
        Add(left, right) => {
            let l = match left {
                Constant(v) => *v,
                Old => value
            };
            let r = match right {
                Constant(v) => *v,
                Old => value
            };
            (l + r) % kgv
        }
        Mul(left, right) => {
            let l = match left {
                Constant(v) => *v,
                Old => value
            };
            let r = match right {
                Constant(v) => *v,
                Old => value
            };
            (l * r) % kgv
        }
    }
}


pub fn run() {
    let input = read_file("input/day11.txt");
    let mut monkeys = parse_monkeys(input);
    run_turns(&mut monkeys, 20, 3);

    println!("Level of monkey business: {}", monkey_business_level(&mut monkeys));

}

pub fn run2() {
    let input = read_file("input/day11.txt");
    let mut monkeys = parse_monkeys(input);
    run_turns(&mut monkeys, 10_000, 1);

    println!("Level of monkey business after 10000 rounds: {}", monkey_business_level(&mut monkeys));

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_monkey_works() {
        assert_eq!(Monkey {
            id: 0,
            items: vec![79, 98],
            op: Mul(Old, Constant(19)),
            test: Test { divisor: 23, true_target: 2, false_target: 3, },
            inspected: 0,
        }, parse_monkey("Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3"));
    }

    #[test]
    fn parse_monkeys_work() {
        let input = read_file("input/day11-test.txt");
        let monkeys = parse_monkeys(input);
        assert_eq!(4, monkeys.len());
    }

    #[test]
    fn perform_ops_work() {
        assert_eq!(1501, perform_op(79, &Mul(Old, Constant(19)),23 * 19 * 13 * 17));
    }

    #[test]
    fn run_turn_works() {
        let input = read_file("input/day11-test.txt");
        let mut monkeys = parse_monkeys(input);
        run_turns(&mut monkeys, 1, 3);
        assert_eq!(vec![20, 23, 27, 26], monkeys[0].items);
        assert_eq!(0, monkeys[2].items.len());
        assert_eq!(0, monkeys[3].items.len());
        assert_eq!(vec![2080, 25, 167, 207, 401, 1046], monkeys[1].items);


        run_turns(&mut monkeys, 19, 3);

        assert_eq!(vec![10, 12, 14, 26, 34], monkeys[0].items);
        assert_eq!(0, monkeys[2].items.len());
        assert_eq!(0, monkeys[3].items.len());
        assert_eq!(vec![245, 93, 53, 199, 115], monkeys[1].items);
    }

    #[test]
    fn counting_works() {
        let input = read_file("input/day11-test.txt");
        let mut monkeys = parse_monkeys(input);
        run_turns(&mut monkeys, 20, 3);
        assert_eq!(101,monkeys[0].inspected);
        assert_eq!(95,monkeys[1].inspected);
        assert_eq!(7,monkeys[2].inspected);
        assert_eq!(105,monkeys[3].inspected);

    }
    #[test]
    fn calc_monkey_business_level_works() {
        let input = read_file("input/day11-test.txt");
        let mut monkeys = parse_monkeys(input);
        run_turns(&mut monkeys, 20, 3);

        assert_eq!(10605, monkey_business_level(&mut monkeys));

    }

    #[test]
    fn calc_monkey_business_level_with_worry_mul_works() {
        let input = read_file("input/day11-test.txt");
        let mut monkeys = parse_monkeys(input);
        run_turns(&mut monkeys, 10_000, 1);

        assert_eq!(2713310158, monkey_business_level(&mut monkeys));

    }


}
