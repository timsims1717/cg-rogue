use amethyst::core::ecs::Entity;
use crate::components::{MovementOptions, ActionOption};
use crate::components::ActionOption::Interact;

pub struct Player {
    pub character: Entity,
    pub mode: ActionOption,
    pub input: PlayerInput,
}

impl Player {
    pub fn new(character: Entity) -> Player {
        Player {
            character,
            mode: Interact,
            input: PlayerInput::default(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct PlayerInput {
    pub l_click: bool,
    pub r_click: bool,
    pub tile: Option<(usize, usize)>,
    // pub button: Option<Button>,
    // pub card: Option<Card>,
    // pub ui_el: Option<UIElement>,
}