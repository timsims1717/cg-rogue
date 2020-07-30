use amethyst::core::ecs::{Component, DenseVecStorage};

#[derive(Debug, Clone)]
pub struct FloorTile {
    pub tile_index: usize,
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