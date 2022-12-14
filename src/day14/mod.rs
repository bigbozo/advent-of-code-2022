use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use itertools::Itertools;
use png::Encoder;
use crate::Point;
use crate::read_file;


type Points = Vec<Point<i32>>;

#[derive(Debug, PartialEq)]
struct Rules {
    points: Points,
}

impl Rules {
    fn max_x(&self) -> Option<i32> {
        self.points.iter().map(|point| point.x).max()
    }
    fn max_y(&self) -> Option<i32> {
        self.points.iter().map(|point| point.y).max()
    }
    fn min_x(&self) -> Option<i32> {
        self.points.iter().map(|point| point.x).min()
    }
    fn min_y(&self) -> Option<i32> {
        self.points.iter().map(|point| point.y).min()
    }
}

struct Ruleset {
    items: Vec<Rules>,
}

impl Ruleset {
    pub fn new(items: Vec<Rules>) -> Ruleset {
        Ruleset {
            items
        }
    }

    fn max_x(&self) -> Option<i32> {
        self.items.iter().map(|rules| rules.max_x()).max()?
    }
    fn max_y(&self) -> Option<i32> {
        self.items.iter().map(|rules| rules.max_y()).max()?
    }
    fn min_x(&self) -> Option<i32> {
        self.items.iter().map(|rules| rules.min_x()).min()?
    }
    fn min_y(&self) -> Option<i32> {
        self.items.iter().map(|rules| rules.min_y()).min()?
    }
}


struct Board {
    map: Vec<Vec<char>>,
    offset: Point<i32>,
}

impl Board {
    pub fn set(&mut self, x: i32, y: i32, c: char) {
        self.map[y as usize][(x - self.offset.x) as usize] = c;
    }
    pub fn get(&self, x: i32, y: i32) -> char {
        let ox = (x - self.offset.x) as usize;
        let oy = y as usize;
        if oy < self.map.len() && ox < self.map[0].len() {
            return self.map[oy][ox];
        }
        '.'
    }

    pub fn is_outside(&self, x: i32, y: i32) -> bool {
        let ox = (x - self.offset.x) as usize;
        let oy = y as usize;


        ox >= self.map[0].len() || oy >= self.map.len()
    }

    pub fn count_sand(&self) -> usize {
        self.map.iter().map(|row| row.iter().map(|cell| match cell {
            'o' => 1,
            _ => 0
        }).sum::<usize>()).sum()
    }

    pub fn save_png(&self, filename: &str) {
        let full_file = "output/".to_string() + filename;
        let path = Path::new(&full_file);
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = Encoder::new(w, self.map[0].len() as u32, self.map.len() as u32);
        encoder.set_color(png::ColorType::Grayscale);
        let mut writer = encoder.write_header().unwrap();

        let data: Vec<u8> = self.map.iter().map(|row| row.iter().map(|cell| *cell as u8).collect::<Vec<u8>>()).collect::<Vec<Vec<u8>>>().concat();

        writer.write_image_data(&data).unwrap();
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.map.iter().map(|row| row.iter().collect::<String>()).join("\n"))
    }
}

fn parse_input(input: String) -> Vec<Rules> {
    let mut ruleset: Vec<Rules> = vec![];

    for line in input.lines() {
        let rules: Rules = Rules {
            points: line.split(" -> ").map(|coordinate| {
                let (x, y) = coordinate.split_once(',').unwrap();
                Point { x: x.parse().unwrap(), y: y.parse().unwrap() }
            }).collect::<Points>()
        };
        ruleset.push(rules);
    }

    ruleset
}

fn create_board(ruleset: Ruleset) -> Option<Board> {
    let x = ruleset.min_x()?;
    let y = ruleset.min_y()?;
    let x1 = ruleset.max_x()?;
    let y1 = ruleset.max_y()?;
    let width = x1 - x + 1;
    let height = y1 + 1;

    let mut board = Board {
        map: vec![vec!['.'; width as usize]; height as usize],
        offset: Point { x, y },
    };


    for rules in ruleset.items {
        for i in 0..rules.points.len() - 1 {
            let cursor = rules.points[i];
            let next = rules.points[i + 1];
            paint_line(&mut board, cursor, next);
        }
    }

    Some(board)
}

fn paint_line(board: &mut Board, from: Point<i32>, to: Point<i32>) {
    match from.x.cmp(&to.x) {
        Ordering::Less => {
            paint_horizontal_line(board, from.x, to.x, from.y);
        }
        Ordering::Equal => {
            match from.y.cmp(&to.y) {
                Ordering::Less => {
                    paint_vertical_line(board, from.x, from.y, to.y);
                }
                Ordering::Equal => {
                    board.set(from.x, from.y, '#');
                }
                Ordering::Greater => {
                    paint_vertical_line(board, from.x, to.y, from.y);
                }
            }
        }
        Ordering::Greater => {
            paint_horizontal_line(board, to.x, from.x, from.y);
        }
    }
}

fn paint_vertical_line(board: &mut Board, x: i32, y1: i32, y2: i32) {
    for y in y1..=y2 {
        board.set(x, y, '#');
    }
}

fn paint_horizontal_line(board: &mut Board, x1: i32, x2: i32, y: i32) {
    for x in x1..=x2 {
        board.set(x, y, '#');
    }
}

fn run_simulation(board: &mut Board) {
    let mut cx = 500;
    let mut cy = 0;


    loop {
        cy += 1;
        if board.get(cx, cy) == '.' {} else if board.get(cx - 1, cy) == '.' {
            cx -= 1;
        } else if board.get(cx + 1, cy) == '.' {
            cx += 1;
        } else {
            board.set(cx, cy - 1, 'o');
            if cx == 500 && cy == 1 {
                break;
            }
            cx = 500;
            cy = 0;
        }

        if board.is_outside(cx, cy) {
            break;
        }
    }
}


pub fn run() {
    let board = create_board(Ruleset::new(parse_input(read_file("input/day14.txt"))));

    let mut board = board.unwrap();

    run_simulation(&mut board);


    println!("{} units sand are collected!", board.count_sand());
    println!("I painted a nice picture for you (output/day14-a.png)");

    board.save_png("day14-a.png");
}

pub fn run2() {
    let mut ruleset = Ruleset::new(parse_input(read_file("input/day14.txt")));
    let y = ruleset.max_y().unwrap();
    ruleset.items.push(Rules { points: vec![Point::new(500 - y - 2, y + 2), Point::new(500 + y + 2, y + 2)] });
    let mut board = create_board(ruleset).unwrap();
    run_simulation(&mut board);

    println!("{} units of sand come to rest", board.count_sand());
    println!("This pic is even nicer (output/day14-b.png)");
    board.save_png("day14-b.png");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_lines() {
        assert_eq!(vec![
            Rules { points: vec![Point::new(1, 1), Point::new(2, 2)] }
        ], parse_input("1,1 -> 2,2".to_string()));
        assert_eq!(vec![
            Rules { points: vec![Point::new(498, 4), Point::new(498, 6), Point::new(496, 6)] },
            Rules { points: vec![Point::new(503, 4), Point::new(502, 4), Point::new(502, 9), Point::new(494, 9)] },
        ], parse_input(read_file("input/day14-test.txt")));
    }

    #[test]
    fn max_min_works() {
        let rules = Rules { points: vec![Point::new(503, 4), Point::new(502, 4), Point::new(502, 9), Point::new(494, 9)] };
        assert_eq!(Some(503), rules.max_x());
        assert_eq!(Some(494), rules.min_x());
        assert_eq!(Some(9), rules.max_y());
        assert_eq!(Some(4), rules.min_y());

        let ruleset = Ruleset {
            items: vec![
                Rules { points: vec![Point::new(498, 4), Point::new(498, 6), Point::new(496, 6)] },
                Rules { points: vec![Point::new(503, 4), Point::new(502, 4), Point::new(502, 9), Point::new(494, 9)] },
            ]
        };
        assert_eq!(Some(503), ruleset.max_x());
        assert_eq!(Some(494), ruleset.min_x());
        assert_eq!(Some(9), ruleset.max_y());
        assert_eq!(Some(4), ruleset.min_y());
    }

    #[test]
    fn painting_board_works() {
        let board = create_board(Ruleset::new(parse_input(read_file("input/day14-test.txt"))));

        let board = board.unwrap();

        println!("{}", &board);
        assert_eq!("..........
..........
..........
..........
....#...##
....#...#.
..###...#.
........#.
........#.
#########.", format!("{}", &board));
    }

    #[test]
    fn filling_with_sand_works() {
        let board = create_board(Ruleset::new(parse_input(read_file("input/day14-test.txt"))));

        let mut board = board.unwrap();

        run_simulation(&mut board);

        println!("{}", &board);
        assert_eq!("..........
..........
......o...
.....ooo..
....#ooo##
...o#ooo#.
..###ooo#.
....oooo#.
.o.ooooo#.
#########.", format!("{}", &board));
    }

    #[test]
    fn counting_sand_works() {
        let board = create_board(Ruleset::new(parse_input(read_file("input/day14-test.txt"))));

        let mut board = board.unwrap();

        run_simulation(&mut board);

        assert_eq!(24, board.count_sand());

        board.save_png("day14-test-a.png");
    }

    #[test]
    fn part2_works() {
        let mut ruleset = Ruleset::new(parse_input(read_file("input/day14-test.txt")));
        let y = ruleset.max_y().unwrap();
        ruleset.items.push(Rules { points: vec![Point::new(500 - y - 2, y + 2), Point::new(500 + y + 2, y + 2)] });
        let mut board = create_board(ruleset).unwrap();
        run_simulation(&mut board);

        println!("{}", board);

        assert_eq!(93, board.count_sand());

        board.save_png("day14-test-b.png");
    }
}