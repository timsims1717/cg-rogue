use amethyst::core::ecs::{Join, System, ReadStorage, Entities, WriteStorage, ReadExpect, LazyUpdate};
use crate::components::{DamageTileAction, Health, HexCoords, FloorTile, Character, Resistance, DamageActions};
use crate::resources::Floor;

pub struct DamageTileSystem;

impl<'s> System<'s> for DamageTileSystem {
    type SystemData = (
        Entities<'s>,
        ReadExpect<'s, Floor>,
        ReadStorage<'s, FloorTile>,
        ReadStorage<'s, DamageTileAction>,
        WriteStorage<'s, DamageActions>,
        ReadStorage<'s, HexCoords>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, (
        entities,
        floor,
        tiles,
        damage_tiles,
        mut damage_actions,
        hexes,
        lazy_update,
    ): Self::SystemData) {
        for (entity, damage_tile, hex) in (&entities, &damage_tiles, &hexes).join() {
            if let Some(tile) = floor.get(hex) {
                if let Some(f_tile) = tiles.get(tile) {
                    if let Some(cha) = f_tile.character {
                        if let Some(damage) = damage_actions.get_mut(cha) {
                            damage.d.push(damage_tile.options.clone());
                        } else {
                            lazy_update.insert(cha, DamageActions{ d: vec![damage_tile.options.clone()], });
                        }
                    }
                }
            }
            entities.delete(entity);
        }
    }
}

pub struct DamageSystem;

impl<'s> System<'s> for DamageSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Character>,
        WriteStorage<'s, DamageActions>,
        WriteStorage<'s, Health>,
        ReadStorage<'s, Resistance>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, (
        entities,
        characters,
        mut damages,
        mut health,
        resistances,
        lazy_update,
    ): Self::SystemData) {
        for (entity, character, damages, hp) in (&entities, &characters, &mut damages, &mut health).join() {
            if damages.d.len() > 0 {
                let damage = damages.d.remove(0);
                // todo: resistances
                let mut curr = damage.amount;
                if hp.temp > 0 {
                    if curr >= hp.temp {
                        curr -= hp.temp;
                        hp.temp = 0;
                    }
                }
                if hp.current <= curr {
                    // todo: death
                    hp.current = 0;
                } else {
                    hp.current -= curr;
                }
            } else {
                lazy_update.remove::<DamageActions>(entity);
            }
        }
    }
}