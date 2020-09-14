use amethyst::core::ecs::{Component, DenseVecStorage};

#[derive(Debug, Clone)]
pub struct Card {

}

impl Component for Card {
    type Storage = DenseVecStorage<Self>;
}