use amethyst::core::ecs::{Join, System, WriteExpect, ReadExpect, WriteStorage, ReadStorage};
use crate::resources::{Game, Player};
use crate::components::{Movement, AI, PC, Character};
use crate::resources::Phase::{PlayerAction, AIAction, AITurn};

pub struct CharacterActionSystem;

impl<'s> System<'s> for CharacterActionSystem {
    type SystemData = (
        WriteExpect<'s, Game>,
        WriteStorage<'s, Movement>,
        ReadStorage<'s, Character>,
        ReadStorage<'s, PC>,
        ReadStorage<'s, AI>,
    );

    fn run(&mut self, (
        mut game,
        mut movements,
        characters,
        pc,
        ai,
    ): Self::SystemData) {
        if game.phase == PlayerAction {
            let mut action_left = false;
            for (movement, _, _) in (&mut movements, &characters, &pc).join() {
                movement.execute();
                action_left = true;
            }
            if !action_left {
                game.phase = AIAction;
            }
        } else if game.phase == AIAction {
            let mut action_left = false;
            for (movement, _, _) in (&mut movements, &characters, &ai).join() {
                movement.execute();
                action_left = true;
            }
            if !action_left {
                game.phase = AITurn;
            }
        }
    }
}