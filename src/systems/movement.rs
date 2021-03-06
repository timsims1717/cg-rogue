use amethyst::core::ecs::{Join, System, WriteStorage, Read, ReadExpect, LazyUpdate, Entities, ReadStorage, Entity};
use amethyst::core::{Transform, Time};
use crate::components::{MovementAction, Character, FloorTile, HexCoords, Acting, ID};
use amethyst::core::math::Vector3;
use crate::util::{map_to_world_hex, PathEnds};
use crate::resources::Floor;

pub struct MovementActionSystem;

impl<'s> System<'s> for MovementActionSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, MovementAction>,
        ReadStorage<'s, ID>,
        WriteStorage<'s, HexCoords>,
        ReadStorage<'s, Acting>,
        Read<'s, Time>,
        ReadExpect<'s, LazyUpdate>,
        ReadExpect<'s, Floor>,
        WriteStorage<'s, FloorTile>,
    );

    fn run(&mut self, (
        entities,
        mut transforms,
        mut movements,
        ids,
        mut hexes,
        acting,
        time,
        lazy_update,
        floor,
        mut tiles,
    ): Self::SystemData) {
        // todo: add a fast mode?
        let movement_speed = 3.5;
        let delta_time = time.delta_real_seconds();
        let move_factor = movement_speed * delta_time;
        for (entity, transform, movement, hex) in (&entities, &mut transforms, &mut movements, &mut hexes).join() {
            if !movement.path_complete() {
                let path = movement.get_move();
                // Attempt to move the character's location
                let result = move_entity(&floor, &path, &entity, &mut tiles, &ids, &acting);
                if result.must_stop {
                    lazy_update.remove::<MovementAction>(entity);
                    lazy_update.remove::<Acting>(entity);
                    continue;
                }
                let (start_x, start_y) = map_to_world_hex(path.a.x as f32, path.a.y as f32);
                let (end_x, end_y) = map_to_world_hex(path.b.x as f32, path.b.y as f32);
                let diff_x = end_x - start_x;
                let diff_y = end_y - start_y;
                let move_x = move_factor * diff_x;
                let move_y = move_factor * diff_y;
                let mut arrived = true;
                let tx = transform.translation().x;
                let ty = -transform.translation().y;

                if (move_x < 0. && tx + move_x > end_x)
                    || (move_x > 0. && tx + move_x < end_x) {
                    arrived = false;
                    transform.move_right(move_x);
                } else {
                    transform.set_translation_x(end_x);
                }
                if (move_y < 0. && ty + move_y > end_y)
                    || (move_y > 0. && ty + move_y < end_y) {
                    arrived = false;
                    transform.move_down(move_y);
                } else {
                    transform.set_translation_y(-end_y);
                }

                if arrived {
                    movement.next_move();
                    hex.x = path.b.x;
                    hex.y = path.b.y;
                }
            } else {
                // todo: remove ui bits and do any other on move stuff
                lazy_update.remove::<MovementAction>(entity);
                lazy_update.remove::<Acting>(entity);
            }
        }
    }
}

/// Changes the location of the entity at path.a to path.b
pub fn move_entity(floor: &Floor, path: &PathEnds, entity: &Entity, tiles: &mut WriteStorage<FloorTile>, ids: &ReadStorage<ID>, acting: &ReadStorage<Acting>) -> MoveResult {
    let id = ids.get(*entity).unwrap();
    // The tiles must exist
    // The first tile must have a character entity
    if let Some(tile_a) = floor.get(&path.a) {
        if let Some(tile_b) = floor.get(&path.b) {
            if let Some(mut f_tile_a) = tiles.get(tile_a) {
                if let Some(mut f_tile_b) = tiles.get(tile_b) {
                    if let Some(entity_b) = f_tile_b.character {
                        // there is already a character at b
                        if let Some(id_b) = ids.get(entity_b) {
                            return if id_b.get() == id.get() {
                                // the character has already started moving to b
                                MoveResult {
                                    move_now: true,
                                    must_stop: false,
                                }
                            } else {
                                // don't move, and stop if character_b is not moving
                                MoveResult {
                                    move_now: false,
                                    must_stop: acting.get(entity_b).is_none(),
                                }
                            }
                        }
                    } else {
                        if let Some(entity_a) = f_tile_a.character {
                            if let Some(id_a) = ids.get(entity_a) {
                                if id_a.get() == id.get() {
                                    // there is no character entity at b. Move the character from a to b.
                                    tiles.get_mut(tile_a).unwrap().character = None;
                                    tiles.get_mut(tile_b).unwrap().character = Some(entity_a);
                                    return MoveResult {
                                        move_now: true,
                                        must_stop: false,
                                    };
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    MoveResult{
        move_now: false,
        must_stop: true,
    }
}

pub struct MoveResult {
    pub move_now: bool,
    pub must_stop: bool,
}