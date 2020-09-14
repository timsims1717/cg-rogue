use amethyst::core::ecs::{Component, DenseVecStorage};
use crate::components::{AITargetChoice, HexCoords, ID};
use crate::util::PathEnds;
use crate::resources::Floor;

#[derive(Debug, Clone)]
pub struct MovementAction {
    path: Vec<HexCoords>,
    path_i: usize,
    pub options: MovementOptions,
}

impl MovementAction {
    pub fn new(path: Vec<HexCoords>, options: MovementOptions) -> MovementAction {
        MovementAction {
            path,
            path_i: 0,
            options,
        }
    }

    pub fn path_complete(&self) -> bool {
        self.path_i >= self.path.len() - 1
    }

    pub fn get_move(&mut self) -> PathEnds {
        let a = self.path[self.path_i].clone();
        let b = self.path[self.path_i+1].clone();
        PathEnds{
            a, b
        }
    }

    pub fn next_move(&mut self) {
        self.path_i += 1;
    }
}

impl Component for MovementAction {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug, Clone)]
pub struct MovementOptions {
    pub range: usize,
    // flying, incorporeal will go here
    pub line: bool,
}

impl MovementOptions {
    pub fn basic(range: usize) -> MovementOptions {
        MovementOptions{
            range,
            line: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AttackAction {
    area: Vec<HexCoords>,
    pub options: AttackOptions,
}

impl AttackAction {
    pub fn new(area: Vec<HexCoords>, options: AttackOptions) -> AttackAction {
        AttackAction {
            area,
            options,
        }
    }

    pub fn get_area(&self) -> &Vec<HexCoords> {
        &self.area
    }
}

impl Component for AttackAction {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug, Clone)]
pub struct AttackOptions {
    pub range: usize,
    pub area: Option<AreaOptions>,
    pub damage: DamageOptions,
}

impl AttackOptions {
    pub fn basic(range: usize, amount: usize, d_type: DamageType) -> AttackOptions {
        AttackOptions{
            range,
            area: None,
            damage: DamageOptions{
                amount,
                d_type,
                retaliate: false,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct DamageTileAction {
    pub options: DamageOptions,
    pub source: ID,
}

impl Component for DamageTileAction {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug, Clone)]
pub struct DamageActions {
    pub d: Vec<DamageOptions>,
}

impl Component for DamageActions {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug, Clone)]
pub struct DamageOptions {
    pub amount: usize,
    pub d_type: DamageType,
    pub retaliate: bool,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DamageType {
    Pierce,
    Slash,
    Blunt,
    Heat,
    Toxic,
    Electric,
    Cold,
    Mental,
}

#[derive(Debug, Clone)]
pub struct AIActionOptionSeq {
    pub sequence: Vec<AIActionOption>,
}

#[derive(Debug, Clone)]
pub struct AIActionOption {
    pub option: ActionOption,
    pub target: AITargetChoice,
}

#[derive(Debug, Clone)]
pub enum ActionOption {
    Interact,
    Move(MovementOptions),
    Attack(AttackOptions),
}

impl Component for ActionOption {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug, Clone)]
pub enum Action {
    NoAction,
    Move(MovementAction),
    Attack(AttackAction),
    Damage(DamageTileAction),
}

#[derive(Debug, Clone)]
pub struct AreaOptions {
    pub area: Vec<HexCoords>,
    pub line: bool,
    pub path: bool,
    pub width: usize,
}