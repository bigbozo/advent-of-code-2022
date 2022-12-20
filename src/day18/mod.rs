use crate::read_file;
use std::fmt;
use std::fmt::Debug;

#[derive(Debug)]
struct Voxel {
    x: i32,
    y: i32,
    z: i32,
}

struct Space {
    width: i32,
    height: i32,
    depth: i32,
    voxels: Vec<SpaceVoxel>,
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Clone)]
struct SpaceVoxel {
    visited: bool,
    is_outside: bool,
}

impl Debug for SpaceVoxel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.visited)
    }
}

impl Debug for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut space = vec![];
        space.push(format!(
            "Width: {}/Height: {}/Depth: {}",
            self.width, self.height, self.depth
        ));
        for z in 0..self.depth {
            for y in 0..self.height {
                let mut row = vec![];
                for x in 0..self.width {
                    row.push(
                        match self
                            .get_visited(x + self.x, y + self.y, z + self.z)
                            .unwrap()
                        {
                            true => '#',
                            false => '.',
                        },
                    );
                }
                space.push(row.into_iter().collect::<String>())
            }
            space.push("".to_string());
        }

        write!(f, "{}", space.join("\n"))
    }
}

impl SpaceVoxel {
    pub fn new() -> SpaceVoxel {
        SpaceVoxel {
            visited: false,
            is_outside: false,
        }
    }
}

impl Space {
    pub fn new(x: i32, y: i32, z: i32, width: i32, height: i32, depth: i32) -> Space {
        Space {
            width,
            height,
            depth,
            x,
            y,
            z,
            voxels: vec![SpaceVoxel::new(); (width * height * depth) as usize],
        }
    }

    pub fn set_visited(&mut self, x: i32, y: i32, z: i32, visited: bool) {
        if self.coordinates_valid(x, y, z) {
            let index = self.get_vec_index(x, y, z);
            self.voxels[index].visited = visited;
        }
    }
    pub fn set_is_outside(&mut self, x: i32, y: i32, z: i32, is_outside: bool) {
        if self.coordinates_valid(x, y, z) {
            let index = self.get_vec_index(x, y, z);
            self.voxels[index].is_outside = is_outside;
        }
    }
    pub fn get_visited(&self, x: i32, y: i32, z: i32) -> Option<bool> {
        if self.coordinates_valid(x, y, z) {
            return Some(self.voxels[self.get_vec_index(x, y, z)].visited);
        }
        None
    }

    fn get_vec_index(&self, x: i32, y: i32, z: i32) -> usize {
        (((x - self.x) * self.height + y - self.y) * self.depth + (z - self.z)) as usize
    }

    pub fn coordinates_valid(&self, x: i32, y: i32, z: i32) -> bool {
        x >= self.x
            && x < self.x + self.width
            && y >= self.y
            && y < self.y + self.height
            && z >= self.z
            && z < self.z + self.depth
    }
}

impl Voxel {
    pub fn new(x: i32, y: i32, z: i32) -> Voxel {
        Voxel { x, y, z }
    }

    pub fn manhatten(&self, other: &Voxel) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

fn parse_input(input: String) -> Vec<Voxel> {
    let mut voxels = vec![];

    for line in input.lines() {
        let mut parts = line.split(",");
        let x = parts.next().unwrap().parse().unwrap();
        let y = parts.next().unwrap().parse().unwrap();
        let z = parts.next().unwrap().parse().unwrap();
        voxels.push(Voxel::new(x, y, z));
    }

    voxels
}

fn count_hidden_faces(voxels: &Vec<Voxel>) -> usize {
    let mut hidden_faces = 0;
    for i in 0..voxels.len() {
        for j in i + 1..voxels.len() {
            if voxels[i].manhatten(&voxels[j]) == 1 {
                hidden_faces += 1;
            }
        }
    }
    hidden_faces
}

pub fn run() {
    let voxels = parse_input(read_file("input/day18.txt"));

    let hidden_faces = count_hidden_faces(&voxels);
    println!(
        "It looks like the lava surface has {} units ",
        6 * voxels.len() - 2 * hidden_faces
    );
}

pub fn run2() {
    let mut voxels = parse_input(read_file("input/day18.txt"));

    let min_x = voxels.iter().map(|voxel| voxel.x).min().unwrap() - 1;
    let max_x = voxels.iter().map(|voxel| voxel.x).max().unwrap() + 1;
    let width = max_x - min_x + 1;

    let min_y = voxels.iter().map(|voxel| voxel.y).min().unwrap() - 1;
    let max_y = voxels.iter().map(|voxel| voxel.y).max().unwrap() + 1;
    let height = max_y - min_y + 1;

    let min_z = voxels.iter().map(|voxel| voxel.z).min().unwrap() - 1;
    let max_z = voxels.iter().map(|voxel| voxel.z).max().unwrap() + 1;
    let depth = max_z - min_z + 1;

    let mut space = Space::new(min_x, min_y, min_z, width, height, depth);
    for voxel in &voxels {
        space.set_visited(voxel.x, voxel.y, voxel.z, true);
    }
    //println!("{:?}", space);
    floodfill(&mut space, min_x, min_y, min_z);
    //println!("{:?}", space);

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            for z in min_z..=max_z {
                if !space.get_visited(x, y, z).unwrap() {
                    voxels.push(Voxel { x, y, z });
                }
            }
        }
    }
    let hidden_faces = count_hidden_faces(&voxels);
    println!(
        "An this is the size of the exterior area: {} units",
        6 * voxels.len() - 2 * hidden_faces
    );
}

fn floodfill(space: &mut Space, x: i32, y: i32, z: i32) {
    if !space.coordinates_valid(x, y, z) {
        return;
    }
    space.set_is_outside(x, y, z, true);
    space.set_visited(x, y, z, true);
    for i in [
        (0, 1, 0),
        (0, -1, 0),
        (-1, 0, 0),
        (1, 0, 0),
        (0, 0, 1),
        (0, 0, -1),
    ] {
        match space.get_visited(x + i.0, y + i.1, z + i.2) {
            None => {}
            Some(visited) => {
                if !visited {
                    floodfill(space, x + i.0, y + i.1, z + i.2);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_floodfill() {
        let mut space = Space::new(0, 0, 0, 3, 2, 1);
        space.set_visited(1, 0, 0, true);
        space.set_visited(1, 1, 0, true);
        floodfill(&mut space, 0, 0, 0);
        println!("{:?}", space);
    }
}
