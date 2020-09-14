use amethyst::core::ecs::{Join, System, ReadStorage, Entities, ReadExpect, LazyUpdate, WriteStorage};
use crate::components::{AI, Character, MovementAction, FloorTile, ai_decide, AITargetChoice, Action, AttackAction, HexCoords};
use crate::resources::{Game, Floor};
use amethyst::core::shred::WriteExpect;
use crate::resources::Phase::{AIDecisionPhase, PlayerDecisionPhase, AIActionPhase};
use rand::{thread_rng, Rng};
use crate::components::{ActionOption, Diplomacy};
use crate::util::{shortest_path, PathEnds, distance};

pub struct AIDecideSystem;

impl<'s> System<'s> for AIDecideSystem {
    type SystemData = (
        ReadStorage<'s, AI>,
        WriteStorage<'s, Character>,
        ReadStorage<'s, HexCoords>,
        ReadStorage<'s, Diplomacy>,
        WriteExpect<'s, Game>,
        ReadExpect<'s, Floor>,
    );

    fn run(&mut self, (
        all_ai,
        mut characters,
        hexes,
        diplomacies,
        mut game,
        floor,
    ): Self::SystemData) {
        if game.phase == AIDecisionPhase {
            let mut all_characters: Vec<(HexCoords, Diplomacy)> = Vec::new();
            for (_, d, hex) in (&characters, &diplomacies, &hexes).join() {
                all_characters.push((hex.clone(), d.clone()));
            }
            for (ai, character, hex) in (&all_ai, &mut characters, &hexes).join() {
                if !ai.action_choices.is_empty() {
                    if let(Some(decision), _) = ai_decide(&ai.tree, &hex, &all_characters) {
                        let choice = &ai.action_choices[decision];
                        let mut curr_hex = hex.clone();
                        for ai_option in choice.sequence.iter() {
                            if let Some(target_hex) = match ai_option.target {
                                AITargetChoice::ClosestAlly(min, max) => {
                                    let (mut best, mut t_x, mut t_y) = (0,0,0);
                                    for (h, d) in all_characters.iter() {
                                        if *d == Diplomacy::Ally || *d == Diplomacy::Player {
                                            let dist = distance(&PathEnds {
                                                a: curr_hex.clone(),
                                                b: h.clone(),
                                            }) as usize;
                                            if dist >= min && dist < max && best == 0 || best > dist {
                                                best = dist;
                                                t_x = h.x;
                                                t_y = h.y;
                                                if dist == min {
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                    if t_x != 0 || t_y != 0 {
                                        Some(HexCoords{ x: t_x, y: t_y })
                                    } else {
                                        None
                                    }
                                },
                                _ => None
                            } {
                                character.actions.push(match &ai_option.option {
                                    ActionOption::Move(m) => {
                                        if let Some(mut path) = shortest_path(PathEnds{
                                                a: curr_hex.clone(),
                                                b: target_hex,
                                            }, &floor) {
                                            path.truncate(m.range + 1);
                                            if path.len() > 0 {
                                                curr_hex = path[path.len() - 1].clone();
                                            }
                                            Action::Move(MovementAction::new(path, m.clone()))
                                        } else {
                                            Action::NoAction
                                        }
                                    },
                                    ActionOption::Attack(a) => {
                                        Action::Attack(AttackAction::new(vec![target_hex], a.clone()))
                                    },
                                    _ => Action::NoAction,
                                });
                            }
                        }
                    }
                }
            }
            game.phase = PlayerDecisionPhase;
        }
    }
}