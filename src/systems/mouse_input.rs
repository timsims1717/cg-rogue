use amethyst::{
    core::Transform,
    ecs::*,
    input::{InputHandler, StringBindings},
    renderer::camera::Camera,
    window::ScreenDimensions,
};
use crate::resources::{CameraHandle, Floor, UISprites, Game, Phase};
use crate::util::{tile_exists, mouse_to_map_hex, shortest_path, PathEnds};
use crate::components::{FloorTile, TileUIElement, Character, MovementAction, MovementOptions, ActionOption, AttackOptions, Player, HexCoords, DamageType, Hovered};
use crate::entities::{create_tile_ui, TileUI};
use amethyst::renderer::rendy::wsi::winit::MouseButton::{Left, Right};
use crate::components::ActionOption::{Move, Interact, Attack};
use crate::resources::Phase::PlayerActionPhase;
use amethyst::ui::UiTransform;

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
        ReadStorage<'s, UiTransform>,
        WriteStorage<'s, Hovered>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, Floor>,
        ReadExpect<'s, CameraHandle>,
        ReadExpect<'s, ScreenDimensions>,
        WriteStorage<'s, Player>,
    );

    fn run(&mut self, (
        entities,
        mut tiles_ui,
        cameras,
        transforms,
        ui_transforms,
        mut hovereds,
        input_handler,
        floor,
        camera_handle,
        screen_dimensions,
        mut players,
    ): Self::SystemData) {
        'outer: for (player) in (&mut players).join() {
            if input_handler.action_is_down("MoveButton").unwrap() {
                player.mode = Move(MovementOptions {
                    range: 2,
                    line: false,
                });
            }

            if input_handler.action_is_down("AttackButton").unwrap() {
                player.mode = Attack(AttackOptions::basic(2, 1, DamageType::Blunt));
            }

            if let Some((xf, yf)) = input_handler.mouse_position() {
                let fyf = screen_dimensions.height() - yf;
                player.input.l_click = input_handler.mouse_button_is_down(Left);
                player.input.r_click = input_handler.mouse_button_is_down(Right); // todo: right click cancels current card
                let camera_transform = transforms.get(camera_handle.camera).unwrap();
                let camera = cameras.get(camera_handle.camera).unwrap();

                let mut ent = None;
                for (entity, ui_transform, _) in (&entities, &ui_transforms, &hovereds).join() {
                    if !ui_transform.position_inside(xf, fyf) {
                        ent = Some(entity);
                    } else {
                        continue 'outer;
                    }
                }
                if let Some(entity) = ent {
                    hovereds.remove(entity);
                }

                let mut top_z = 0.0;
                let mut ent = None;
                for (entity, ui_transform) in (&entities, &ui_transforms).join() {
                    if ui_transform.position_inside(xf, fyf) && top_z < ui_transform.local_z {
                        top_z = ui_transform.local_z;
                        ent = Some(entity);
                    }
                }
                if let Some(entity) = ent {
                    hovereds.insert(entity, Hovered{});
                    player.input.tile = None;
                    continue;
                }

                for (entity, tile) in (&*entities, &mut tiles_ui).join() {
                    if tile.hover {
                        entities.delete(entity);
                    }
                } // todo: create a ui hover system
                let (map_x, map_y) = mouse_to_map_hex(xf, yf, &screen_dimensions, camera, camera_transform);
                if tile_exists(map_x as isize, map_y as isize, floor.dimensions.width as isize, floor.dimensions.height as isize) {
                    let (tile_x, tile_y) = (map_x as usize, map_y as usize);
                    player.input.tile = Some(HexCoords{ x: tile_x, y: tile_y });
                } else {
                    player.input.tile = None;
                }
            }
        }
    }
}