use std::collections::HashMap;
use crate::day22::TileType::{Floor, Void, Wall};
use crate::read_file;

#[derive(PartialEq)]
enum TileType {
    Void,
    Wall,
    Floor,
}

struct Tile {
    tile_type: TileType,
    left: usize,
    top: usize,
    right: usize,
    bottom: usize,
}

impl Tile {
    pub fn new(tile_type: TileType) -> Tile {
        Tile {
            tile_type,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        }
    }
}

struct Board {
    tiles: HashMap<usize, Tile>,
    width: usize,
    height: usize,
}

impl Board {
    pub fn new() -> Board {
        Board {
            tiles: HashMap::new(),
            width: 0,
            height: 0,
        }
    }
    pub fn get_index(&self, x: usize, y: usize) -> usize {
        self.height * (x + self.width) % self.width + (y + self.height) % self.height
    }
    pub fn get_tile(&self, x: usize, y: usize) -> Tile {
        self.tiles[&self.get_index(x, y)]
    }
    pub fn set_tile_from_char(&mut self, x: usize, y: usize, character: char) {
        match character {
            '.' => self.tiles.insert(self.get_index(x, y), Tile::new(Floor)),
            '#' => self.tiles.insert(self.get_index(x, y), Tile::new(Wall)),
            ' ' => self.tiles.insert(self.get_index(x, y), Tile::new(Void)),
            _ => {
                panic!("unknown char in map");
            }
        };
    }
    pub fn get_pos(&self, id: usize) -> (usize, usize) {
        let x = id % self.height;
        let y = id / self.height;
        (x, y)
    }
}


pub fn run() {
    let input = read_file("input/day22.txt");
    let (map, _commands) = input.split_once("\n\n").unwrap();

    let mut board: Board = Board::new();
    board.height = map.lines().count();
    for (y, line) in map.lines().enumerate() {
        board.width = line.chars().count();
        for (x, character) in line.chars().enumerate() {
            board.set_tile_from_char(x, y, character);
        }
    }
    for (_id, tile) in board.tiles.iter_mut() {
        if tile.tile_type == Floor {

        }
    }
}