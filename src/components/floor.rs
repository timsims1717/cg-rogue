use amethyst::core::ecs::{Component, Entity, DenseVecStorage};
use crate::util::{distance_world, PathEnds};
use crate::resources::Floor;

#[derive(Debug, Clone)]
pub struct FloorTile {
    pub sprite_index: usize,
    pub character: Option<Entity>,
    pub solid: bool,
    pub walkable: bool,
}

impl Component for FloorTile {
    type Storage = DenseVecStorage<Self>;
}

// a ui element for a tile
#[derive(Debug, Clone)]
pub struct TileUIElement {
    pub x: usize,
    pub y: usize,
    pub hover: bool,
}

impl Component for TileUIElement {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialOrd, PartialEq)]
pub struct HexCoords {
    pub x: usize,
    pub y: usize,
}

impl HexCoords {
    pub fn new(x: usize, y: usize) -> HexCoords {
        HexCoords{
            x, y,
        }
    }

    pub fn distance_world(&self, other: &HexCoords) -> u32 {
        distance_world(&PathEnds{
            a: self.clone(),
            b: other.clone(),
        })
    }

    pub fn successors(&self, floor: &Floor) -> Vec<(HexCoords, u32)> {
        floor.neighbors_simple(self.x, self.y).into_iter()
            .map(|(x, y)| (HexCoords::new(x, y), 1)).collect()
    }
}

impl Component for HexCoords {
    type Storage = DenseVecStorage<Self>;
}

