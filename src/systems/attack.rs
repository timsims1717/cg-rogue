use amethyst::core::ecs::{Join, System, Entities, WriteStorage, Read, ReadExpect, LazyUpdate};
use crate::components::{AttackAction, Character};
use amethyst::core::Time;

pub struct AttackActionSystem;

impl<'s> System<'s> for AttackActionSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, AttackAction>,
        WriteStorage<'s, Character>,
        Read<'s, Time>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, (
        entities,
        mut attacks,
        mut characters,
        _time,
        lazy_update,
    ): Self::SystemData) {
        for (attacker, attack) in (&entities, &mut attacks).join() {
            if attack.is_go() {
                for (x, y) in attack.get_area().iter() {
                    for (target, character) in (&entities, &mut characters).join() {
                        if character.x == *x && character.y == *y {
                            entities.delete(target);
                        }
                    }
                }
                lazy_update.remove::<AttackAction>(attacker);
            }
        }
    }
}