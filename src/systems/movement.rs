use amethyst::core::ecs::{Join, System, WriteStorage, Read, ReadExpect, LazyUpdate, Entities, ReadStorage};
use amethyst::core::{Transform, Time};
use crate::components::{MovementAction, Character, FloorTile};
use amethyst::core::math::Vector3;
use crate::util::map_to_world_hex;
use crate::resources::Floor;

pub struct MovementActionSystem;

impl<'s> System<'s> for MovementActionSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, MovementAction>,
        WriteStorage<'s, Character>,
        Read<'s, Time>,
        ReadExpect<'s, LazyUpdate>,
        ReadExpect<'s, Floor>,
        WriteStorage<'s, FloorTile>,
    );

    fn run(&mut self, (
        entities,
        mut transforms,
        mut movements,
        mut characters,
        time,
        lazy_update,
        floor,
        mut tiles,
    ): Self::SystemData) {
        // todo: add a fast mode?
        let movement_speed = 3.5;
        let delta_time = time.delta_real_seconds();
        let move_factor = movement_speed * delta_time;
        for (entity, transform, movement, character) in (&entities, &mut transforms, &mut movements, &mut characters).join() {
            if movement.is_go() && !movement.path_complete() {
                let (a_x, a_y, b_x, b_y) = movement.get_move();
                if let Some(next_tile) = tiles.get_mut(floor.get(b_x, b_y)) {
                    if next_tile.occupied && movement.paused {
                        continue;
                    } else {
                        next_tile.occupied = true;
                        movement.paused = false;
                    }
                    let (start_x, start_y) = map_to_world_hex(a_x as f32, a_y as f32);
                    let (end_x, end_y) = map_to_world_hex(b_x as f32, b_y as f32);
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
                        character.x = b_x;
                        character.y = b_y;
                        // todo: remove ui bits and do any other on move stuff
                        movement.paused = true;
                        if movement.path_complete() {
                            character.acting = false;
                            lazy_update.remove::<MovementAction>(entity);
                        } else {
                            next_tile.occupied = false;
                        }
                    }
                } else {
                    character.acting = false;
                    lazy_update.remove::<MovementAction>(entity);
                    continue;
                }
                if movement.first_run() {
                    if let Some(prev_tile) = tiles.get_mut(floor.get(a_x, a_y)) {
                        prev_tile.occupied = false;
                    }
                }
            }
            if movement.path_complete() {
                character.acting = false;
                lazy_update.remove::<MovementAction>(entity);
            }
        }
    }
}