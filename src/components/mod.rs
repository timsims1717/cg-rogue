
mod actions;
mod ai;
mod cards;
mod characters;
mod floor;
mod player;
mod ui;

pub use self::actions::*;
pub use self::ai::*;
pub use self::cards::*;
pub use self::characters::*;
pub use self::floor::*;
pub use self::player::*;
pub use self::ui::*;

use amethyst::core::ecs::{Component, VecStorage};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Name {
    name: String,
}
impl Name {
    pub fn new(name: String) -> Name {
        Name{
            name,
        }
    }

    pub fn get(&self) -> String {
        self.name.clone()
    }
}
impl Component for Name {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, Clone)]
pub struct ID {
    id: Uuid,
}
impl ID {
    pub fn new() -> ID {
        ID{
            id: Uuid::new_v4(),
        }
    }

    pub fn get(&self) -> Uuid {
        self.id
    }
}
impl Component for ID {
    type Storage = VecStorage<Self>;
}