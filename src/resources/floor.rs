use amethyst::core::ecs::{Entity, WriteStorage};
use crate::components::FloorTile;
use crate::util::{distance, PathEnds};
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct FloorSize {
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Clone)]
pub struct Floor {
    pub dimensions: FloorSize,
    tiles: Vec<Vec<Entity>>,
}

impl Floor {
    pub fn new(size: FloorSize) -> Floor {
        Floor{
            dimensions: size,
            tiles: Vec::new(),
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Entity {
        self.tiles[y][x]
    }

    pub fn append(&mut self, element: Entity) {
        for (i, row) in self.tiles.iter().enumerate() {
            if row.len() >= self.dimensions.width {
                continue;
            }
            self.tiles[i].push(element);
            return;
        }
        self.tiles.push(Vec::new());
        self.tiles.last_mut().unwrap().push(element);
    }

    pub fn neighbors_simple(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut v = Vec::new();
        if y > 0 {
            v.push((x, y-1));
        }
        if x < self.dimensions.width - 1 {
            v.push((x+1, y));
        }
        if y < self.dimensions.height - 1 {
            v.push((x, y+1));
        }
        if x > 0 {
            v.push((x-1, y));
        }
        if x % 2 == 0 {
            if y > 0 && x < self.dimensions.width - 1 {
                v.push((x+1, y-1));
            }
            if x > 0 && y > 0 {
                v.push((x-1, y-1));
            }
        } else {
            if x < self.dimensions.width - 1 && y < self.dimensions.height - 1 {
                v.push((x+1, y+1));
            }
            if x > 0 && y < self.dimensions.height - 1 {
                v.push((x-1, y+1));
            }
        }
        v
        // vec![(x, y-1), (x+1, y-1), (x+1, y), (x, y+1), (x-1, y), (x-1, y-1)]
        // vec![(x, y-1), (x+1, y), (x+1, y+1), (x, y+1), (x-1, y+1), (x-1, y)]
    }
}

#[derive(Clone, Eq, Hash, Ord, PartialOrd, PartialEq)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Pos {
        Pos{
            x, y,
        }
    }

    pub fn just_point(x: usize, y:usize) -> Pos {
        Pos{
            x, y,
        }
    }

    pub fn distance(&self, other: &Pos) -> u32 {
        distance(PathEnds{
            a_x: self.x,
            a_y: self.y,
            b_x: other.x,
            b_y: other.y,
        })
    }

    pub fn successors(&self, floor: &Floor) -> Vec<(Pos, u32)> {
        floor.neighbors_simple(self.x, self.y).into_iter()
            .map(|(x, y)| (Pos::new(x, y), 1)).collect()
    }
}

#[derive(Debug, Clone)]
pub struct PreFloor {
    pub dimensions: FloorSize,
    pub tiles: Vec<Vec<FloorTile>>,
}

impl PreFloor {
    pub fn new() -> PreFloor {
        PreFloor {
            dimensions: FloorSize {
                width: 20,
                height: 20,
            },
            tiles: vec![vec![FloorTile {
                tile_index: 0,
            }; 20]; 20],
        }
    }
}