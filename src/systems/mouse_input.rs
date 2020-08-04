use amethyst::{
    core::Transform,
    ecs::*,
    input::{InputHandler, StringBindings},
    renderer::camera::Camera,
    window::ScreenDimensions,
};
use crate::resources::{CameraHandle, Floor, UISprites, Player, Game, Phase};
use crate::util::{tile_exists, mouse_to_map_hex, shortest_path, PathEnds};
use crate::components::{FloorTile, TileUIElement, Character, Movement, MovementOptions, Mode};
use crate::entities::{create_tile_ui, TileUI};
use amethyst::renderer::rendy::wsi::winit::MouseButton::{Left, Right};
use crate::components::Mode::{Move, Interact};
use crate::resources::Phase::PlayerAction;

pub struct MouseInputSystem {
    last_map: (usize, usize),
}

impl MouseInputSystem{
    pub fn new() -> MouseInputSystem {
        MouseInputSystem{
            last_map: (0, 0),
        }
    }
}

impl<'s> System<'s> for MouseInputSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, TileUIElement>,
        ReadStorage<'s, Camera>,
        ReadStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, Floor>,
        ReadExpect<'s, CameraHandle>,
        ReadExpect<'s, ScreenDimensions>,
        WriteExpect<'s, Player>,
    );

    fn run(&mut self, (
        entities,
        mut tiles_ui,
        cameras,
        transforms,
        input_handler,
        floor,
        camera_handle,
        screen_dimensions,
        mut player,
    ): Self::SystemData) {

        if input_handler.action_is_down("MoveButton").unwrap() {
            player.mode = Move(MovementOptions{
                range: 2,
                line: false,
            });
        }

        if let Some((xf, yf)) = input_handler.mouse_position() {
            player.input.l_click = input_handler.mouse_button_is_down(Left);
            player.input.r_click = input_handler.mouse_button_is_down(Right); // todo: right click cancels current card
            let camera_transform = transforms.get(camera_handle.camera).unwrap();
            let camera = cameras.get(camera_handle.camera).unwrap();

            // todo: ui first

            for (entity, tile) in (&*entities, &mut tiles_ui).join() {
                if tile.hover {
                    entities.delete(entity);
                }
            } // todo: destroy only if something has changed
            let (map_x, map_y) = mouse_to_map_hex(xf, yf, &screen_dimensions, camera, camera_transform);
            if tile_exists(map_x as isize, map_y as isize, floor.dimensions.width as isize, floor.dimensions.height as isize) {
                let (tile_x, tile_y) =  (map_x as usize, map_y as usize);
                player.input.tile = Some((tile_x, tile_y));
                // if (tile_x, tile_y) == self.last_map {
                //     todo: this is how to check if anything has changed
                // }
                // let tile_ui_need: Vec<(usize, usize)> = vec![(tile_x, tile_y)];
                //
                // // go through all ui_tiles, if they aren't needed, remove them, if they are, add them to "found"
                // let mut tile_ui_found: Vec<(usize, usize)> = vec![];
                // for (entity, tile_ui) in (&*entities, &mut tiles_ui).join() {
                //     if tile_ui_need.iter().any(|(ix, iy)| *ix == tile_ui.x && *iy == tile_ui.y) {
                //         tile_ui_found.push((tile_ui.x, tile_ui.y));
                //     } else if tile_ui.hover {
                //         entities.delete(entity);
                //     }
                // }
                // for (x, y) in tile_ui_need.iter() {
                //     // unless the ui_tile already exists in found ...
                //     if left_click || !(tile_ui_found.iter().any(|(ix, iy)| ix == x && iy == y)) {
                //         let parent = floor.get(*x, *y);
                //         if let Some(_tile) = tiles.get(parent) {
                //             // todo: use the tile to adjust what kind of ui element is created
                //             create_tile_ui(&entities, ui_sprites.set.clone(), *x, *y, true, TileUI::Move, &lazy_update);
                //             if left_click {
                //                 if let Some(character) = entities.get(player.character) {
                //                     if let Some(path) = shortest_path(
                //                         PathEnds{
                //                             a_x: character.x,
                //                             a_y: character.y,
                //                             b_x: *x,
                //                             b_y: *y
                //                         }, &floor, &tiles,
                //                     ) {
                //                         lazy_update.insert(character, Movement::new(path, true));
                //                     }
                //                 }
                //             }
                //         }
                //     }
                // }
            } else {
                // if mouse is outside the map
                // for (entity, tile) in (&*entities, &mut tiles_ui).join() {
                //     if tile.hover {
                //         entities.delete(entity);
                //     }
                // }
            }
        };
    }
}