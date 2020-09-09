use amethyst::core::ecs::{Component, DenseVecStorage};
use crate::components::{AITargetChoice, HexCoords};
use crate::util::PathEnds;

#[derive(Debug, Clone)]
pub struct MovementAction {
    path: Vec<HexCoords>,
    path_i: usize,
    options: MovementOptions,
    start: bool,
    first: bool,
}

impl MovementAction {
    pub fn new(path: Vec<HexCoords>, options: MovementOptions) -> MovementAction {
        MovementAction {
            path,
            path_i: 0,
            options,
            start: false,
            first: true,
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

    pub fn execute(&mut self) {
        self.start = true;
    }

    pub fn is_go(&self) -> bool {
        self.start
    }

    pub fn first_run(&mut self) -> bool {
        if self.first {
            self.first = false;
            return true;
        }
        false
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
    options: AttackOptions,
    go: bool,
}

impl AttackAction {
    pub fn new(area: Vec<HexCoords>, options: AttackOptions) -> AttackAction {
        AttackAction {
            area,
            options,
            go: false,
        }
    }

    pub fn get_area(&self) -> &Vec<HexCoords> {
        &self.area
    }

    pub fn execute(&mut self) {
        self.go = true;
    }

    pub fn is_go(&self) -> bool {
        self.go
    }
}

impl Component for AttackAction {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug, Clone)]
pub struct AttackOptions {
    pub range: usize,
    pub area: Vec<HexCoords>,
    pub damage: DamageOptions,
    pub line: bool,
    pub path: bool,
    pub width: usize,
    pub self_damage: bool,
}

impl AttackOptions {
    pub fn basic(range: usize, dmg: usize) -> AttackOptions {
        AttackOptions{
            range,
            area: vec![],
            damage: DamageOptions{
                normal: dmg,
                area: 1.0,
            },
            line: false,
            path: false,
            width: 0,
            self_damage: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DamageOptions {
    pub normal: usize,
    pub area: f32,
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
}