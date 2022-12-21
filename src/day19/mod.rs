use crate::read_file;
use rayon::prelude::*;
use regex::{Captures, Regex};
use std::cmp::{max};
use std::fmt::{Display, Formatter};

#[derive(Clone)]
struct GameState {
    blueprint_id: u32,
    // built robots
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
    // blueprint_costs
    ore_cost: u32,
    clay_cost: u32,
    obsidian_ore_cost: u32,
    obsidian_clay_cost: u32,
    geode_ore_cost: u32,
    geode_obsidian_cost: u32,
    // harvested items
    ores: u32,
    clays: u32,
    obsidians: u32,
    geodes: u32,
    max_ore_robots: u32,
    max_clay_robots: u32,
    max_obsidian_robots: u32,
}

impl Display for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{},{},{},{} / {},{},{},{}]",
            self.ores, self.clays, self.obsidians, self.geodes,
            self.ore_robots, self.clay_robots, self.obsidian_robots, self.geode_robots
        )
    }
}

impl GameState {
    pub fn new(
        blueprint_id: u32,
        ore_cost: u32,
        clay_cost: u32,
        obsidian_ore_cost: u32,
        obsidian_clay_cost: u32,
        geode_ore_cost: u32,
        geode_obsidian_cost: u32,
    ) -> GameState {
        GameState {
            blueprint_id,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            // blueprint_costs
            ore_cost,
            clay_cost,
            obsidian_ore_cost,
            obsidian_clay_cost,
            geode_ore_cost,
            geode_obsidian_cost,
            // harvested items
            ores: 0,
            clays: 0,
            obsidians: 0,
            geodes: 0,

            max_ore_robots: max(max(ore_cost, obsidian_ore_cost), geode_ore_cost),
            max_clay_robots: obsidian_clay_cost,
            max_obsidian_robots: geode_obsidian_cost,
        }
    }

    pub fn collect_materials(&mut self, time: u32) {
        self.ores += time * self.ore_robots;
        self.clays += time * self.clay_robots;
        self.obsidians += time * self.obsidian_robots;
        self.geodes += time * self.geode_robots;
    }

    pub fn uncollect_materials(&mut self, time: u32) {
        self.ores -= time * self.ore_robots;
        self.clays -= time * self.clay_robots;
        self.obsidians -= time * self.obsidian_robots;
        self.geodes -= time * self.geode_robots;
    }

    pub fn time_to_geode(&self) -> Option<u32> {
        if self.obsidian_robots == 0 {
            return None;
        }
        Some(max(
            self.time_to(self.geode_ore_cost, self.ores, self.ore_robots)?,
            self.time_to(self.geode_obsidian_cost, self.obsidians, self.obsidian_robots)?,
        ))
    }
    pub fn time_to_obsidian(&self) -> Option<u32> {
        if self.clay_robots == 0 {
            return None;
        }
        Some(max(
            self.time_to(self.obsidian_ore_cost, self.ores, self.ore_robots)?,
            self.time_to(self.obsidian_clay_cost, self.clays, self.clay_robots)?,
        ))
    }
    pub fn time_to_clay(&self) -> Option<u32> {
        if self.ores >= self.clay_cost {
            return Some(1);
        }
        self.time_to(self.clay_cost, self.ores, self.ore_robots)
    }
    pub fn time_to_ore(&self) -> Option<u32> {
        self.time_to(self.ore_cost, self.ores, self.ore_robots)
    }

    pub fn time_to(&self, need: u32, have: u32, multi: u32) -> Option<u32> {
        if have >= need {
            return Some(1);
        }
        let missing = need - have;
        let mut time = missing / multi;
        if missing % multi > 0 {
            time += 1;
        }
        Some(time + 1)
    }
}

fn parse_input(input: String) -> Vec<GameState> {
    let mut game_state = vec![];

    for line in input.lines() {
        game_state.push(parse_line(line));
    }

    game_state
}

fn parse_line(line: &str) -> GameState {
    let regex = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();

    let cap: Captures = regex.captures(line).unwrap();

    GameState::new(
        cap[1].parse().unwrap(),
        cap[2].parse().unwrap(),
        cap[3].parse().unwrap(),
        cap[4].parse().unwrap(),
        cap[5].parse().unwrap(),
        cap[6].parse().unwrap(),
        cap[7].parse().unwrap(),
    )
}

fn run_simulation(
    game_state: &mut GameState,
    steps: u32,
) -> (u32, Vec<char>) {
    let mut max_score: u32 = 0;

    // 1. do nothing
    max_score = max(max_score, (steps + 1) * game_state.geode_robots);
    let mut my_move: Vec<char> = vec!['!'];


    // 2. try building geode
    match game_state.time_to_geode() {
        None => {}
        Some(time) => {
            if time <= steps {
                game_state.collect_materials(time);
                game_state.ores -= game_state.geode_ore_cost;
                game_state.obsidians -= game_state.geode_obsidian_cost;
                game_state.geode_robots += 1;
                let (score, sub_move) = run_simulation(game_state, steps - time);
                if score > max_score {
                    max_score = score;
                    my_move = sub_move;
                    my_move.push('G');
                }
                game_state.geode_robots -= 1;
                game_state.ores += game_state.geode_ore_cost;
                game_state.obsidians += game_state.geode_obsidian_cost;
                game_state.uncollect_materials(time);
            }
        }
    }
    match game_state.time_to_obsidian() {
        None => {}
        Some(time) => {
            if time <= steps && game_state.obsidian_robots < game_state.max_obsidian_robots {
                game_state.collect_materials(time);
                game_state.ores -= game_state.obsidian_ore_cost;
                game_state.clays -= game_state.obsidian_clay_cost;
                game_state.obsidian_robots += 1;
                let (score, sub_move) = run_simulation(game_state, steps - time);
                if score > max_score {
                    max_score = score;
                    my_move = sub_move;
                    my_move.push('O');
                }
                game_state.obsidian_robots -= 1;
                game_state.ores += game_state.obsidian_ore_cost;
                game_state.clays += game_state.obsidian_clay_cost;
                game_state.uncollect_materials(time);
            }
        }
    }
    match game_state.time_to_clay() {
        None => {}
        Some(time) => {
            if time <= steps && game_state.clay_robots < game_state.max_clay_robots {
                game_state.collect_materials(time);
                game_state.ores -= game_state.clay_cost;
                game_state.clay_robots += 1;
                let (score, sub_move) = run_simulation(game_state, steps - time);
                if score > max_score {
                    max_score = score;
                    my_move = sub_move;
                    my_move.push('C');
                }
                game_state.clay_robots -= 1;
                game_state.ores += game_state.clay_cost;
                game_state.uncollect_materials(time);
            }
        }
    }
    match game_state.time_to_ore() {
        None => {}
        Some(time) => {
            if time <= steps && game_state.ore_robots < game_state.max_ore_robots {
                game_state.collect_materials(time);
                game_state.ores -= game_state.ore_cost;
                game_state.ore_robots += 1;
                let (score, sub_move) = run_simulation(game_state, steps - time);
                if score > max_score {
                    max_score = score;
                    my_move = sub_move;
                    my_move.push('R');
                }
                game_state.ore_robots -= 1;
                game_state.ores += game_state.ore_cost;
                game_state.uncollect_materials(time);
            }
        }
    }


    (max(max_score, game_state.geodes), my_move)
}

pub fn run() {
    let game_states = parse_input(read_file("input/day19-test.txt"));
    let final_score = game_states
        .par_iter()
        .map(|game_state| {
            println!("Start Blueprint {}", game_state.blueprint_id);
            let mut gs = game_state.clone();
            let (score, sub_move) = run_simulation(&mut gs, 24);
            println!("Score for Blueprint {}: {}", game_state.blueprint_id, score);
            println!("Path for Blueprint {}: {}", game_state.blueprint_id, sub_move.iter().rev().collect::<String>());
            score * game_state.blueprint_id
        })
        .sum::<u32>();

    println!("Final score: {}", final_score);
}

pub fn run2() {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::day19::Material::Obsidian;
    use crate::read_file;

    #[test]
    fn parses_input() {
        let blueprints: Vec<Blueprint> = parse_input(read_file("input/day19-test.txt"));

        assert_eq!(vec![Ore(4)], blueprints[0].ore_robot);
        assert_eq!(vec![Ore(2)], blueprints[0].clay_robot);
        assert_eq!(vec![Ore(3), Clay(14)], blueprints[0].obsidian_robot);
        assert_eq!(vec![Ore(2), Obsidian(7)], blueprints[0].geode_robot);
    }
}
