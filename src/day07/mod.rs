use std::collections::HashMap;
use std::process::exit;
use crate::read_file;

#[derive(PartialEq)]
enum LineMode {
    COMMAND,
    OUTPUT,
}


pub fn calculate() -> HashMap<String, u32> {
    let input = read_file("input/day07.txt");
    let mut dir_map: HashMap<String, u32> = HashMap::new();
    let mut current_dir: Vec<String> = vec!["/".to_string()];
    let mut line_mode = LineMode::COMMAND;
    let mut size_string: &str;

    for line in input.lines() {
        if line_mode == LineMode::OUTPUT {
            if line[0..1] == "$".to_string() {
                line_mode = LineMode::COMMAND;
            } else {
                (size_string, _) = line.split_once(" ").unwrap();
                if size_string == "dir" {
                    continue
                }
                let size: u32 = size_string.parse().unwrap();
                for i in 0..current_dir.len() {
                    let key = current_dir[0..=i].join("/");
                    dir_map.entry(key).and_modify(|dir_size| *dir_size += size).or_insert(size);
                }
            }
        }
        if line_mode == LineMode::COMMAND {
            let chars : Vec<char> = line.chars().collect();
            if chars[0] != '$' {
                panic!("Expected command");
            }

            // Command

            match &line[2..=3] {
                "ls" => {
                    line_mode = LineMode::OUTPUT;
                },
                "cd" => {
                    let dir = &line[5..];
                    if dir == ".." && current_dir.len()>0 {
                        current_dir.pop();
                    } else if dir == "/" {
                        current_dir = vec!["/".to_string()];
                    } else {
                        current_dir.push(dir.to_owned());
                    }
                }
                _ => {
                    println!("Unknown command {}",&line[2..3]);
                    exit(1)
                }
            }
        }
    }
    dir_map
}

pub fn run() {
    let dir_map = calculate();
    let v: Vec<u32> = dir_map.into_values().filter(|value| *value<100_000).collect();
    println!("Sum of below 100_000 dirs is: {:?}",v.iter().sum::<u32>());
}
pub fn run2() {
    let dir_map = calculate();
    let total = *dir_map.get("/").unwrap();
    let needed: u32 = total - 40_000_000;
    println!("Total used is {}",total);
    println!("To be freed {}",needed);
    let mut target_key = "/".to_string();
    let mut min_val = total;
    for (key,val) in dir_map.iter() {
        if *val < min_val {
            if *val >= needed {
                target_key = key.to_string();
                min_val = *val;
            }
        }
    }
    println!("We should delete {} with size: {}",target_key,min_val);

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_command() {
        run();
    }
}





