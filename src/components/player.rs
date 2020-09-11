use crate::components::{ActionOption, HexCoords};
use crate::components::ActionOption::Interact;
use amethyst::core::ecs::{Component, DenseVecStorage};

#[derive(Debug, Clone)]
pub struct Player {
    pub mode: ActionOption,
    pub input: PlayerInput,
}

impl Player {
    pub fn new() -> Player {
        Player {
            mode: Interact,
            input: PlayerInput::default(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct PlayerInput {
    pub l_click: bool,
    pub r_click: bool,
    pub tile: Option<HexCoords>,
    // pub button: Option<Button>,
    // pub card: Option<Card>,
    // pub ui_el: Option<UIElement>,
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}