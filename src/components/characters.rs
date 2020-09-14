use amethyst::core::ecs::{Component, DenseVecStorage, VecStorage, NullStorage};
use crate::components::{Action, DamageType};

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
    type Storage = VecStorage<Self>;
}

#[derive(Default, Debug, Clone)]
pub struct Acting {}

impl Component for Acting {
    type Storage = NullStorage<Self>;
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