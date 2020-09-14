use amethyst::core::ecs::{DenseVecStorage, Component, NullStorage};

#[derive(Default, Debug, Clone)]
pub struct Clickable {}
impl Component for Clickable {
    type Storage = NullStorage<Self>;
}

#[derive(Default, Debug, Clone)]
pub struct Hoverable {}
impl Component for Hoverable {
    type Storage = NullStorage<Self>;
}

#[derive(Default, Debug, Clone)]
pub struct Hovered {}
impl Component for Hovered {
    type Storage = NullStorage<Self>;
}