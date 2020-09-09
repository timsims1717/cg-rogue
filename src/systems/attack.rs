use amethyst::core::ecs::{Join, System, Entities, WriteStorage, Read, ReadExpect, LazyUpdate, ReadStorage};
use crate::components::{AttackAction, Character, HexCoords};
use amethyst::core::Time;

pub struct AttackActionSystem;

impl<'s> System<'s> for AttackActionSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, AttackAction>,
        ReadStorage<'s, Character>,
        WriteStorage<'s, HexCoords>,
        Read<'s, Time>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, (
        entities,
        mut attacks,
        characters,
        mut hexes,
        _time,
        lazy_update,
    ): Self::SystemData) {
        for (attacker, attack) in (&entities, &mut attacks).join() {
            if attack.is_go() {
                for hex_area in attack.get_area().iter() {
                    for (target, character, hex) in (&entities, &characters, &mut hexes).join() {
                        if hex.x == hex_area.x && hex.y == hex_area.y {
                            entities.delete(target);
                        }
                    }
                }
                lazy_update.remove::<AttackAction>(attacker);
            }
        }
    }
}