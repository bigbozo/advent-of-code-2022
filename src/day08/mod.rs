use crate::day08::Direction::{East, North, South, West};
use crate::read_file;
use ansi_term::Colour::Yellow;
use std::cmp::max;

#[derive(Debug, PartialEq, Eq)]
pub struct Tree {
    north: bool,
    east: bool,
    south: bool,
    west: bool,
    height: i32,
}

type Forest = Vec<Vec<Tree>>;

enum Direction {
    North,
    East,
    South,
    West,
}

impl Tree {
    fn visible_from_dir(&mut self, dir: &Direction) {
        match dir {
            East => self.west = true,
            West => self.east = true,
            North => self.south = true,
            South => self.north = true,
        }
    }
    fn is_visible(&self) -> bool {
        self.west || self.east || self.north || self.south
    }
}

fn parse_input(input: String) -> Forest {
    let mut rows: Forest = Vec::new();
    for line in input.lines() {
        let mut cols: Vec<Tree> = Vec::new();
        for item in line.chars() {
            cols.push(Tree {
                north: false,
                east: false,
                south: false,
                west: false,
                height: item.to_digit(10).unwrap() as i32,
            })
        }
        rows.push(cols);
    }
    rows
}

fn scan_horizontal(forest: &mut Forest, y_range: Vec<usize>, x_range: Vec<usize>, dir: Direction) {
    for y in &y_range {
        let mut act_height: i32 = -1;
        for x in &x_range {
            if forest[*y][*x].height > act_height {
                forest[*y][*x].visible_from_dir(&dir);
                act_height = forest[*y][*x].height;
                if act_height == 9 {
                    break;
                }
            }
        }
    }
}

fn scan_vertical(forest: &mut Forest, y_range: Vec<usize>, x_range: Vec<usize>, dir: Direction) {
    for x in &x_range {
        let mut act_height: i32 = -1;
        for y in &y_range {
            if forest[*y][*x].height > act_height {
                forest[*y][*x].visible_from_dir(&dir);
                act_height = forest[*y][*x].height;
                if act_height == 9 {
                    break;
                }
            }
        }
    }
}

fn scan_forest(forest: &mut Forest) -> &mut Forest {
    let height = forest.len();
    let width = forest[0].len();

    scan_horizontal(
        forest,
        (0..height).collect::<Vec<_>>(),
        (0..width).collect::<Vec<_>>(),
        East,
    );
    scan_horizontal(
        forest,
        (0..height).collect::<Vec<_>>(),
        (0..width).rev().collect::<Vec<_>>(),
        West,
    );
    scan_vertical(
        forest,
        (0..height).collect::<Vec<_>>(),
        (0..width).collect::<Vec<_>>(),
        South,
    );
    scan_vertical(
        forest,
        (0..height).rev().collect::<Vec<_>>(),
        (0..width).collect::<Vec<_>>(),
        North,
    );

    forest
}

pub fn input_forest(filename: &str) -> Forest {
    let input = read_file(filename);
    let mut forest = parse_input(input);
    scan_forest(&mut forest);

    forest
}

pub fn count_visibles(forest: &mut Forest) -> i32 {
    let mut count = 0;
    for trees in forest {
        for tree in trees {
            count += i32::from(tree.is_visible());
        }
    }

    count
}

pub fn max_scenic_score(forest: &Forest) -> i32 {
    let height = forest.len();
    let width = forest[0].len();
    let mut scenic_score = 0;

    for y in 0..height {
        for x in 0..width {
            scenic_score = max(scenic_score, calc_scenic_score(forest, y, x));
        }
    }
    scenic_score
}

fn calc_scenic_score(forest: &Forest, y: usize, x: usize) -> i32 {
    let height = forest.len();
    let width = forest[0].len();

    let tree_height = forest[y][x].height;
    let mut d1 = 0;
    let mut d2 = 0;
    let mut d3 = 0;
    let mut d4 = 0;

    if x < width - 1 {
        for x1 in (x + 1)..width {
            d1 += 1;
            if tree_height <= forest[y][x1].height {
                break;
            }
        }
    }
    if x > 0 {
        for x1 in 0..x {
            d2 += 1;
            if tree_height <= forest[y][x - 1 - x1].height {
                break;
            }
        }
    }

    if y < height - 1 {
        for y1 in (y + 1)..height as usize {
            d3 += 1;
            if tree_height <= forest[y1][x].height {
                break;
            }
        }
    }
    if y > 0 {
        for y1 in 0..y {
            d4 += 1;
            if tree_height <= forest[y - 1 - y1][x].height {
                break;
            }
        }
    }
    d1 * d2 * d3 * d4
}

pub fn run() {
    let mut forest = input_forest("input/day08.txt");

    let count = count_visibles(&mut forest);
    println!(
        "There are {} trees visible",
        Yellow.bold().paint(format!("{}", count))
    );
}

pub fn run2() {
    let forest = input_forest("input/day08.txt");
    let score = max_scenic_score(&forest);
    println!(
        "The highest scenic scoring tree scores {}",
        Yellow.bold().paint(format!("{}", score))
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_input_works() {
        let forest = input_forest("input/day08-test.txt");

        assert_eq!(
            Tree {
                north: true,
                west: true,
                east: false,
                south: false,
                height: 3
            },
            forest[0][0]
        );
        assert_eq!(
            Tree {
                north: true,
                west: true,
                east: false,
                south: false,
                height: 5
            },
            forest[1][1]
        );
        assert_eq!(
            Tree {
                north: true,
                west: false,
                east: true,
                south: false,
                height: 5
            },
            forest[1][2]
        );
        assert_eq!(
            Tree {
                north: false,
                west: false,
                east: false,
                south: false,
                height: 1
            },
            forest[1][3]
        );
        assert_eq!(
            Tree {
                north: false,
                west: false,
                east: true,
                south: false,
                height: 5
            },
            forest[2][1]
        );
        assert_eq!(
            Tree {
                north: false,
                west: false,
                east: false,
                south: false,
                height: 3
            },
            forest[2][2]
        );
        assert_eq!(
            Tree {
                north: false,
                west: false,
                east: true,
                south: false,
                height: 3
            },
            forest[2][3]
        );
    }

    #[test]
    fn summing_workd() {
        let mut forest = input_forest("input/day08-test.txt");

        let count = count_visibles(&mut forest);
        assert_eq!(21, count);
    }

    #[test]
    fn scenic_score_is_correct() {
        let mut forest = input_forest("input/day08-test.txt");

        assert_eq!(4, calc_scenic_score(&mut forest, 1, 2));
        assert_eq!(8, calc_scenic_score(&mut forest, 3, 2));
        assert_eq!(8, max_scenic_score(&forest));
    }
}
