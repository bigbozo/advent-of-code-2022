use std::cmp::max;
use std::collections::{HashMap};
use regex::{Captures, Regex};
use crate::day19::Material::{Clay, Geode, Obsidian, Ore};
use crate::read_file;

#[derive(Eq, Hash, PartialEq, Debug)]
enum Material {
    Ore(u32),
    Clay(u32),
    Obsidian(u32),
    Geode(u32),
}

type RobotType = Vec<Material>;

#[derive(Debug)]
struct Blueprint {
    ore_robot: RobotType,
    clay_robot: RobotType,
    obsidian_robot: RobotType,
    geode_robot: RobotType,
}

#[derive(Debug)]
struct Robot {
    produces: Material,
}

fn parse_input(input: String) -> Vec<Blueprint> {
    let mut blueprints = vec![];

    for line in input.lines() {
        blueprints.push(parse_line(line));
    }


    blueprints
}

fn parse_line(line: &str) -> Blueprint {
    let regex = Regex::new(r"Blueprint \d+: Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();

    let cap: Captures = regex.captures(line)
        .unwrap();

    Blueprint {
        ore_robot: vec![Ore(cap[1].parse().unwrap())],
        clay_robot: vec![Ore(cap[2].parse().unwrap())],
        obsidian_robot: vec![Ore(cap[3].parse().unwrap()), Clay(cap[4].parse().unwrap())],
        geode_robot: vec![Ore(cap[5].parse().unwrap()), Obsidian((cap[6]).parse().unwrap())],
    }
}


pub fn run() {
    let blueprints = parse_input(read_file("input/day19-test.txt"));
    for blueprint in blueprints {
        let mut robots = vec![Robot { produces: Ore(1) }];
        let mut items = HashMap::new();

        items.insert(Ore(1), 0);
        items.insert(Clay(1), 0);
        items.insert(Obsidian(1), 0);
        items.insert(Geode(1), 0);

        println!("SCore for Blueprint: {}", run_simulation(&blueprint, &mut items, &mut robots, 24));
    }
}

fn run_simulation(blueprint: &Blueprint, items: &mut HashMap<Material, u32>, robots: &mut Vec<Robot>, steps: i32) -> u32 {
    if steps == 0 {
        return 0;
    }

    let mut max_score = 0;
    // collect
    {
        for robot in robots.iter() {
            match robot.produces {
                Ore(amount) => { items.entry(Ore(1)).and_modify(|count| *count += amount); }
                Clay(amount) => { items.entry(Clay(1)).and_modify(|count| *count += amount); }
                Obsidian(amount) => { items.entry(Obsidian(1)).and_modify(|count| *count += amount); }
                Geode(amount) => { items.entry(Geode(1)).and_modify(|count|
                    *count += amount); }
            }
        }
    }
    // 0. build nothing

    if steps>1 {
        max_score = max(max_score, run_simulation(blueprint, items, robots, steps - 1));
        // 1. try building ore
        if is_material_sufficient(&blueprint.ore_robot, items) {
            build_robot(&blueprint.ore_robot, items);
            robots.push(Robot { produces: Ore(1) });
            max_score = max(max_score, run_simulation(blueprint, items, robots, steps - 1));
            robots.pop();
            unbuild_robot(&blueprint.ore_robot, items);
        }
        // 2. try building clay
        if is_material_sufficient(&blueprint.clay_robot, items) {
            robots.push(Robot { produces: Clay(1) });
            build_robot(&blueprint.clay_robot, items);
            max_score = max(max_score, run_simulation(blueprint, items, robots, steps - 1));
            unbuild_robot(&blueprint.clay_robot, items);
            robots.pop();
        }

        // 3. try building obsidian
        if is_material_sufficient(&blueprint.obsidian_robot, items) {
            build_robot(&blueprint.obsidian_robot, items);
            robots.push(Robot { produces: Obsidian(1) });
            max_score = max(max_score, run_simulation(blueprint, items, robots, steps - 1));
            unbuild_robot(&blueprint.obsidian_robot, items);
            robots.pop();
        }

        // 4. try building geode
        if is_material_sufficient(&blueprint.geode_robot, items) {
            robots.push(Robot { produces: Geode(1) });
            build_robot(&blueprint.geode_robot, items);
            max_score = max(max_score, run_simulation(blueprint, items, robots, steps - 1));
            unbuild_robot(&blueprint.geode_robot, items);
            robots.pop();
        }
    }


    let result = match items.get(&Geode(1)) {
        Some(amount) => {
            if *amount > max_score && steps>3 {
                println!("new highscore {} /  {}", steps, *amount);
            }
            max(max_score, *amount)
        }
        None => 0
    };

    // uncollect
    {
        for robot in robots.iter() {
            match robot.produces {
                Ore(amount) => { items.entry(Ore(1)).and_modify(|count|
                    *count -= amount); }
                Clay(amount) => { items.entry(Clay(1)).and_modify(|count|
                    *count -= amount); }
                Obsidian(amount) => { items.entry(Obsidian(1)).and_modify(|count|
                    *count -= amount); }
                Geode(amount) => { items.entry(Geode(1)).and_modify(|count|
                    *count -= amount); }
            }
        }
    }

    result
}

fn build_robot(blueprint: &Vec<Material>, items: &mut HashMap<Material, u32>) -> bool {
    // use material and return robot
    if is_material_sufficient(blueprint, items) {
        for material in blueprint {
            match material {
                Ore(amount) => {
                    items.entry(Ore(1)).and_modify(|a| *a -= amount);
                }
                Clay(amount) => {
                    items.entry(Clay(1)).and_modify(|a| *a -= amount);
                }
                Obsidian(amount) => {
                    items.entry(Obsidian(1)).and_modify(|a| *a -= amount);
                }
                Geode(amount) => {
                    items.entry(Geode(1)).and_modify(|a| *a -= amount);
                }
            }
        }
        return true;
    }
    false
}

fn is_material_sufficient(blueprint: &Vec<Material>, items: &mut HashMap<Material, u32>) -> bool {
    for material in blueprint {
        match material {
            Ore(amount) => {
                match items.get(&Ore(1)) {
                    None => { return false; }
                    Some(a) => {
                        if a < amount {
                            return false;
                        }
                    }
                }
            }
            Clay(amount) => {
                match items.get(&Clay(1)) {
                    None => { return false; }
                    Some(a) => {
                        if a < amount {
                            return false;
                        }
                    }
                }
            }
            Obsidian(amount) => {
                match items.get(&Obsidian(1)) {
                    None => { return false; }
                    Some(a) => {
                        if a < amount {
                            return false;
                        }
                    }
                }
            }
            Geode(amount) => {
                match items.get(&Geode(1)) {
                    None => { return false; }
                    Some(a) => {
                        if a < amount {
                            return false;
                        }
                    }
                }
            }
        }
    }
    true
}

fn unbuild_robot(blueprint: &Vec<Material>, items: &mut HashMap<Material, u32>) {
    for material in blueprint {
        match material {
            Ore(amount) => {
                items.entry(Ore(1)).and_modify(|a| *a += amount);
            }
            Clay(amount) => {
                items.entry(Clay(1)).and_modify(|a| *a += amount);
            }
            Obsidian(amount) => {
                items.entry(Obsidian(1)).and_modify(|a| *a += amount);
            }
            Geode(amount) => {
                items.entry(Geode(1)).and_modify(|a| *a += amount);
            }
        }
    }
}

pub fn run2() {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::day19::Material::Obsidian;
    use crate::read_file;
    use super::*;

    #[test]
    fn parses_input() {
        let blueprints: Vec<Blueprint> = parse_input(read_file("input/day19-test.txt"));

        assert_eq!(vec![Ore(4)], blueprints[0].ore_robot);
        assert_eq!(vec![Ore(2)], blueprints[0].clay_robot);
        assert_eq!(vec![Ore(3), Clay(14)], blueprints[0].obsidian_robot);
        assert_eq!(vec![Ore(2), Obsidian(7)], blueprints[0].geode_robot);
    }
}

