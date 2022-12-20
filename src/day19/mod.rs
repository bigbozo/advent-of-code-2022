use crate::read_file;
use rayon::prelude::*;
use regex::{Captures, Regex};
use std::cmp::max;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
struct GameState {
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
        ore_cost: u32,
        clay_cost: u32,
        obsidian_ore_cost: u32,
        obsidian_clay_cost: u32,
        geode_ore_cost: u32,
        geode_obsidian_cost: u32,
    ) -> GameState {
        GameState {
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

    pub fn collect_materials(&mut self) {
        self.ores += self.ore_robots;
        self.clays += self.clay_robots;
        self.obsidians += self.obsidian_robots;
        self.geodes += self.geode_robots;
    }

    pub fn uncollect_materials(&mut self) {
        self.ores -= self.ore_robots;
        self.clays -= self.clay_robots;
        self.obsidians -= self.obsidian_robots;
        self.geodes -= self.geode_robots;
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
    let regex = Regex::new(r"Blueprint \d+: Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();

    let cap: Captures = regex.captures(line).unwrap();

    GameState::new(
        cap[1].parse().unwrap(),
        cap[2].parse().unwrap(),
        cap[3].parse().unwrap(),
        cap[4].parse().unwrap(),
        cap[5].parse().unwrap(),
        cap[6].parse().unwrap(),
    )
}

fn run_simulation(
    game_state: &mut GameState,
    steps: i32,
    messages: &mut Vec<String>,
) -> (u32, Vec<String>) {
    if steps == 0 {
        return (0, vec![]);
    }

    let mut best_steps: Vec<String> = vec![];
    let mut best_step = format!("{} - init",steps);

    // collect

    let mut max_score = 0;


    // 4. try building geode
    if steps > 1
        && game_state.ores >= game_state.geode_ore_cost
        && game_state.obsidians >= game_state.geode_obsidian_cost
    {
        game_state.ores -= game_state.geode_ore_cost;
        game_state.obsidians -= game_state.geode_obsidian_cost;
        game_state.collect_materials();
        game_state.geode_robots += 1;
        let (score, done_steps) = run_simulation(game_state, steps - 1, messages);
        if score > max_score {
            best_steps = done_steps;
            best_step = format!("{}: G [{}]", steps, game_state);
            max_score = score;
        }
        game_state.geode_robots -= 1;
        game_state.uncollect_materials();
        game_state.ores += game_state.geode_ore_cost;
        game_state.obsidians += game_state.geode_obsidian_cost;
    } else {
        if steps > 2 && game_state.obsidian_robots < game_state.max_obsidian_robots
            && game_state.ores >= game_state.obsidian_ore_cost
            && game_state.clays >= game_state.obsidian_clay_cost
        {
            game_state.ores -= game_state.obsidian_ore_cost;
            game_state.clays -= game_state.obsidian_clay_cost;
            game_state.collect_materials();
            game_state.obsidian_robots += 1;
            let (score, done_steps) = run_simulation(game_state, steps - 1, messages);
            if score > max_score {
                best_steps = done_steps;
                best_step = format!("{}: O [{}]", steps, game_state);
                max_score = score;
            }
            game_state.obsidian_robots -= 1;
            game_state.uncollect_materials();
            game_state.ores += game_state.obsidian_ore_cost;
            game_state.clays += game_state.obsidian_clay_cost;
        }

        // 2. try building clay
        if steps > 3 && game_state.clay_robots < game_state.max_clay_robots
            && game_state.ores >= game_state.clay_cost {
            game_state.ores -= game_state.clay_cost;
            game_state.collect_materials();
            game_state.clay_robots += 1;
            let (score, done_steps) = run_simulation(game_state, steps - 1, messages);
            if score > max_score {
                best_steps = done_steps;
                best_step = format!("{}: C [{}]", steps, game_state);
                max_score = score;
            }
            game_state.clay_robots -= 1;
            game_state.uncollect_materials();
            game_state.ores += game_state.clay_cost;
        }

        // 1. try building ore
        if steps > 2 && game_state.ore_robots < game_state.max_ore_robots
            && game_state.ores >= game_state.ore_cost {
            game_state.ores -= game_state.ore_cost;
            game_state.collect_materials();
            game_state.ore_robots += 1;
            let (score, done_steps) = run_simulation(game_state, steps - 1, messages);
            if score > max_score {
                best_steps = done_steps;
                best_step = format!("{}: I [{}]", steps, game_state);
                max_score = score;
            }
            game_state.ore_robots -= 1;
            game_state.uncollect_materials();
            game_state.ores += game_state.ore_cost;
        }

        if steps > 0 {
            game_state.collect_materials();
            let (score, done_steps) = run_simulation(game_state, steps - 1, messages);
            game_state.uncollect_materials();
            if score > max_score {
                best_steps = done_steps;
                best_step = format!("{}: - [{}]", steps, game_state);
                max_score = score;
            }
        }
    }

    best_steps.push(best_step);



    (max(max_score, game_state.geodes), best_steps)
}

pub fn run() {
    let game_states = parse_input(read_file("input/day19-test.txt"));
    let final_score = game_states
        .par_iter()
        .enumerate()
        .map(|(id, game_state)| {
            println!("Start Blueprint {}", id);
            let mut messages = vec![];
            let mut gs = game_state.clone();
            let (score, done_steps) = run_simulation(&mut gs, 25, &mut messages);
            println!("Score for Blueprint {}: {}", id, score);
            for line in done_steps.iter().rev() {
                println!("{}", line);
            }


            score * (id as u32 + 1)
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
