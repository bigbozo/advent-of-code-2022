use crate::{read_file, Point};
use std::fmt;
use std::fmt::Debug;

#[derive(PartialEq, Copy, Clone)]
struct Field {
    height: u32,
    distance: u32,
}
impl Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[h:{}|D:{}]", self.height, self.distance)
    }
}
impl Field {
    pub fn new(height: u32) -> Field {
        Field {
            height,
            distance: u32::MAX,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Game {
    height_map: HeightMap,
    start: Point<usize>,
    end: Point<usize>,
}

type HeightMap = Vec<Vec<Field>>;

fn parse_input(input: String) -> Game {
    let mut height_map: HeightMap = vec![];
    let mut end: Point<usize> = Point { x: 0, y: 0 };
    let mut start: Point<usize> = Point { x: 0, y: 0 };

    for (y, line) in input.lines().enumerate() {
        let mut row: Vec<Field> = vec![];

        for (x, char) in line.chars().enumerate() {
            match char {
                'E' => {
                    row.push(Field::new(0));
                    end = Point { x, y };
                }
                'S' => {
                    start = Point { x, y };
                    row.push(Field::new(1));
                }
                'a'..='z' => {
                    row.push(Field::new(char as u32 - 96));
                }
                _ => {
                    row.push(Field::new(u32::MAX));
                }
            }
        }
        height_map.push(row);
    }
    height_map[end.y as usize][end.x as usize].height = height_map
        .iter()
        .map(|line| line.iter().map(|cell| cell.height).max().unwrap())
        .max()
        .unwrap();

    Game {
        height_map,
        start,
        end,
    }
}

fn walk(height_map: &mut HeightMap, p: Point<usize>, distance: u32) {
    let field = height_map[p.y][p.x];

    if height_map[p.y][p.x].distance > distance {
        height_map[p.y][p.x].distance = distance
    } else {
        return;
    }

    // UP
    if p.y > 0 {
        let target = height_map[p.y - 1][p.x];
        if target.height >= field.height - 1 && target.distance > distance + 1 {
            walk(height_map, Point { y: p.y - 1, x: p.x }, distance + 1);
        }
    }
    // DOWN
    if p.y < height_map.len() - 1 {
        let target = height_map[p.y + 1][p.x];
        if target.height >= field.height - 1 && target.distance > distance + 1 {
            walk(height_map, Point { y: p.y + 1, x: p.x }, distance + 1);
        }
    }
    // LEFT
    if p.x > 0 {
        let target = height_map[p.y][p.x - 1];
        if target.height >= field.height - 1 && target.distance > distance + 1 {
            walk(height_map, Point { y: p.y, x: p.x - 1 }, distance + 1);
        }
    }
    //RIGHT
    if p.x < height_map[0].len() - 1 {
        let target = height_map[p.y][p.x + 1];
        if target.height >= field.height - 1 && target.distance > distance + 1 {
            walk(height_map, Point { y: p.y, x: p.x + 1 }, distance + 1);
        }
    }
}

pub fn run() {
    let mut game = parse_input(read_file("input/day12.txt"));

    walk(&mut game.height_map, game.end, 0);
    println!(
        " The way is {} long",
        game.height_map[game.start.y][game.start.x].distance
    );
}
pub fn run2() {
    let mut game = parse_input(read_file("input/day12.txt"));

    walk(&mut game.height_map, game.end, 0);

    let shortest = game
        .height_map
        .iter()
        .map(|line| {
            line.iter()
                .filter(|cell| cell.height == 1)
                .map(|cell| cell.distance)
                .min()
                .unwrap()
        })
        .min()
        .unwrap();

    println!(" The shortest way from height a is {} long", shortest);
}
