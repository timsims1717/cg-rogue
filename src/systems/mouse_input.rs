use amethyst::{
    core::Transform,
    ecs::*,
    input::{InputHandler, StringBindings},
    renderer::camera::Camera,
    window::ScreenDimensions,
};
use crate::resources::{CameraHandle, Floor, UISprites};
use crate::util::{tile_exists, mouse_to_map_hex};
use crate::components::{FloorTile, TileUIElement, Character, Movement};
use crate::entities::{create_tile_ui, TileUI};
use amethyst::renderer::rendy::wsi::winit::MouseButton::{Left, Right};

pub struct MouseInputSystem;

impl<'s> System<'s> for MouseInputSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, FloorTile>,
        WriteStorage<'s, TileUIElement>,
        ReadStorage<'s, Camera>,
        ReadStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, Floor>,
        ReadExpect<'s, CameraHandle>,
        ReadExpect<'s, LazyUpdate>,
        ReadExpect<'s, ScreenDimensions>,
        ReadExpect<'s, UISprites>,
        WriteStorage<'s, Character>
    );

    fn run(&mut self, (
        entities,
        mut tiles,
        mut tiles_ui,
        cameras,
        transforms,
        input_handler,
        floor,
        camera_handle,
        lazy_update,
        screen_dimensions,
        ui_sprites,
        characters
    ): Self::SystemData) {
        if let Some((xf, yf)) = input_handler.mouse_position() {
            let left_click = input_handler.mouse_button_is_down(Left);
            let right_click = input_handler.mouse_button_is_down(Right); // todo: right click cancels current card
            let camera_transform = transforms.get(camera_handle.camera).unwrap();
            let camera = cameras.get(camera_handle.camera).unwrap();

            // todo: ui first

            let (map_x, map_y) = mouse_to_map_hex(xf, yf, &screen_dimensions, camera, camera_transform);
            if tile_exists(map_x as isize, map_y as isize, floor.dimensions.width as isize, floor.dimensions.height as isize) {
                let (tile_x, tile_y) =  (map_x as usize, map_y as usize);
                let _tile = floor.get(tile_x, tile_y);
                // todo: use the tile to identify if highlighting should happen
                // todo: identify which tiles should be highlighted
                let tile_ui_need: Vec<(usize, usize)> = vec![(tile_x, tile_y)];

                // go through all ui_tiles, if they aren't needed, remove them, if they are, add them to "found"
                let mut tile_ui_found: Vec<(usize, usize)> = vec![];
                for (entity, tile_ui) in (&*entities, &mut tiles_ui).join() {
                    if tile_ui_need.iter().any(|(ix, iy)| *ix == tile_ui.x && *iy == tile_ui.y) {
                        tile_ui_found.push((tile_ui.x, tile_ui.y));
                    } else if tile_ui.hover {
                        entities.delete(entity);
                    }
                }
                for (x, y) in tile_ui_need.iter() {
                    // unless the ui_tile already exists in found ...
                    if left_click || !(tile_ui_found.iter().any(|(ix, iy)| ix == x && iy == y)) {
                        let parent = floor.get(*x, *y);
                        if let Some(_tile) = tiles.get(parent) {
                            // todo: use the tile to adjust what kind of ui element is created
                            create_tile_ui(&entities, ui_sprites.set.clone(), *x, *y, !left_click, TileUI::Move, &lazy_update);
                            if left_click {
                                for (entity, character) in (&*entities, &characters).join() {
                                    lazy_update.insert(entity, Movement{
                                        path: vec![(character.x, character.y),(*x, *y)],
                                        path_i: 0,
                                        smooth: true,
                                    });
                                }
                            }
                        }
                    }
                }
            } else {
                // if mouse is outside the map
                for (entity, tile) in (&*entities, &mut tiles_ui).join() {
                    if tile.hover {
                        entities.delete(entity);
                    }
                }
            }
        };
    }
}