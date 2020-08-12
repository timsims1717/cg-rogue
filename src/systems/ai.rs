use amethyst::core::ecs::{Join, System, ReadStorage, Entities, ReadExpect, LazyUpdate, WriteStorage};
use crate::components::{AI, Character, MovementAction, FloorTile, ai_decide, AITargetChoice, Action, AttackAction};
use crate::resources::{Player, Game, Floor};
use amethyst::core::shred::WriteExpect;
use crate::resources::Phase::{AIDecisionPhase, PlayerDecisionPhase, AIActionPhase};
use rand::{thread_rng, Rng};
use crate::components::{ActionOption, Diplomacy};
use crate::util::{shortest_path, PathEnds, distance};

pub struct AIDecideSystem;

impl<'s> System<'s> for AIDecideSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, AI>,
        WriteStorage<'s, Character>,
        ReadExpect<'s, Player>,
        WriteExpect<'s, Game>,
        ReadExpect<'s, Floor>,
        ReadStorage<'s, FloorTile>,
        WriteStorage<'s, MovementAction>,
    );

    fn run(&mut self, (
        entities,
        all_ai,
        mut characters,
        player,
        mut game,
        floor,
        tiles,
        mut movements,
    ): Self::SystemData) {
        if game.phase == AIDecisionPhase {
            let mut all_characters: Vec<(usize, usize, Diplomacy)> = Vec::new();
            for (ch) in (&characters).join() {
                all_characters.push((ch.x, ch.y, ch.d.clone()));
            }
            let pc = characters.get(player.character).unwrap();
            for (ai, character) in (&all_ai, &mut characters).join() {
                if !ai.action_choices.is_empty() {
                    if let(Some(decision), _) = ai_decide(&ai.tree, (character.x, character.y), &all_characters) {
                        let choice = &ai.action_choices[decision];
                        for ai_option in choice.sequence.iter() {
                            if let Some((target_x, target_y)) = match ai_option.target {
                                AITargetChoice::ClosestAlly(min, max) => {
                                    let (mut best, mut t_x, mut t_y) = (0,0,0);
                                    for (x, y, d) in all_characters.iter() {
                                        if *d == Diplomacy::Ally || *d == Diplomacy::Player {
                                            let dist = distance(&PathEnds {
                                                a_x: character.x,
                                                a_y: character.y,
                                                b_x: *x,
                                                b_y: *y,
                                            }) as usize;
                                            if dist >= min && dist < max && best == 0 || best > dist {
                                                best = dist;
                                                t_x = *x;
                                                t_y = *y;
                                                if dist == min {
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                    if t_x != 0 || t_y != 0 {
                                        Some((t_x, t_y))
                                    } else {
                                        None
                                    }
                                },
                                _ => None
                            } {
                                character.actions.push(match &ai_option.option {
                                    ActionOption::Move(m) => {
                                        if let Some(mut path) = shortest_path(PathEnds{
                                                a_x: character.x, a_y: character.y, b_x: target_x, b_y: target_y
                                            }, &floor) {
                                            path.truncate(m.range + 1);
                                            Action::Move(MovementAction::new(path, m.clone()))
                                        } else {
                                            Action::NoAction
                                        }
                                    },
                                    ActionOption::Attack(a) => {
                                        Action::Attack(AttackAction::new(vec![(target_x, target_y)], a.clone()))
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