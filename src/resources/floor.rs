use amethyst::core::ecs::Entity;
use crate::components::FloorTile;

#[derive(Default, Debug, Clone)]
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
                width: 10,
                height: 10,
            },
            tiles: vec![vec![FloorTile {
                tile_index: 0,
            }; 10]; 10],
        }
    }
}