use amethyst::core::ecs::{Entity, WriteStorage, ReadStorage};
use crate::components::{FloorTile, Character, HexCoords};
use crate::util::{distance, PathEnds, distance_world};
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;

/// A simple struct to hold the dimensions of the floor
#[derive(Debug, Clone)]
pub struct FloorSize {
    pub width: usize,
    pub height: usize,
}

/// The struct that holds all the information of the current floor
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

    pub fn get(&self, c: &HexCoords) -> Entity {
        self.tiles[c.y][c.x]
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
                sprite_index: 0,
                character: None,
                solid: false,
                walkable: true,
            }; 20]; 20],
        }
    }
}