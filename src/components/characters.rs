use amethyst::core::ecs::{Component, DenseVecStorage};
use crate::components::MovementOptions;

#[derive(Debug, Clone)]
pub struct Character {
    pub x: usize,
    pub y: usize,
}

impl Component for Character {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug, Clone)]
pub struct AI {
    pub actions: Vec<Mode>,
}

impl Component for AI {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug, Clone)]
pub struct PC {}

impl Component for PC {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug, Clone)]
pub enum Mode {
    Interact,
    Move(MovementOptions),
}

impl Component for Mode {
    type Storage = DenseVecStorage<Self>;
}