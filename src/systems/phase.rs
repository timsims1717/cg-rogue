use amethyst::core::ecs::{Join, System, WriteExpect, ReadExpect, WriteStorage, ReadStorage, Entities, LazyUpdate};
use crate::resources::Game;
use crate::components::{MovementAction, AI, Character, AttackAction, Action, Player, Acting};
use crate::resources::Phase::{PlayerActionPhase, AIActionPhase, AIDecisionPhase};

pub struct PhaseSystem;

impl<'s> System<'s> for PhaseSystem {
    type SystemData = (
        Entities<'s>,
        WriteExpect<'s, Game>,
        WriteStorage<'s, Character>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, AI>,
        ReadExpect<'s, LazyUpdate>,
        WriteStorage<'s, Acting>,
    );

    fn run(&mut self, (
        entities,
        mut game,
        mut characters,
        players,
        ai,
        lazy_update,
        mut acting,
    ): Self::SystemData) {
        let mut action_left = false;
        for (entity, character) in (&entities, &mut characters).join() {
            if (game.phase == PlayerActionPhase && players.get(entity).is_some())
                || (game.phase == AIActionPhase && ai.get(entity).is_some()) {
                if !character.actions.is_empty() {
                    if acting.get(entity).is_none() {
                        match character.actions.remove(0) {
                            Action::Move(mut m) => {
                                lazy_update.insert(entity, m);
                                acting.insert(entity, Acting {});
                            },
                            Action::Attack(mut a) => {
                                lazy_update.insert(entity, a);
                                acting.insert(entity, Acting {});
                            },
                            _ => {},
                        };
                    }
                }
                action_left = action_left || acting.get(entity).is_some() || !character.actions.is_empty();
            }
        }
        if !action_left {
            if game.phase == PlayerActionPhase {
                game.phase = AIActionPhase;
            } else if game.phase == AIActionPhase {
                game.phase = AIDecisionPhase;
            }
        }
    }
}