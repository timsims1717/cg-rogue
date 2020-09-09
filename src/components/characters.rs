use amethyst::core::ecs::{Component, DenseVecStorage};
use crate::components::{MovementOptions, AttackOptions, AIActionOptionSeq, Action, HexCoords, ActionOption};
use rand::{thread_rng, Rng};
use crate::util::{PathEnds, distance};
use uuid::Uuid;
use crate::components::ActionOption::Interact;

#[derive(Debug, Clone)]
pub struct Character {
    pub id: Uuid,
    pub actions: Vec<Action>,
}

impl Character {
    pub fn new() -> Character {
        Character{
            id: Uuid::new_v4(),
            actions: vec![],
        }
    }
}

impl Component for Character {
    type Storage = DenseVecStorage<Self>;
}


#[derive(Debug, Clone)]
pub struct AI {
    pub action_choices: Vec<AIActionOptionSeq>,
    pub tree: Option<Vec<AITree>>,
}

impl Component for AI {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug, Clone)]
pub struct AITree {
    pub require: AIRequire,
    pub decision: Option<usize>,
    pub tree: Option<Vec<AITree>>,
}

#[derive(Debug, Clone)]
pub enum AIRequire {
    //Previous(usize),
    //Sequence(Vec<usize>),
    Random(usize),
    Target(AITargetDecision),
}

#[derive(Debug, Clone)]
pub enum AITargetDecision {
    AnyAlly(usize, usize),
    //AnyPlayer(usize, usize),
    //AnyEnemy(usize, usize),
    //AnyCharacter(usize, usize),
}

#[derive(Debug, Clone)]
pub enum AITargetChoice {
    RandomAlly(usize, usize),
    ClosestAlly(usize, usize),
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

pub fn ai_decide(node: &Option<Vec<AITree>>, hex: &HexCoords, all_characters: &Vec<(HexCoords, Diplomacy)>) -> (Option<usize>, bool) {
    if let Some(branch) = node {
        for t in branch.iter() {
            let passed = match &t.require {
                AIRequire::Random(n) => (thread_rng().gen_range(0, 100) < *n),
                AIRequire::Target(target) => {
                    match target {
                        AITargetDecision::AnyAlly(min, max) => {
                            all_characters.iter().any(|(h, d)| {
                                    let dist = distance(&PathEnds {
                                        a: hex.clone(),
                                        b: h.clone(),
                                    }) as usize;
                                    dist >= *min && dist < *max && (*d == Diplomacy::Ally || *d == Diplomacy::Player)
                                }
                            )
                        },
                        _ => false,
                    }
                }
                _ => false,
            };
            if passed {
                if let Some(d) = &t.decision {
                    return (Some(*d), false);
                } else {
                    let (d, c) = ai_decide(&t.tree, hex, all_characters);
                    if !c {
                        return (d, c);
                    }
                    if let Some(_) = d {
                        return (d, c);
                    }
                }
            }
        }
    }
    (None, false)
}

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

#[derive(Debug, Clone)]
pub struct Acting {}

impl Component for Acting {
    type Storage = DenseVecStorage<Self>;
}