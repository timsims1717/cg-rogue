use amethyst::core::ecs::{Component, DenseVecStorage};

#[derive(Debug, Clone)]
pub struct Character {
    pub x: usize,
    pub y: usize,
}

impl Component for Character {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug, Clone)]
pub struct Movement {
    pub path: Vec<(usize, usize)>,
    pub path_i: usize,
    pub smooth: bool,
}

impl Component for Movement {
    type Storage = DenseVecStorage<Self>;
}