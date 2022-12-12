use std::fmt;
use std::fmt::Debug;
use crate::{Point, read_file};

#[derive(PartialEq, Copy, Clone)]
struct Field {
    height: u32,
    distance_to_end: u32,
}
impl Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[h:{}|D:{}]", self.height, self.distance_to_end)
    }
}
impl Field {
    pub fn new(height: u32) -> Field {
        Field {
            height,
            distance_to_end: u32::MAX,
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

    for (y,line) in input.lines().enumerate() {
        let mut row: Vec<Field> = vec![];

        for (x,char) in line.chars().enumerate() {
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
        .map(|line| line
            .iter()
            .map(|cell| cell.height)
            .max().unwrap())
        .max().unwrap();


    Game {
        height_map,
        start,
        end,
    }
}

fn walk(height_map: &mut HeightMap, point: Point<usize>, distance: u32) {
    let field = height_map[point.y][point.x];

    if height_map[point.y][point.x].distance_to_end > distance {
        height_map[point.y][point.x].distance_to_end = distance
    } else {
        return;
    }

    // UP
    if point.y > 0 {
        let target = height_map[point.y - 1][point.x];
        if target.height >= field.height - 1 &&
            target.distance_to_end > distance + 1 {
            walk(height_map, Point { y: point.y - 1, x: point.x }, distance + 1);
        }
    }
    // DOWN
    if point.y < height_map.len() - 1 {
        let target = height_map[point.y + 1][point.x];
        if target.height >= field.height - 1 &&
            target.distance_to_end > distance + 1 {
            walk(height_map, Point { y: point.y + 1, x: point.x }, distance + 1);
        }
    }
    // LEFT
    if point.x > 0 {
        let target = height_map[point.y][point.x - 1];
        if target.height >= field.height - 1 &&
            target.distance_to_end > distance + 1 {
            walk(height_map, Point { y: point.y, x: point.x - 1 }, distance + 1);
        }
    }
    //RIGHT
    if point.x < height_map[0].len() - 1 {
        let target = height_map[point.y][point.x + 1];
        if target.height >= field.height - 1 &&
            target.distance_to_end > distance + 1 {
            walk(height_map, Point { y: point.y, x: point.x + 1 }, distance + 1);
        }
    }
}

pub fn run() {
    let mut game = parse_input(read_file("input/day12.txt"));

    walk(&mut game.height_map, game.end, 0);
    println!(" The way is {} long", game.height_map[game.start.y][game.start.x].distance_to_end);


}
pub fn run2() {
    let mut game = parse_input(read_file("input/day12.txt"));

    walk(&mut game.height_map, game.end, 0);

    let shortest = game.height_map
        .iter()
        .map(|line| line
            .iter()
            .filter(|cell| cell.height == 1)
            .map(|cell| cell.distance_to_end)
            .min().unwrap())
        .min().unwrap();

    println!(" The shortest way from height a is {} long", shortest);
}