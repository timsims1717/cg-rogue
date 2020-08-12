use amethyst::core::ecs::{Join, System, WriteExpect, ReadExpect, WriteStorage, ReadStorage, Entities, LazyUpdate};
use crate::resources::{Game, Player};
use crate::components::{MovementAction, AI, PC, Character, AttackAction, Action};
use crate::resources::Phase::{PlayerActionPhase, AIActionPhase, AIDecisionPhase};

pub struct PhaseSystem;

impl<'s> System<'s> for PhaseSystem {
    type SystemData = (
        Entities<'s>,
        WriteExpect<'s, Game>,
        WriteStorage<'s, MovementAction>,
        WriteStorage<'s, AttackAction>,
        WriteStorage<'s, Character>,
        ReadStorage<'s, PC>,
        ReadStorage<'s, AI>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, (
        entities,
        mut game,
        mut movements,
        mut attacks,
        mut characters,
        pc,
        ai,
        lazy_update,
    ): Self::SystemData) {
        if game.phase == PlayerActionPhase {
            let mut action_left = false;
            for (movement, _, _) in (&mut movements, &characters, &pc).join() {
                movement.execute();
                action_left = true;
            }
            for (attack, _, _) in (&mut attacks, &characters, &pc).join() {
                attack.execute();
                action_left = true;
            }
            if !action_left {
                game.phase = AIActionPhase;
            }
        } else if game.phase == AIActionPhase {
            let mut action_left = false;
            for (entity, character, _) in (&entities, &mut characters, &ai).join() {
                if !character.actions.is_empty() {
                    if !character.acting {
                        character.acting = match character.actions.remove(0) {
                            Action::Move(mut m) => {
                                m.execute();
                                lazy_update.insert(entity, m);
                                true
                            },
                            Action::Attack(mut a) => {
                                a.execute();
                                lazy_update.insert(entity, a);
                                true
                            },
                            _ => false,
                        };
                    }
                }
                action_left = action_left || character.acting || !character.actions.is_empty();
            }
            if !action_left {
                game.phase = AIDecisionPhase;
            }
        }
    }
}