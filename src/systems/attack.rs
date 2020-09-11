use amethyst::core::ecs::{Join, System, Entities, WriteStorage, Read, ReadExpect, LazyUpdate, ReadStorage};
use crate::components::{AttackAction, Character, HexCoords, Health, Acting, DamageTileAction, ID};
use amethyst::core::Time;

pub struct AttackActionSystem;

impl<'s> System<'s> for AttackActionSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, ID>,
        ReadStorage<'s, Acting>,
        ReadStorage<'s, AttackAction>,
        Read<'s, Time>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, (
        entities,
        ids,
        acting,
        attacks,
        _time,
        lazy_update,
    ): Self::SystemData) {
        for (attacker, id, attack, _) in (&entities, &ids, &attacks, &acting).join() {
            for hex in attack.get_area().iter() {
                let damage_tile = entities.create();
                lazy_update.insert(damage_tile, DamageTileAction{
                    options: attack.options.damage.clone(),
                    source: id.clone(),
                });
                lazy_update.insert(damage_tile, hex.clone());
            }
            lazy_update.remove::<AttackAction>(attacker);
            lazy_update.remove::<Acting>(attacker);
        }
    }
}