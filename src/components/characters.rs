use amethyst::core::ecs::{Component, DenseVecStorage};
use uuid::Uuid;
use crate::components::{Action, DamageType};

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
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug, Clone)]
pub struct Character {
    pub actions: Vec<Action>,
}

impl Character {
    pub fn new() -> Character {
        Character{
            actions: vec![],
        }
    }
}

impl Component for Character {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Diplomacy {
    Player,
    Ally,
    Enemy,
    Neutral,
    Unknown,
}

impl Component for Diplomacy {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug, Clone)]
pub struct Acting {}

impl Component for Acting {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug, Clone)]
pub struct Health {
    pub max: usize,
    pub current: usize,
    pub temp: usize,
}

impl Health {
    pub fn new(max: usize) -> Health {
        Health{
            max,
            current: max,
            temp: 0,
        }
    }
}

impl Component for Health {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug, Clone)]
pub struct Resistance {
    pub immunities: Vec<DamageType>,
    pub resistances: Vec<DamageType>,
    pub vulnerabilities: Vec<DamageType>,
}

impl Component for Resistance {
    type Storage = DenseVecStorage<Self>;
}